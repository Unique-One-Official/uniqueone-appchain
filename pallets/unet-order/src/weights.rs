//! Autogenerated weights for unet_order
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-10, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/uniqueone-appchain
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=unet_order
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --template=./.maintain/pallet-weight-template.hbs
// --output=./pallets/unet-order/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for unet_order.
pub trait WeightInfo {
	fn submit_order(c: u32, ) -> Weight;
	fn submit_offer(c: u32, ) -> Weight;
	fn take_order() -> Weight;
	fn take_offer() -> Weight;
	fn remove_order() -> Weight;
	fn remove_offer() -> Weight;
}

/// Weights for unet_order using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: UnetConf MaxCommissionRewardRate (r:1 w:0)
	// Storage: UnetConf MinOrderDeposit (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: OrmlNFT TokensByOwner (r:1 w:1)
	// Storage: UnetConf NextId (r:1 w:1)
	// Storage: UnetOrder Orders (r:0 w:1)
	fn submit_order(c: u32, ) -> Weight {
		17_587_000_u64			// Standard Error: 81_000
			.saturating_add((10_585_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	// Storage: UnetConf MaxCommissionRewardRate (r:1 w:0)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: UnetConf NextId (r:1 w:1)
	// Storage: UnetOrder Offers (r:0 w:1)
	fn submit_offer(c: u32, ) -> Weight {
		16_481_000_u64			// Standard Error: 54_000
			.saturating_add((5_410_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: UnetOrder Orders (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: OrmlNFT TokensByOwner (r:2 w:2)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: UnetConf PlatformFeeRate (r:1 w:0)
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Storage: OrmlNFT OwnersByToken (r:0 w:1)
	fn take_order() -> Weight {
		112_000_000_u64
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: UnetOrder Offers (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: UnetConf PlatformFeeRate (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Storage: OrmlNFT TokensByOwner (r:2 w:2)
	// Storage: OrmlNFT OwnersByToken (r:0 w:1)
	fn take_offer() -> Weight {
		109_000_000_u64
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	// Storage: UnetOrder Orders (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT TokensByOwner (r:1 w:1)
	fn remove_order() -> Weight {
		35_000_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: UnetOrder Offers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_offer() -> Weight {
		29_000_000_u64
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: UnetConf MaxCommissionRewardRate (r:1 w:0)
	// Storage: UnetConf MinOrderDeposit (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: OrmlNFT TokensByOwner (r:1 w:1)
	// Storage: UnetConf NextId (r:1 w:1)
	// Storage: UnetOrder Orders (r:0 w:1)
	fn submit_order(c: u32, ) -> Weight {
		17_587_000_u64			// Standard Error: 81_000
			.saturating_add((10_585_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(c as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c as Weight)))
	}
	// Storage: UnetConf MaxCommissionRewardRate (r:1 w:0)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: UnetConf NextId (r:1 w:1)
	// Storage: UnetOrder Offers (r:0 w:1)
	fn submit_offer(c: u32, ) -> Weight {
		16_481_000_u64			// Standard Error: 54_000
			.saturating_add((5_410_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(c as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: UnetOrder Orders (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: OrmlNFT TokensByOwner (r:2 w:2)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: UnetConf PlatformFeeRate (r:1 w:0)
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Storage: OrmlNFT OwnersByToken (r:0 w:1)
	fn take_order() -> Weight {
		112_000_000_u64
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	// Storage: UnetOrder Offers (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:0)
	// Storage: UnetConf PlatformFeeRate (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Storage: OrmlNFT TokensByOwner (r:2 w:2)
	// Storage: OrmlNFT OwnersByToken (r:0 w:1)
	fn take_offer() -> Weight {
		109_000_000_u64
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	// Storage: UnetOrder Orders (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT TokensByOwner (r:1 w:1)
	fn remove_order() -> Weight {
		35_000_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: UnetOrder Offers (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_offer() -> Weight {
		29_000_000_u64
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}