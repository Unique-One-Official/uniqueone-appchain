#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use crate::currency::deposit;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use beefy_primitives::{crypto::AuthorityId as BeefyId, mmr::MmrLeafVersion};
use sp_api::impl_runtime_apis;
use sp_consensus_babe::{
	AllowedSlots::PrimaryAndSecondaryVRFSlots, BabeConfiguration, BabeEpochConfiguration, Epoch,
	OpaqueKeyOwnershipProof, Slot,
};
use sp_core::{
	crypto::KeyTypeId,
	sr25519,
	OpaqueMetadata,
	H256,
};

use sp_inherents::{CheckInherentsResult, InherentData};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		self, AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto, IdentifyAccount,
		Keccak256, NumberFor, OpaqueKeys, SaturatedConversion, StaticLookup, Verify,
	},
	transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, FixedPointNumber, MultiSignature, OpaqueExtrinsic, Perbill, Permill,
	Perquintill,
};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

#[cfg(any(feature = "std", test))]
pub use frame_system::Call as SystemCall;
#[cfg(any(feature = "std", test))]
pub use pallet_balances::Call as BalancesCall;
#[cfg(any(feature = "std", test))]
pub use pallet_sudo::Call as SudoCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

use frame_support::{
	construct_runtime, parameter_types,
	dispatch::DispatchClass,
	traits::{
		AsEnsureOriginWithArg, ConstU32, EitherOfDiverse, EqualPrivilegeOnly, Everything, Imbalance,
		InstanceFilter, KeyOwnerProofSystem, Nothing, OnUnbalanced, 
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		ConstantMultiplier, IdentityFee, Weight,
	},
	PalletId,
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	offchain, EnsureRoot, EnsureSigned,
};

use pallet_babe::{AuthorityId as BabeId};
use pallet_balances::NegativeImbalance;
use pallet_contracts::weights::WeightInfo;
use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_session::historical as pallet_session_historical;
pub use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};
use sp_mmr_primitives as mmr;
use pallet_octopus_bridge::impls::RmrkBaseMetadataConvertor;
use unet_nft::impl_nonfungibles::UniqueOneBaseMetadataConvertor;

// Local
pub use unet_traits::constants_types::Amount;
pub use unet_traits::constants_types::CurrencyId;

/// An index to a block.
pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/// Balance of an account.
pub type Balance = u128;
/// Type used for expressing timestamp.
pub type Moment = u64;
/// Index of a transaction in the chain.
pub type Index = u32;
/// A hash of some data used by the chain.
pub type Hash = H256;
/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// The SignedExtension to the basic transaction logic.
///
/// When you change this, you **MUST** modify [`sign`] in `bin/node/testing/src/keyring.rs`!
///
/// [`sign`]: <../../testing/src/keyring.rs.html>
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;
type Migrations = ();
/// Council instance type.
pub type CouncilInstance = pallet_collective::Instance1;
/// Tech committee instancee type.
pub type TechCommitteeInstance = pallet_collective::Instance2;
/// Assets instancee type.
pub type OctopusAssetsInstance = pallet_assets::Instance1;
pub type AssetBalance = u128;
pub type AssetId = u32;
pub type ClassId = u128;
pub type InstanceId = u128;

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
pub struct OctopusAppCrypto;

// #[cfg(feature = "runtime-benchmarks")]
// #[macro_use]
// extern crate frame_benchmarking;

// #[cfg(feature = "runtime-benchmarks")]
// mod benches {
// 	define_benchmarks!(
// 		[unet_auction, UnetAuction]
// 		[unet_nft, UnetNft]
// 		[unet_config, UnetConf]
// 		[unet_order, UnetOrder]
// 	);
// }

/// The type used to represent the kinds of proxying allowed.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Debug, MaxEncodedLen, TypeInfo,
)]
pub enum ProxyType {
	/// All calls can be proxied. This is the trivial/most permissive filter.
	Any = 0,
	/// Only extrinsics that do not transfer funds.
	NonTransfer = 1,
	/// Only extrinsics related to governance (democracy and collectives).
	Governance = 2,
	/// Only extrinsics related to staking.
	Staking = 3,
	/// Allow to veto an announced proxy call.
	CancelProxy = 4,
	/// Allow extrinsic related to Balances.
	Balances = 5,
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
///
// pub mod opaque {
// 	use super::*;

// 	//pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

// 	/// Opaque block header type.
// 	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
// 	/// Opaque block type.
// 	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
// 	/// Opaque block identifier type.
// 	pub type BlockId = generic::BlockId<Block>;

// 	impl_opaque_keys! {
// 		pub struct SessionKeys {
// 			pub babe: Babe,
// 			pub grandpa: Grandpa,
// 			pub im_online: ImOnline,
// 			pub beefy: Beefy,
// 			pub octopus: OctopusAppchain,
// 		}
// 	}
// }

/// The native token, uses 18 decimals of precision.
pub mod currency {
	use super::Balance;

	pub const OCTS: Balance = 1_000_000_000_000_000_000;
	pub const SUPPLY_FACTOR: Balance = 1;

