use hex_literal::hex;
use uniqueone_appchain_runtime::{
	opaque::Block, opaque::SessionKeys, AccountId, BabeConfig, Balance, BalancesConfig, 
	GenesisConfig, GrandpaConfig, ImOnlineConfig, OctopusAppchainConfig, OctopusLposConfig, 
	SessionConfig, Signature, SudoConfig, SystemConfig, UNET, 
	CouncilCollectiveConfig, TechComitteeCollectiveConfig, DemocracyConfig, SchedulerConfig, WASM_BINARY,
	// EVM 
	Precompiles, EVMConfig, EthereumConfig, 

};

use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};
	  
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	beefy: BeefyId,
	octopus: OctopusId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	s: &str,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<BeefyId>(s),
		get_from_seed::<OctopusId>(s),
	)
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	properties
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let properties = get_properties("UNET", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Unique One Development",
		// ID
		"uniqueone_dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Council Members
				vec![get_account_id_from_seed::<sr25519::Public>("Alice"), get_account_id_from_seed::<sr25519::Public>("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				Some(vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
				]),
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uniqueone-dev"),
		// Properties
        Some(properties),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let properties = get_properties("UNET", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Unique One Local Testnet",
		// ID
		"uniqueone_local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Council Members
				vec![get_account_id_from_seed::<sr25519::Public>("Alice"), get_account_id_from_seed::<sr25519::Public>("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				Some(vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				]),
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uniqueone-local-testnet"),
		// Properties
        Some(properties),
		// Extensions
		None,
	))
}

pub fn staging_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let properties = get_properties("UNET", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Unique One Staging Testnet",
		// ID
		"uniqueone_staging_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].into(),
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].unchecked_into(),
						// 5HgtmpdnGgKJ1Wia5XTsFPGw3JYjKXFrrCXQq7TparBwLzJi
						hex!["f8c6bff47eec7c2fe49be0b0d6c638757ff19562ea9ae1a76ed63d22f979b79c"].unchecked_into(),
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].unchecked_into(),
						// 5CtT5c71rGrnnj3VJqvNqvJQKeBwyh73jGm4EduQ32igi1ym
						hex!["02e23d37735cebc72732f31c86cca3a70f2c5d3087854ac94fe5aa1fec1bbb0e18"].unchecked_into(),
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].unchecked_into(),

					),
					(
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].into(),
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].unchecked_into(),
						// 5CYZaymcu6jdq3bzsch7dgfsQdG347VEjPNbygwrtUXqGqmu
						hex!["153f07c39a47483fbdc7be025a78139fec54e350894fbaf567125135571c0e66"].unchecked_into(),
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].unchecked_into(),
						// 5CvRworpEZfn7FYeeXJmTHBVoEZvow3HRDxWCEZ5cN9dJKF6
						hex!["036ce9ccdf3a1a4a6f5e65c44cd2453640ba4cee5a148e6362baa5d2129aed94fb"].unchecked_into(),
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].unchecked_into(),

					),					
				],
				// Council Members
				// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
				vec![ 
					hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into() 
				],

				// Sudo account
				// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
				hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),
				// Pre-funded accounts
				Some(vec![
					// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
					hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),
				]),
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uniqueone-staging-testnet"),
		// Properties
        Some(properties),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId)>,
	council_members: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	_enable_println: bool,
) -> GenesisConfig {

	// This is the simplest bytecode to revert without returning any data.
	// We will pre-deploy it under all of our precompiles to ensure they can be called from
	// within contracts.
	// (PUSH1 0x00 PUSH1 0x00 REVERT)
	let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		]
	});

	initial_authorities.iter().map(|x| &x.0).for_each(|x| {
		if !endowed_accounts.contains(&x) {
			endowed_accounts.push(x.clone())
		}
	});
	let validators = initial_authorities.iter().map(|x| (x.0.clone(), STASH)).collect::<Vec<_>>();

	const ENDOWMENT: Balance = 10_000_000 * UNET;
	const STASH: Balance = 100 * 1_000_000_000_000_000_000; // 100 OCT with 18 decimals

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.1.clone(),
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		sudo: SudoConfig { key: root_key },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(uniqueone_appchain_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		assets: Default::default(),
		beefy: BeefyConfig { authorities: vec![] },
		octopus_appchain: OctopusAppchainConfig {
			anchor_contract: "".to_string(),			
			asset_id_by_name: vec![("usdc.testnet".to_string(), 0)],
			validators,
			premined_amount: 1024 * UNET,
		},
		octopus_lpos: OctopusLposConfig { era_payout: 2 * UNET, ..Default::default() },		
		evm: EVMConfig {
			// We need _some_ code inserted at the precompile address so that
			// the evm will actually call the address.
			accounts: Precompiles::used_addresses()
				.map(|addr| {
					(
						addr.into(),
						GenesisAccount {
							nonce: Default::default(),
							balance: Default::default(),
							storage: Default::default(),
							code: revert_bytecode.clone(),
						},
					)
				})
				.collect(),
		},
		ethereum: EthereumConfig {},		
		democracy: DemocracyConfig::default(),
		scheduler: SchedulerConfig {},		
		council_collective: CouncilCollectiveConfig {
			phantom: Default::default(),
			members: council_members,
		},
		tech_comittee_collective: TechComitteeCollectiveConfig {
			phantom: Default::default(),
			members: vec![], // TODO : Set members
		},		
		treasury: Default::default(),

	}
}
