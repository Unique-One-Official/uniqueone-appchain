#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

/// Weight functions needed for unet_nft.
pub trait WeightInfo {
	fn create_class() -> Weight;
	fn mint(i: u32, ) -> Weight;
	fn transfer() -> Weight;
	fn burn() -> Weight;
	fn destroy_class() -> Weight;
}


impl WeightInfo for () {
	fn create_class() -> Weight {
		(190_284_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn mint(i: u32, ) -> Weight {
		(59_950_000 as Weight)
			// Standard Error: 51_000
			.saturating_add((79_894_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
			.saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
	}
	fn transfer() -> Weight {
		(282_524_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(7 as Weight))
	}
	fn burn() -> Weight {
		(202_979_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn destroy_class() -> Weight {
		(229_885_000 as Weight)
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
}