	pub const UNITS: Balance = 1_000_000_000_000_000_000;
	pub const DOLLARS: Balance = UNITS;
	pub const CENTS: Balance = DOLLARS / 100;
	pub const MILLICENTS: Balance = CENTS / 1_000;
	pub const PENNY: Balance = MILLICENTS / 1_000;
	pub const GIGAWEI: Balance = PENNY / 1_000;
	pub const MEGAWEI: Balance = GIGAWEI / 1_000;
	pub const KILOWEI: Balance = MEGAWEI / 1_000;
	pub const WEI: Balance = KILOWEI / 1_000;

	pub const TRANSACTION_BYTE_FEE: Balance = 10 * MILLICENTS * SUPPLY_FACTOR;
	pub const STORAGE_BYTE_FEE: Balance = 100 * MILLICENTS * SUPPLY_FACTOR;
	pub const EXISTENSIAL_DEPOSIT: Balance = 0;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		(items as Balance) * DOLLARS * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
	}
}

// To learn more about runtime versioning and what each of the following value means:
//   https://docs.substrate.io/v3/runtime/origins#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("appchain"),
	impl_name: create_runtime_str!("uniqueone-appchain"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 120,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// Since BABE is probabilistic this is the average expected block time that
/// we are targeting. Blocks will be produced at a minimum duration defined
/// by `SLOT_DURATION`, but some slots will not be allocated to any
/// authority and hence no block will be produced. We expect to have this
/// block time on average following the defined slot duration and the value
/// of `c` configured for BABE (where `1 - c` represents the probability of
/// a slot being empty).
/// This value is only used indirectly to define the unit constants below
/// that are expressed in blocks. The rest of the code should use
/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
/// minimum period).
///
/// If using BABE with secondary slots (default) then all of the slots will
/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
/// `SLOT_DURATION` should have the same value.
///
/// <https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html#-6.-practical-results>
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const WEEKS: BlockNumber = DAYS * 7;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
//       Attempting to do so will brick block production.
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 4 * HOURS;
pub const EPOCH_DURATION_IN_SLOTS: Moment = {
	const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

	(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as Moment
};

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: BabeEpochConfiguration =
	BabeEpochConfiguration { c: PRIMARY_PROBABILITY, allowed_slots: PrimaryAndSecondaryVRFSlots };

/// We assume that an on-initialize consumes 1% of the weight on average, hence a single extrinsic
/// will not be allowed to consume more than `AvailableBlockRatio - 1%`.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(1);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.saturating_mul(2);

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;
/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND.ref_time() / GAS_PER_SECOND;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

// Configure FRAME pallets to include in runtime.
parameter_types! {
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const BlockHashCount: BlockNumber = 2400;
	pub const SS58Prefix: u16 = 42;
	pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = Everything;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous RuntimeEvent type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = frame_system::weights::SubstrateWeight<Runtime>;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();
	/// The maximum number of consumers allowed on a single account.
	type MaxConsumers = ConstU32<16>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
	pub const ExistentialDeposit: Balance = currency::EXISTENSIAL_DEPOSIT;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Runtime>;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const TransactionByteFee: Balance = currency::TRANSACTION_BYTE_FEE;
	pub const OperationalFeeMultiplier: u8 = 5;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate =
		TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: <Signature as traits::Verify>::Signer,
		account: AccountId,
		nonce: Index,
	) -> Option<(RuntimeCall, <UncheckedExtrinsic as traits::Extrinsic>::SignaturePayload)> {
		let tip = 0;
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let era = generic::Era::mortal(period, current_block);
		let extra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(era),
			frame_system::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = <Runtime as frame_system::Config>::Lookup::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}

impl offchain::SigningTypes for Runtime {
	type Public = <Signature as traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
}

impl offchain::AppCrypto<<Signature as Verify>::Signer, Signature> for OctopusAppCrypto {
	type RuntimeAppPublic = pallet_octopus_appchain::sr25519::AuthorityId;
	type GenericSignature = sr25519::Signature;
	type GenericPublic = sr25519::Public;
}

parameter_types! {
	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const EpochDuration: Moment = EPOCH_DURATION_IN_SLOTS;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const MaxAuthorities: u32 = 100;
	pub const ReportLongevity: Moment = BondingDuration::get() as Moment * SessionsPerEra::get() as Moment * EpochDuration::get();
}

impl pallet_babe::Config for Runtime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = pallet_babe::ExternalTrigger;
	type DisabledValidators = Session;

	type KeyOwnerProofSystem = Historical;

	type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		pallet_babe::AuthorityId,
	)>>::Proof;

	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		pallet_babe::AuthorityId,
	)>>::IdentificationTuple;

	type HandleEquivocation = pallet_babe::EquivocationHandler<
		Self::KeyOwnerIdentification,
		pallet_octopus_lpos::FilterHistoricalOffences<OctopusLpos, Offences>,
		ReportLongevity,
	>;

	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const CollectionDeposit: Balance = 100 * currency::DOLLARS;
	pub const ItemDeposit: Balance = 1 * currency::DOLLARS;
	pub const KeyLimit: u32 = 32;
	pub const ValueLimit: u32 = 256;
}

