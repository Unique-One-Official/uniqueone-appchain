use hex_literal::hex;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};

use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};

use beefy_primitives::crypto::AuthorityId as BeefyId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::{Ss58Codec, UncheckedInto}, sr25519, H160, U256, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{traits::{IdentifyAccount, Verify}, PerU16};

use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;

use uniqueone_appchain_runtime::{
	currency::{OCTS, UNITS as UNET},
	opaque::{Block, SessionKeys},
	AccountId, BabeConfig, Balance, BalancesConfig, GenesisConfig, OctopusAppchainConfig,
	OctopusLposConfig, SessionConfig, Signature, SudoConfig, SystemConfig,
	CouncilCollectiveConfig, TechComitteeCollectiveConfig, DemocracyConfig, EVMConfig, EthereumConfig,
	TokensConfig, UnetConfConfig, UnetNftConfig, EthereumChainIdConfig,
	BABE_GENESIS_EPOCH_CONFIG, WASM_BINARY,
};

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
	seed: &str,
	stash_amount: Balance,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId, Balance) {
	(
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<BeefyId>(seed),
		get_from_seed::<OctopusId>(seed),
		stash_amount,
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

/// Helper function to generate appchain config
pub fn appchain_config(
	relay_contract: &str,
	asset_id_by_name: &str,
	premined_amount: Balance,
	era_payout: Balance,
) -> (String, String, Balance, Balance) {
	(relay_contract.to_string(), asset_id_by_name.to_string(), premined_amount, era_payout)
}

pub fn mainnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/octopus-mainnet.json")[..])
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/octopus-testnet.json")[..])
}

pub fn staging_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UNET", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"UniqueOne Staging Testnet",
		// ID
		"uniqueone_staging_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
				hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),

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

						100 * OCTS

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

						100 * OCTS

					),
				],
				// Council Members
				vec![
					// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
					hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].into(),
					// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
					hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].into()

				],
				// Tech Comitee Members
				vec![],
				// Pre-funded accounts
				vec![
					(
						// 5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx
						hex!["9c5e883c0a7795c81d354aa2d596364e71f4bb07d047c8dcb67547fbe1114f12"].into(),
						// Balance amount
						124_999_990 * UNET,
					),
					(
						// 5HGtNjqdYQxn8mhBX22Z6HPjRSSmeb54zTTs3s798yyu4fk9
						hex!["e6778539813675cb74a29d82d68ec7d9626430cf5818bc75da3af738c8a48666"].into(),
						// Balance amount
						10 * UNET,
					),
					(
						// 5G9LtRirf1bVqaVChnZmCXmQ2f4dgFCdjDQsS1eA4sGSE8NS
						hex!["b47a9211bf46832f093a29965082003d5b40c817cc678808bf177c879abbbc42"].into(),
						// Balance amount
						10 * UNET,
					)
				],
				// Appchain config
				appchain_config(
					// Relay Contract
					"",
					// Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					875_000_000 * UNET,
					// Era Payout
					68_493 * UNET,
				),
				388,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uniqueone-staging-testnet"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn development_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UNET", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"UniqueOne Development Testnet",
		// ID
		"uniqueone_development_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					authority_keys_from_seed("Alice", 100 * OCTS),
					// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
					authority_keys_from_seed("Bob", 100 * OCTS),
				],
				// Council Members
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob")
				],
				// Tech Comitee Members
				vec![],
				// Pre-funded accounts
				vec![
					(
						// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						// Balance amount
						124_999_990 * UNET,
					),
					(
						// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						// Balance amount
						10 * UNET,
					),
				],
				// Appchain config
				appchain_config(
					// Relay Contract
					"",
					// Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					875_000_000 * UNET,
					// Era Payout
					68_493 * UNET,
				),
				387,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uniqueone-development-testnet"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn local_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UNET", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"UniqueOne Local",
		// ID
		"uniqueone_local",
		ChainType::Local,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					authority_keys_from_seed("Alice", 100 * OCTS),
					// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
					authority_keys_from_seed("Bob", 100 * OCTS),
				],
				// Council Members
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob")
				],
				// Tech Comitee Members
				vec![],
				// Pre-funded accounts
				vec![
					(
						// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						// Balance amount
						124_999_990 * UNET,
					),
					(
						// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						// Balance amount
						10 * UNET,
					),
				],
				// Appchain config
				appchain_config(
					// Relay Contract
					"",
					// Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					875_000_000 * UNET,
					// Era Payout
					68_493 * UNET,
				),
				386,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uniqueone-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("UNET", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"UniqueOne Development",
		// ID
		"uniqueone_development",
		ChainType::Development,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					authority_keys_from_seed("Alice", 100 * OCTS),
				],
				// Council Members
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob")
				],
				// Tech Comitee Members
				vec![],
				// Pre-funded accounts
				vec![
					(
						// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						// Balance amount
						124_999_990 * UNET,
					),
					(
						// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						// Balance amount
						10 * UNET,
					),
				],
				// Appchain config
				appchain_config(
					// Relay Contract
					"",
					// Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					875_000_000 * UNET,
					// Era Payout
					68_493 * UNET,
				),
				385,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("uniqueone-development"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

/// Configure initial storage state for FRAME modules.
fn genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		BeefyId,
		OctopusId,
		Balance,
	)>,
	council_members: Vec<AccountId>,
	tech_comittee_members: Vec<AccountId>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	appchain_config: (String, String, Balance, Balance),
	chain_id: u64,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().map(|x| (x.0.clone(), x.1)).collect(),
		},
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		beefy: Default::default(),
		octopus_appchain: OctopusAppchainConfig {
			anchor_contract: appchain_config.0,
			asset_id_by_name: vec![(appchain_config.1, 0)],
			premined_amount: appchain_config.2,
			validators: initial_authorities.iter().map(|x| (x.0.clone(), x.6)).collect(),
		},
		octopus_lpos: OctopusLposConfig { era_payout: appchain_config.3, ..Default::default() },
		octopus_assets: Default::default(),
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
				.collect(),
		},
		treasury: Default::default(),
		council_collective: CouncilCollectiveConfig {
			phantom: Default::default(),
			members: council_members,
		},
		tech_comittee_collective: TechComitteeCollectiveConfig {
			phantom: Default::default(),
			members: tech_comittee_members,
		},
		democracy: DemocracyConfig::default(),
		ethereum_chain_id: EthereumChainIdConfig { chain_id },
		base_fee: Default::default(),
		dynamic_fee: Default::default(),
		evm: EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b").expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from(5_000 * UNET),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		},
		ethereum: EthereumConfig {},
		sudo: SudoConfig { key: Some(root_key) },
		// TODO: make this dynamic, put on params
		tokens: TokensConfig {
			endowed_accounts: Default::default(),
		},
		orml_nft: Default::default(),
		unet_conf: UnetConfConfig {
			white_list: Default::default(),
			auction_close_delay: unet_traits::time::MINUTES * 10,
			category_list: vec![
				b"Arts".to_vec(),
				b"Animation".to_vec(),
				b"Manga".to_vec(),
				b"Meme".to_vec(),
				b"Trading Cards".to_vec(),
				b"Collectibles".to_vec(),
				b"Unique".to_vec(),
				b"Audio".to_vec(),
				b"Video".to_vec(),
				b"3D".to_vec(),
			],
			..Default::default()
		},
		unet_nft: UnetNftConfig {
			classes: vec![
				// Unique One Default Collection
				unet_traits::ClassConfig {
					class_id: 0,
					class_metadata: String::from_utf8(
						br#"{\"image\":\"https://img.unique.one/ipfs/QmQxTW2N5YSPSaA4gmD5ZjTyB7CCYAG51ooAtn5cCzNEG7\"}"#.to_vec(),
					)
					.unwrap(),
					category_ids: vec![0],
					name: String::from_utf8(b"Unique One".to_vec()).unwrap(),
					description: String::from_utf8(b"Unique One Collection".to_vec()).unwrap(),
					properties: 1 | 2,  // 1 = Transferable, 2 = Burnable
					royalty_rate: PerU16::from_percent(0),
					admins: vec![
						AccountId::from_ss58check(
							"5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx",
						)
						.unwrap(),
					],
					tokens: vec![
						unet_traits::TokenConfig {
							token_id: 0,
							token_metadata: String::from_utf8(
								br#"{\"name\":\"Unique One\",\"description\":\"Unique.One is a next generation decentralised NFT arts marketplace for the growing world of digital artists and collectors.\",\"image\":\"https://img.unique.one/ipfs/QmQxTW2N5YSPSaA4gmD5ZjTyB7CCYAG51ooAtn5cCzNEG7\"}"#.to_vec(),
							)
							.unwrap(),
							royalty_rate: PerU16::from_percent(10),
							token_owner: AccountId::from_ss58check(
								"5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx",
							)
							.unwrap(),
							token_creator: AccountId::from_ss58check(
								"5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx",
							)
							.unwrap(),
							royalty_beneficiary: AccountId::from_ss58check(
								"5FbjQgSg97nvPsfuf21D886B26mwtNvZTgEfGfWR6gdNy3Tx",
							)
							.unwrap(),
							quantity: 1000,
						},
					],
				},
			],
			..Default::default()
		},
		unet_order: Default::default(),
		unet_auction: Default::default(),
	}
}
