//! Service implementation. Specialized wrapper over substrate service.

use crate::cli::Cli;
use futures::StreamExt;

use std::{
	collections::BTreeMap,
	sync::{Arc, Mutex},
	time::Duration,
};

use sp_runtime::traits::Block as BlockT;

use sc_client_api::{BlockchainEvents, ExecutorProvider};
use sc_consensus_babe::{self, SlotProportion};
use sc_executor::NativeElseWasmExecutor;
use sc_finality_grandpa as grandpa;
use sc_network::NetworkService;
use sc_service::{BasePath, config::Configuration, error::Error as ServiceError, RpcHandlers, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};

use fc_consensus::FrontierBlockImport;
use fc_mapping_sync::{MappingSyncWorker, SyncStrategy};
use fc_rpc::EthTask;
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};

use uniqueone_appchain_runtime::{opaque::Block, RuntimeApi};

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		uniqueone_appchain_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		uniqueone_appchain_runtime::native_version()
	}
}

/// The full client type definition.
pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport = grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;
/// The transaction pool type defintion.
pub type TransactionPool = sc_transaction_pool::FullPool<Block, FullClient>;

pub fn frontier_database_dir(config: &Configuration) -> std::path::PathBuf {
	let config_dir = config
		.base_path
		.as_ref()
		.map(|base_path| base_path.config_dir(config.chain_spec.id()))
		.unwrap_or_else(|| {
			BasePath::from_project("", "", "uniqueone")
				.config_dir(config.chain_spec.id())
		});
	config_dir.join("frontier").join("db")
}

pub fn open_frontier_backend(config: &Configuration) -> Result<Arc<fc_db::Backend<Block>>, String> {
	Ok(Arc::new(fc_db::Backend::<Block>::new(
		&fc_db::DatabaseSettings {
			source: fc_db::DatabaseSettingsSrc::RocksDb {
				path: frontier_database_dir(&config),
				cache_size: 0,
			},
		},
	)?))
}

/// Creates a new partial node.
#[allow(clippy::type_complexity)]
pub fn new_partial(
	config: &Configuration,
	cli: &Cli,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			(
				sc_consensus_babe::BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
				grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
				sc_consensus_babe::BabeLink<Block>,
				beefy_gadget::notification::BeefySignedCommitmentStream<Block>,
				beefy_gadget::notification::BeefySignedCommitmentSender<Block>,
			),
			(
				bool,
				Arc<fc_db::Backend<Block>>,
				u32,
				Option<FilterPool>,
				u64,
				FeeHistoryCache,
			),
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
	);

	let (client, backend, keystore_container, task_manager) = sc_service::new_full_parts::<Block, RuntimeApi, _>(
		config,
		telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
		executor,
	)?;

	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let enable_dev_signer = cli.run.enable_dev_signer;
	let frontier_backend = open_frontier_backend(config)?;
	let max_past_logs = cli.run.max_past_logs;
	let fee_history_limit = cli.run.fee_history_limit;
	let filter_pool: Option<FilterPool> = Some(Arc::new(Mutex::new(BTreeMap::new())));
	let fee_history_cache: FeeHistoryCache = Arc::new(Mutex::new(BTreeMap::new()));

	let (grandpa_block_import, grandpa_link) = grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let justification_import = grandpa_block_import.clone();

	let (babe_block_import, babe_link) = sc_consensus_babe::block_import(
		sc_consensus_babe::Config::get_or_compute(&*client)?,
		grandpa_block_import,
		client.clone(),
	)?;

	let slot_duration = babe_link.config().slot_duration();

	let block_import = FrontierBlockImport::new(
		babe_block_import.clone(),
		client.clone(),
		frontier_backend.clone(),
	);

	let import_queue = sc_consensus_babe::import_queue(
		babe_link.clone(),
		block_import,
		Some(Box::new(justification_import)),
		client.clone(),
		select_chain.clone(),
		move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot = sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
				*timestamp,
				slot_duration,
			);

			let uncles = sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

			Ok((timestamp, slot, uncles))
		},
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
		sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let (beefy_link, beefy_commitment_stream) = beefy_gadget::notification::BeefySignedCommitmentStream::channel();

	let import_setup = (babe_block_import, grandpa_link, babe_link, beefy_commitment_stream, beefy_link);

	let frontier_setup = (enable_dev_signer, frontier_backend, max_past_logs, filter_pool, fee_history_limit, fee_history_cache);

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (import_setup, frontier_setup, telemetry),
	})
}