pub type CollectionId = u128;
pub type ItemId = u128;

impl pallet_uniques::Config<pallet_uniques::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = CollectionId;
	type ItemId = ItemId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = pallet_uniques::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
}

parameter_types! {
	pub const ApprovalDeposit: Balance = currency::DOLLARS;
	pub const AssetAccountDeposit: Balance = currency::DOLLARS;
	pub const AssetDeposit: Balance = 100 * currency::DOLLARS;
	pub const MetadataDepositBase: Balance = 10 * currency::DOLLARS;
	pub const MetadataDepositPerByte: Balance = currency::DOLLARS;
	pub const StringLimit: u32 = 2048;
}

impl pallet_assets::Config<pallet_assets::Instance1> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = AssetBalance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type KeyOwnerProofSystem = Historical;

	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;

	type HandleEquivocation = pallet_grandpa::EquivocationHandler<
		Self::KeyOwnerIdentification,
		pallet_octopus_lpos::FilterHistoricalOffences<OctopusLpos, Offences>,
		ReportLongevity,
	>;

	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type RuntimeEvent = RuntimeEvent;
	type NextSessionRotation = Babe;
	type ValidatorSet = Historical;
	type ReportUnresponsiveness =
		pallet_octopus_lpos::FilterHistoricalOffences<OctopusLpos, Offences>;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
	type MaxKeys = MaxKeys;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
	type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
}

impl pallet_offences::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = ();
}

impl pallet_beefy::Config for Runtime {
	type BeefyId = BeefyId;
	type MaxAuthorities = MaxAuthorities;
	type OnNewValidatorSet = MmrLeaf;
}

parameter_types! {
	/// Version of the produced MMR leaf.
	///
	/// The version consists of two parts;
	/// - `major` (3 bits)
	/// - `minor` (5 bits)
	///
	/// `major` should be updated only if decoding the previous MMR Leaf format from the payload
	/// is not possible (i.e. backward incompatible change).
	/// `minor` should be updated if fields are added to the previous MMR Leaf, which given SCALE
	/// encoding does not prevent old leafs from being decoded.
	///
	/// Hence we expect `major` to be changed really rarely (think never).
	/// See [`MmrLeafVersion`] type documentation for more details.
	pub LeafVersion: MmrLeafVersion = MmrLeafVersion::new(0, 0);
}

impl pallet_beefy_mmr::Config for Runtime {
	type LeafVersion = LeafVersion;
	type BeefyAuthorityToMerkleLeaf = pallet_beefy_mmr::BeefyEcdsaToEthereum;
	type LeafExtra = Vec<u8>;
	type BeefyDataProvider = ();
}

type MmrHash = <Keccak256 as sp_runtime::traits::Hash>::Output;

impl pallet_mmr::Config for Runtime {
	const INDEXING_PREFIX: &'static [u8] = b"mmr";
	type Hashing = Keccak256;
	type Hash = MmrHash;
	type OnNewRoot = pallet_beefy_mmr::DepositBeefyDigest<Runtime>;
	type WeightInfo = ();
	type LeafData = pallet_beefy_mmr::Pallet<Runtime>;
}

parameter_types! {
	pub const OctopusPalletId: PalletId = PalletId(*b"py/octps");
	pub const GracePeriod: u32 = 10;
	pub const UnsignedPriority: u64 = 1 << 21;
	pub const RequestEventLimit: u32 = 10;
}

impl pallet_octopus_appchain::Config for Runtime {
	type AuthorityId = pallet_octopus_appchain::sr25519::AuthorityId;
	type AppCrypto = OctopusAppCrypto;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type BridgeInterface = OctopusBridge;
	type LposInterface = OctopusLpos;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type GracePeriod = GracePeriod;
	type UnsignedPriority = UnsignedPriority;
	type MaxValidators = MaxAuthorities;
	type RequestEventLimit = RequestEventLimit;
	type WeightInfo = pallet_octopus_appchain::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const NativeTokenDecimals: u128 = 1_000_000_000_000_000_000;
	pub const FeeTh: u64 = 300;
}

impl pallet_octopus_bridge::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = OctopusPalletId;
	type Currency = Balances;
	type AppchainInterface = OctopusAppchain;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type AssetIdByTokenId = OctopusBridge;
	type AssetId = AssetId;
	type AssetBalance = AssetBalance;
	type Fungibles = OctopusAssets;
	type CollectionId = CollectionId;
	type ItemId = ItemId;
	type Nonfungibles = OctopusUniques;
	type Convertor = UniqueOneBaseMetadataConvertor<Runtime>;
	type NativeTokenDecimals = NativeTokenDecimals;
	type Threshold = FeeTh;
	type WeightInfo = pallet_octopus_bridge::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	pub const BondingDuration: pallet_octopus_lpos::EraIndex = 24 * 21;
}

