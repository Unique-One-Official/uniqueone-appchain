//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
	fn transfer() -> Weight {
		(Weight::from_ref_time(158_835_000 as u64))
			.saturating_add(DbWeight::get().reads(4 as u64))
			.saturating_add(DbWeight::get().writes(2 as u64))
	}
	fn transfer_all() -> Weight {
		(Weight::from_ref_time(162_545_000 as u64))
			.saturating_add(DbWeight::get().reads(4 as u64))
			.saturating_add(DbWeight::get().writes(2 as u64))
	}
}