/// Result of [`new_full_base`].
pub struct NewFullBase {
	/// The task manager of the node.
	pub task_manager: TaskManager,
	/// The client instance of the node.
	pub client: Arc<FullClient>,
	/// The networking service of the node.
	pub network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	/// The transaction pool of the node.
	pub transaction_pool: Arc<TransactionPool>,
	/// The rpc handlers of the node.
	pub rpc_handlers: RpcHandlers,
}

/// Creates a full service from the configuration.
pub fn new_full_base(
	mut config: Configuration,
	cli: &Cli,
	with_startup_data: impl FnOnce(
		&sc_consensus_babe::BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
		&sc_consensus_babe::BabeLink<Block>,
	),
) -> Result<NewFullBase, ServiceError> {
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (import_setup, frontier_setup, mut telemetry),
	} = new_partial(&config, cli)?;

	let (block_import, grandpa_link, babe_link, beefy_commitment_stream, beefy_link) = import_setup;

	let (enable_dev_signer, frontier_backend, max_past_logs, filter_pool, fee_history_limit, fee_history_cache) = frontier_setup;

	config.network.extra_sets.push(grandpa::grandpa_peers_set_config());
	config.network.extra_sets.push(beefy_gadget::beefy_peers_set_config());

	let warp_sync = Arc::new(grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		grandpa_link.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, network_starter) = sc_service::build_network(sc_service::BuildNetworkParams {
		config: &config,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		spawn_handle: task_manager.spawn_handle(),
		import_queue,
		block_announce_validator_builder: None,
		warp_sync: Some(warp_sync),
	})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let role = config.role.clone();
	let is_authority = config.role.is_authority();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks = Some(sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging::default());
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();

	let (rpc_extensions_builder, rpc_setup) = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let chain_spec = config.chain_spec.cloned_box();

		let babe_config = babe_link.config().clone();
		let shared_epoch_changes = babe_link.epoch_changes().clone();
		let keystore = keystore_container.sync_keystore();

		let shared_voter_state = grandpa::SharedVoterState::empty();
		let rpc_setup = shared_voter_state.clone();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let justification_stream = grandpa_link.justification_stream().clone();
		let finality_proof_provider = grandpa::FinalityProofProvider::new_for_service(
			backend.clone(),
			Some(shared_authority_set.clone()),
		);

		let network = network.clone();
		let frontier_backend = frontier_backend.clone();
		let filter_pool = filter_pool.clone();
		let fee_history_cache = fee_history_cache.clone();

		let rpc_extensions_builder = move |deny_unsafe, subscription_executor: sc_rpc::SubscriptionTaskExecutor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				graph: pool.pool().clone(),
				select_chain: select_chain.clone(),
				chain_spec: chain_spec.cloned_box(),
				deny_unsafe,
				babe: crate::rpc::BabeDeps {
					babe_config: babe_config.clone(),
					shared_epoch_changes: shared_epoch_changes.clone(),
					keystore: keystore.clone(),
				},
				grandpa: crate::rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
					justification_stream: justification_stream.clone(),
					subscription_executor: subscription_executor.clone(),
					finality_provider: finality_proof_provider.clone(),
				},
				beefy: crate::rpc::BeefyDeps {
					beefy_commitment_stream: beefy_commitment_stream.clone(),
					subscription_executor: subscription_executor.clone(),
				},
				frontier:  crate::rpc::FrontierDeps {
					network: network.clone(),
					enable_dev_signer,
					backend: frontier_backend.clone(),
					is_authority,
					max_past_logs,
					filter_pool: filter_pool.clone(),
					fee_history_limit,
					fee_history_cache: fee_history_cache.clone(),
				},
			};

			crate::rpc::create_full(deps, subscription_executor).map_err(Into::into)
		};

		(rpc_extensions_builder, rpc_setup)
	};

	let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		backend: backend.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		network: network.clone(),
		rpc_extensions_builder: Box::new(rpc_extensions_builder),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;

	(with_startup_data)(&block_import, &babe_link);

	if let sc_service::config::Role::Authority { .. } = &role {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let can_author_with = sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let client_clone = client.clone();
		let slot_duration = babe_link.config().slot_duration();
		let babe_config = sc_consensus_babe::BabeParams {
			keystore: keystore_container.sync_keystore(),
			client: client.clone(),
			select_chain,
			env: proposer,
			block_import,
			sync_oracle: network.clone(),
			justification_sync_link: network.clone(),
			create_inherent_data_providers: move |parent, ()| {
				let client_clone = client_clone.clone();
				async move {
					let uncles = sc_consensus_uncles::create_uncles_inherent_data_provider(
						&*client_clone,
						parent,
					)?;

					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot = sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
						*timestamp,
						slot_duration,
					);

					let storage_proof = sp_transaction_storage_proof::registration::new_data_provider(
						&*client_clone,
						&parent,
					)?;

					Ok((timestamp, slot, uncles, storage_proof))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			can_author_with,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		let babe = sc_consensus_babe::start_babe(babe_config)?;
		task_manager.spawn_essential_handle().spawn_blocking("babe-proposer", Some("block-authoring"), babe);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };

	let beefy_params = beefy_gadget::BeefyParams {
		client: client.clone(),
		backend: backend.clone(),
		key_store: keystore.clone(),
		network: network.clone(),
		signed_commitment_sender: beefy_link,
		min_block_delta: 8,
		prometheus_registry: prometheus_registry.clone(),
	};

	let gadget = beefy_gadget::start_beefy_gadget::<_, _, _, _>(beefy_params);

	task_manager.spawn_handle().spawn_blocking("beefy-gadget", None, gadget);

	let config = grandpa::Config {
		// FIXME #1578 make this available through chainspec
		gossip_duration: std::time::Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		local_role: role,
		telemetry: telemetry.as_ref().map(|x| x.handle()),
	};

	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = grandpa::GrandpaParams {
			config,
			link: grandpa_link,
			network: network.clone(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			voting_rule: grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state: rpc_setup,
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager
			.spawn_essential_handle()
			.spawn_blocking("grandpa-voter", None, grandpa::run_grandpa_voter(grandpa_config)?);
	}

	task_manager.spawn_essential_handle().spawn(
		"frontier-mapping-sync-worker",
		None,
		MappingSyncWorker::new(
			client.import_notification_stream(),
			Duration::new(6, 0),
			client.clone(),
			backend.clone(),
			frontier_backend.clone(),
			SyncStrategy::Normal,
		)
		.for_each(|()| futures::future::ready(())),
	);

	// Spawn Frontier EthFilterApi maintenance task.
	if let Some(filter_pool) = filter_pool {
		// Each filter is allowed to stay in the pool for 100 blocks.
		const FILTER_RETAIN_THRESHOLD: u64 = 100;
		task_manager.spawn_essential_handle().spawn(
			"frontier-filter-pool",
			None,
			EthTask::filter_pool_task(Arc::clone(&client), filter_pool.clone(), FILTER_RETAIN_THRESHOLD),
		);
	}

	task_manager.spawn_essential_handle().spawn(
		"frontier-schema-cache-task",
		None,
		EthTask::ethereum_schema_cache_task(Arc::clone(&client), Arc::clone(&frontier_backend)),
	);

	network_starter.start_network();

	Ok(NewFullBase { task_manager, client, network, transaction_pool, rpc_handlers })
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration, cli: &Cli,) -> Result<TaskManager, ServiceError> {
	new_full_base(config, cli, |_, _| ()).map(|NewFullBase { task_manager, .. }| task_manager)
}