impl pallet_octopus_lpos::Config for Runtime {
	type Currency = Balances;
	type UnixTime = Timestamp;
	type RuntimeEvent = RuntimeEvent;
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SessionInterface = Self;
	type AppchainInterface = OctopusAppchain;
	type UpwardMessagesInterface = OctopusUpwardMessages;
	type PalletId = OctopusPalletId;
	type WeightInfo = pallet_octopus_lpos::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MaxMessagePayloadSize: u32 = 2048;
	pub const MaxMessagesPerCommit: u32 = 20;
}

impl pallet_octopus_upward_messages::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Hashing = Keccak256;
	type MaxMessagePayloadSize = MaxMessagePayloadSize;
	type MaxMessagesPerCommit = MaxMessagesPerCommit;
	type WeightInfo = pallet_octopus_upward_messages::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 0;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (OctopusLpos, ImOnline);
}

// impl pallet_session_historical::Config for Runtime {
// 	type FullIdentification = u128;
// 	type FullIdentificationOf = pallet_octopus_lpos::ExposureOf<Runtime>;
// }

impl_opaque_keys! {
	pub struct SessionKeys {
		pub babe: Babe,
		pub grandpa: Grandpa,
		pub im_online: ImOnline,
		pub beefy: Beefy,
		pub octopus: OctopusAppchain,
	}
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = Babe;
	type NextSessionRotation = Babe;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, OctopusLpos>;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = u128;
	type FullIdentificationOf = pallet_octopus_lpos::ExposureOf<Runtime>;
}

impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_treasury::Config,
	pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, 80% are burned, 20% to the treasury
			let (_, to_treasury) = fees.ration(80, 20);
			// Balances pallet automatically burns dropped Negative Imbalances by decreasing
			// total_supply accordingly
			<pallet_treasury::Pallet<R> as OnUnbalanced<_>>::on_unbalanced(to_treasury);
		}
	}

	// this is called from pallet_evm for Ethereum-based transactions
	// (technically, it calls on_unbalanced, which calls this when non-zero)
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		// Balances pallet automatically burns dropped Negative Imbalances by decreasing
		// total_supply accordingly
		let (_, to_treasury) = amount.ration(80, 20);
		<pallet_treasury::Pallet<R> as OnUnbalanced<_>>::on_unbalanced(to_treasury);
	}
}

parameter_types! {
	pub const MaxApprovals: u32 = 100;
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = currency::DOLLARS * currency::SUPPLY_FACTOR;
	pub const SpendPeriod: BlockNumber = 6 * DAYS;
	pub const TreasuryId: PalletId = PalletId(*b"pc/trsry");
}

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = EnsureRoot<AccountId>;
	type Burn = ();
	type BurnDestination = ();
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type MaxApprovals = MaxApprovals;
	type OnSlash = Treasury;
	type PalletId = TreasuryId;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type RejectOrigin = EnsureRoot<AccountId>;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u128>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const BasicDeposit: Balance = currency::deposit(1, 258);      // 258 bytes on-chain
	pub const FieldDeposit: Balance = currency::deposit(0, 66);       // 66 bytes on-chain
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
	pub const SubAccountDeposit: Balance = currency::deposit(1, 53);   // 53 bytes on-chain
}

impl pallet_identity::Config for Runtime {
	type BasicDeposit = BasicDeposit;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type FieldDeposit = FieldDeposit;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type Slashed = Treasury;
	type SubAccountDeposit = SubAccountDeposit;
	type WeightInfo = ();
}

parameter_types! {
	/// The maximum number of council members.
	pub const CouncilMaxMembers: u32 = 100;
	/// The maximum number of Proposlas that can be open in the council at once.
	pub const CouncilMaxProposals: u32 = 100;
	/// The maximum amount of time (in blocks) for council members to vote on motions.
	/// Motions may end in fewer blocks if enough votes are cast to determine the result.
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
}

impl pallet_collective::Config<CouncilInstance> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type RuntimeEvent = RuntimeEvent;
	type MaxMembers = CouncilMaxMembers;
	type MaxProposals = CouncilMaxProposals;
	type MotionDuration = CouncilMotionDuration;
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type WeightInfo = ();
}

parameter_types! {
	/// The maximum number of technical committee members.
	pub const TechComitteeMaxMembers: u32 = 100;
	/// The maximum number of Proposlas that can be open in the technical committee at once.
	pub const TechComitteeMaxProposals: u32 = 100;
	/// The maximum amount of time (in blocks) for technical committee members to vote on motions.
	/// Motions may end in fewer blocks if enough votes are cast to determine the result.
	pub const TechComitteeMotionDuration: BlockNumber = 7 * DAYS;
}

impl pallet_collective::Config<TechCommitteeInstance> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type RuntimeEvent = RuntimeEvent;
	type MaxMembers = TechComitteeMaxMembers;
	type MaxProposals = TechComitteeMaxProposals;
	type MotionDuration = TechComitteeMotionDuration;
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type WeightInfo = ();
}

parameter_types! {
	pub const CooloffPeriod: BlockNumber = 7 * DAYS;
	pub const EnactmentPeriod: BlockNumber = 1 * DAYS;
	pub const FastTrackVotingPeriod: BlockNumber = 3 * HOURS;
	pub const InstantAllowed: bool = true;
	pub const LaunchPeriod: BlockNumber = 1 * DAYS;
	pub const MaxProposals: u32 = 100;
	pub const MaxVotes: u32 = 100;
	pub const MinimumDeposit: Balance = 4 * currency::DOLLARS * currency::SUPPLY_FACTOR;
	pub const PreimageByteDeposit: Balance = currency::STORAGE_BYTE_FEE;
	pub const VotingPeriod: BlockNumber = 5 * DAYS;
	pub const VoteLockingPeriod: BlockNumber = 1 * DAYS;
}

// todo : ensure better origins
impl pallet_democracy::Config for Runtime {
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = VoteLockingPeriod;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	type MinimumDeposit = MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 2>;
	/// A majority can have the next scheduled referendum be a straight majority-carries vote.
	type ExternalMajorityOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 3, 5>;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 3, 5>;
	/// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, TechCommitteeInstance, 1, 2>;
	/// Instant is currently not allowed.
	type InstantOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, TechCommitteeInstance, 3, 5>;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 3, 5>,
	>;
	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
	// Root must agree.
	type CancelProposalOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, TechCommitteeInstance, 3, 5>,
	>;
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cooloff period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechCommitteeInstance>;
	type CooloffPeriod = CooloffPeriod;
	type PreimageByteDeposit = PreimageByteDeposit;
	type Slash = ();
	type InstantAllowed = InstantAllowed;
	type Scheduler = Scheduler;
	type MaxVotes = MaxVotes;
	type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilInstance>;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
	type MaxProposals = MaxProposals;
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = NORMAL_DISPATCH_RATIO * RuntimeBlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MaximumWeight = MaximumSchedulerWeight;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type RuntimeOrigin = RuntimeOrigin;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PalletsOrigin = OriginCaller;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type PreimageProvider = ();
	type NoPreimagePostponement = ();
}

impl pallet_sudo::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub AnnouncementDepositBase: Balance = currency::deposit(1, 8);
	pub AnnouncementDepositFactor: Balance = currency::deposit(0, 66);
	// One storage item; key size 32, value size 8; .
	pub ProxyDepositBase: Balance = currency::deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub ProxyDepositFactor: Balance = currency::deposit(0, 33);
	pub const MaxPending: u16 = 32;
	pub const MaxProxies: u16 = 32;
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => {
				matches!(
					c,
					RuntimeCall::System(..) |
					RuntimeCall::Timestamp(..) |
					// RuntimeCall::Session(..) |
					RuntimeCall::Democracy(..) |
					RuntimeCall::CouncilCollective(..) |
					RuntimeCall::TechComitteeCollective(..) |
					// RuntimeCall::Treasury(..) |
					RuntimeCall::Utility(..) |
					RuntimeCall::Scheduler(..) |
					RuntimeCall::Proxy(..)
				)
			},
			ProxyType::Governance => matches!(
				c,
				RuntimeCall::Democracy(..) |
				RuntimeCall::CouncilCollective(..) |
				RuntimeCall::TechComitteeCollective(..) |
				// RuntimeCall::Treasury(..) |
				RuntimeCall::Utility(..)
			),
			ProxyType::Staking => matches!(c, RuntimeCall::Utility(..)),
			ProxyType::CancelProxy => {
				matches!(c, RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. }))
			},
			ProxyType::Balances => {
				matches!(c, RuntimeCall::Balances(..) | RuntimeCall::Utility(..))
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
	type RuntimeCall = RuntimeCall;
	type CallHasher = BlakeTwo256;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type MaxPending = MaxPending;
	type MaxProxies = MaxProxies;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type ProxyType = ProxyType;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxValueSize: u32 = 16 * 1024;
	// The lazy deletion runs inside on_initialize.
	pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
		RuntimeBlockWeights::get().max_block;
	//pub DeletionQueueDepth: u32 = 128;
	// The weight needed for decoding the queue should be less or equal than a fifth
	// of the overall weight dedicated to the lazy deletion.
	pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get().ref_time() / (
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1).ref_time()  -
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0).ref_time()
		)) / 5) as u32;
	pub Schedule: pallet_contracts::Schedule<Runtime> = Default::default();
	pub const DepositPerByte: Balance = deposit(1, 0);
	pub const DepositPerItem: Balance = deposit(0, 1);
}

impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `Call` structure itself
	/// is not allowed to change the indices of existing pallets, too.
	type CallFilter = Nothing;
	type CallStack = [pallet_contracts::Frame<Self>; 31];
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = ();
	type DeletionQueueDepth = DeletionQueueDepth;
	type DeletionWeightLimit = DeletionWeightLimit;
	type Schedule = Schedule;
	type DepositPerByte = DepositPerByte;
	type DepositPerItem = DepositPerItem;
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	// This node is geared towards development and testing of contracts.
	// We decided to increase the default allowed contract size for this
	// reason (the default is `128 * 1024`).
	//
	// Our reasoning is that the error code `CodeTooLarge` is thrown
	// if a too-large contract is uploaded. We noticed that it poses
	// less friction during development when the requirement here is
	// just more lax.
	type MaxCodeLen = ConstU32<{ 256 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
	type ContractAccessWeight = ();
}

unet_orml_traits::parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		if currency_id == &unet_traits::constants_types::NATIVE_CURRENCY_ID {
			ExistentialDeposit::get()
		} else  {
			Default::default()
		}
	};
}

impl unet_orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = unet_traits::constants_types::NATIVE_CURRENCY_ID;
}

pub type AdaptedBasicCurrency = unet_orml_currencies::BasicCurrencyAdapter<
	Runtime,
	Balances,
	Amount,
	unet_traits::constants_types::Moment,
>;

impl unet_orml_currencies::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

impl unet_orml_nft::Config for Runtime {
	type ClassId = unet_traits::ClassId;
	type TokenId = unet_traits::TokenId;
	type ClassData = unet_traits::ClassData<BlockNumber>;
	type TokenData = unet_traits::TokenData<AccountId, BlockNumber>;
}

impl unet_config::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const CreateClassDeposit: Balance = 20 * currency::MILLICENTS * currency::SUPPLY_FACTOR;
	pub const CreateTokenDeposit: Balance = 10 * currency::MILLICENTS * currency::SUPPLY_FACTOR;
	pub const MetaDataByteDeposit: Balance = 1 * currency::MILLICENTS * currency::SUPPLY_FACTOR;
	pub const NftModuleId: PalletId = PalletId(*b"unetnft*");
}

impl unet_nft::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ExtraConfig = UnetConf;
	type CreateClassDeposit = CreateClassDeposit;
	type MetaDataByteDeposit = MetaDataByteDeposit;
	type CreateTokenDeposit = CreateTokenDeposit;
	type ModuleId = NftModuleId;
	type Currency = Balances;
	type MultiCurrency = Currencies;
	type WeightInfo = ();
}

impl unet_order::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Currencies;
	type Currency = Balances;
	type ClassId = unet_traits::ClassId;
	type TokenId = unet_traits::TokenId;
	type NFT = UnetNft;
	type ExtraConfig = UnetConf;
	type TreasuryPalletId = TreasuryId;
	type WeightInfo = ();
}

impl unet_auction::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Currencies;
	type Currency = Balances;
	type ClassId = unet_traits::ClassId;
	type TokenId = unet_traits::TokenId;
	type NFT = UnetNft;
	type ExtraConfig = UnetConf;
	type TreasuryPalletId = TreasuryId;
	type WeightInfo = ();
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = generic::Block<Header, OpaqueExtrinsic>,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Call, Config, Event<T>, Pallet, Storage},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		Balances: pallet_balances::{Call, Config<T>, Event<T>, Pallet, Storage},
		TransactionPayment: pallet_transaction_payment::{Pallet, Event<T>, Storage},
		Babe: pallet_babe::{Call, Config, Pallet, Storage, ValidateUnsigned},
		Timestamp: pallet_timestamp::{Call, Inherent, Pallet, Storage},
		OctopusUniques: pallet_uniques::<Instance1>::{Call, Event<T>, Pallet, Storage},
		OctopusAssets: pallet_assets::<Instance1>::{Call, Config<T>, Event<T>, Pallet, Storage},
		Grandpa: pallet_grandpa::{Call, Config, Event, Pallet, Storage, ValidateUnsigned},
		ImOnline: pallet_im_online::{Call, Config<T>, Event<T>, Pallet, Storage, ValidateUnsigned},
		Offences: pallet_offences::{Event, Pallet, Storage},
		Beefy: pallet_beefy::{Config<T>, Pallet, Storage},
		MmrLeaf: pallet_beefy_mmr::{Pallet, Storage},
		Mmr: pallet_mmr::{Pallet, Storage},
		OctopusAppchain: pallet_octopus_appchain::{Call, Config<T>, Event<T>, Pallet, Storage, ValidateUnsigned},
		OctopusBridge: pallet_octopus_bridge::{Call, Config<T>, Event<T>, Pallet, Storage},
		OctopusLpos: pallet_octopus_lpos::{Call, Config, Event<T>, Pallet, Storage},
		OctopusUpwardMessages: pallet_octopus_upward_messages::{Call, Config<T>, Event<T>, Pallet, Storage},
		Authorship: pallet_authorship::{Call, Inherent, Pallet, Storage},
		Session: pallet_session::{Call, Event, Config<T>, Pallet, Storage},
		Historical: pallet_session_historical::{Pallet},
		Treasury: pallet_treasury::{Call, Config, Event<T>, Pallet, Storage},
		Identity: pallet_identity::{Call, Event<T>, Pallet, Storage},
		CouncilCollective: pallet_collective::<Instance1>::{Call, Config<T>, Event<T>, Origin<T>, Pallet, Storage},
		TechComitteeCollective: pallet_collective::<Instance2>::{Call, Config<T>, Event<T>, Origin<T>, Pallet, Storage},
		Democracy: pallet_democracy::{Call, Config<T>, Event<T>, Pallet, Storage},
		Utility: pallet_utility::{Call, Event, Pallet},
		Scheduler: pallet_scheduler::{Call, Event<T>, Pallet, Storage},
		Sudo: pallet_sudo::{Call, Config<T>, Event<T>, Pallet, Storage},
		Proxy: pallet_proxy::{Call, Event<T>, Pallet, Storage},
		Contracts: pallet_contracts::{Call, Event<T>, Pallet, Storage},
		Currencies: unet_orml_currencies::{Call, Event<T>, Pallet},
		Tokens: unet_orml_tokens::{Config<T>, Event<T>, Pallet, Storage},
		OrmlNFT: unet_orml_nft::{Config<T>, Pallet, Storage},
		UnetConf: unet_config::{Call, Config<T>, Event<T>, Pallet, Storage},
		UnetNft: unet_nft::{Call, Event<T>, Config<T>, Pallet, Storage},
		UnetOrder: unet_order::{Call, Config<T>, Event<T>, Pallet, Storage},
		UnetAuction: unet_auction::{Call, Event<T>, Config<T>, Pallet, Storage},
	}
);

pub type MmrHashing = <Runtime as pallet_mmr::Config>::Hashing;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_babe::BabeApi<Block> for Runtime {
		fn configuration() -> BabeConfiguration {
			// The choice of `c` parameter (where `1 - c` represents the
			// probability of a slot being empty), is done in accordance to the
			// slot duration and expected target block time, for safely
			// resisting network delays of maximum two seconds.
			// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
			BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: BABE_GENESIS_EPOCH_CONFIG.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: BABE_GENESIS_EPOCH_CONFIG.allowed_slots,
			}
		}

		fn current_epoch_start() -> Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: Slot,
			authority_id: BabeId,
		) -> Option<OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((fg_primitives::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(fg_primitives::OpaqueKeyOwnershipProof::new)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(call: RuntimeCall, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(call: RuntimeCall, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
	}


	impl mmr::MmrApi<Block, Hash> for Runtime {
		fn generate_proof(leaf_index: u64)
			-> Result<(mmr::EncodableOpaqueLeaf, mmr::Proof<Hash>), mmr::Error>
		{
			Mmr::generate_batch_proof(vec![leaf_index])
				.and_then(|(leaves, proof)| Ok((
					mmr::EncodableOpaqueLeaf::from_leaf(&leaves[0]),
					mmr::BatchProof::into_single_leaf_proof(proof)?
				)))
		}

		fn verify_proof(leaf: mmr::EncodableOpaqueLeaf, proof: mmr::Proof<Hash>)
			-> Result<(), mmr::Error>
		{
			pub type MmrLeaf = <<Runtime as pallet_mmr::Config>::LeafData as mmr::LeafDataProvider>::LeafData;
			let leaf: MmrLeaf = leaf
				.into_opaque_leaf()
				.try_decode()
				.ok_or(mmr::Error::Verify)?;
			Mmr::verify_leaves(vec![leaf], mmr::Proof::into_batch_proof(proof))
		}

		fn verify_proof_stateless(
			root: Hash,
			leaf: mmr::EncodableOpaqueLeaf,
			proof: mmr::Proof<Hash>
		) -> Result<(), mmr::Error> {
			let node = mmr::DataOrHash::Data(leaf.into_opaque_leaf());
			pallet_mmr::verify_leaves_proof::<MmrHashing, _>(root, vec![node], mmr::Proof::into_batch_proof(proof))
		}

		fn mmr_root() -> Result<Hash, mmr::Error> {
			Ok(Mmr::mmr_root())
		}

		fn generate_batch_proof(leaf_indices: Vec<mmr::LeafIndex>)
			-> Result<(Vec<mmr::EncodableOpaqueLeaf>, mmr::BatchProof<Hash>), mmr::Error>
		{
			Mmr::generate_batch_proof(leaf_indices)
				.map(|(leaves, proof)| (leaves.into_iter().map(|leaf| mmr::EncodableOpaqueLeaf::from_leaf(&leaf)).collect(), proof))
		}

		fn verify_batch_proof(leaves: Vec<mmr::EncodableOpaqueLeaf>, proof: mmr::BatchProof<Hash>)
			-> Result<(), mmr::Error>
		{
			pub type MmrLeaf = <<Runtime as pallet_mmr::Config>::LeafData as mmr::LeafDataProvider>::LeafData;
			let leaves = leaves.into_iter().map(|leaf|
				leaf.into_opaque_leaf()
				.try_decode()
				.ok_or(mmr::Error::Verify)).collect::<Result<Vec<MmrLeaf>, mmr::Error>>()?;
			Mmr::verify_leaves(leaves, proof)
		}

		fn verify_batch_proof_stateless(
			root: Hash,
			leaves: Vec<mmr::EncodableOpaqueLeaf>,
			proof: mmr::BatchProof<Hash>
		) -> Result<(), mmr::Error> {
			let nodes = leaves.into_iter().map(|leaf|mmr::DataOrHash::Data(leaf.into_opaque_leaf())).collect();
			pallet_mmr::verify_leaves_proof::<MmrHashing, _>(root, nodes, proof)
		}
	}

	impl beefy_primitives::BeefyApi<Block> for Runtime {
		fn validator_set() -> Option<beefy_primitives::ValidatorSet<BeefyId>> {
			Beefy::validator_set()
		}
	}

	impl beefy_merkle_tree::BeefyMmrApi<Block, Hash> for RuntimeApi {
		fn authority_set_proof() -> beefy_primitives::mmr::BeefyAuthoritySet<Hash> {
			MmrLeaf::authority_set_proof()
		}

		fn next_authority_set_proof() -> beefy_primitives::mmr::BeefyNextAuthoritySet<Hash> {
			MmrLeaf::next_authority_set_proof()
		}
	}

	impl pallet_contracts_rpc_runtime_api::ContractsApi<
		Block, AccountId, Balance, BlockNumber, Hash,
	>
		for Runtime
	{
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: u64,
			storage_deposit_limit: Option<Balance>,
			input_data: Vec<u8>,
		) -> pallet_contracts_primitives::ContractExecResult<Balance> {

			Contracts::bare_call(origin, dest, value, Weight::from_ref_time(gas_limit), storage_deposit_limit, input_data, true)
		}

		fn instantiate(
			origin: AccountId,
			endowment: Balance,
			gas_limit: u64,
			storage_deposit_limit: Option<Balance>,
			code: pallet_contracts_primitives::Code<Hash>,
			data: Vec<u8>,
			salt: Vec<u8>,
		) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance>
		{
			Contracts::bare_instantiate(origin, endowment, Weight::from_ref_time(gas_limit), storage_deposit_limit, code, data, salt, true)
		}

		fn upload_code(
			origin: AccountId,
			code: Vec<u8>,
			storage_deposit_limit: Option<Balance>,
		) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance> {
			Contracts::bare_upload_code(origin, code, storage_deposit_limit)
		}

		fn get_storage(
			address: AccountId,
			key: Vec<u8>,
		) -> pallet_contracts_primitives::GetStorageResult {
			Contracts::get_storage(address, key)
		}
	}

	impl unet_rpc_runtime_api::UnetApi<Block> for Runtime {
		fn mint_token_deposit(metadata_len: u32) -> Balance {
			UnetNft::mint_token_deposit(metadata_len)
		}
		fn add_class_admin_deposit(admin_count: u32) -> Balance {
			UnetNft::add_class_admin_deposit(admin_count)
		}
		fn create_class_deposit(metadata_len: u32, name_len: u32, description_len: u32) -> (Balance, Balance) {
			UnetNft::create_class_deposit(metadata_len, name_len, description_len)
		}
		fn get_dutch_auction_current_price(
			max_price: Balance, min_price: Balance,
			created_block: BlockNumber,
			deadline: BlockNumber,
			current_block: BlockNumber,
		) -> Balance {
			unet_auction::calc_current_price::<Runtime>(max_price, min_price, created_block, deadline, current_block)
		}
		fn get_auction_deadline(
			allow_delay: bool, deadline: BlockNumber, last_bid_block: BlockNumber
		) -> BlockNumber {
			unet_auction::get_deadline::<Runtime>(allow_delay, deadline, last_bid_block)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade().unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			log::info!(
				target: "node-runtime",
				"try-runtime: executing block {:?} / root checks: {:?} / try-state-select: {:?}",
				block.header.hash(),
				state_root_check,
				select,
			);
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, select).unwrap()
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
			use frame_benchmarking::baseline::Pallet as BaselineBench;
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			// use unet_auction::Pallet as UnetAuctionBench;

			let mut list = Vec::<BenchmarkList>::new();

			// list_benchmark!(list, extra, frame_benchmarking, BaselineBench::<Runtime>);
			// list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			// list_benchmark!(list, extra, pallet_balances, Balances);
			// list_benchmark!(list, extra, pallet_timestamp, Timestamp);
			// list_benchmark!(list, extra, unet_auction, UnetAuction);
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{add_benchmark, Benchmarking, BenchmarkBatch, TrackedStorageKey};
			impl frame_benchmarking::baseline::Config for Runtime {}
			use frame_system_benchmarking::Pallet as SystemBench;
			use frame_benchmarking::baseline::Pallet as BaselineBench;
			impl frame_system_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			// add_benchmark!(params, batches, frame_benchmarking, BaselineBench::<Runtime>);
			// add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			// add_benchmark!(params, batches, pallet_balances, Balances);
			// add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			// add_benchmark!(params, batches, unet_auction, UnetAuction);

			add_benchmarks!(params, batches);

			Ok(batches)
		}
	}
}
