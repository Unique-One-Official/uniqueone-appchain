#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as UnetConf;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_whitelist {
		let caller_origin = T::Origin::from(RawOrigin::Root);
		let bob: T::AccountId = whitelisted_caller();
	}: {
		UnetConf::<T>::add_whitelist(caller_origin, bob)?;
	}

	remove_whitelist {
		let caller_origin = T::Origin::from(RawOrigin::Root);
		let bob: T::AccountId = whitelisted_caller();
	}: {
		UnetConf::<T>::remove_whitelist(caller_origin, bob)?;
	}

	create_category {
		let caller_origin = T::Origin::from(RawOrigin::Root);
	}: {
		UnetConf::<T>::create_category(caller_origin, Vec::from("1"),)?;
	}

	update_category {
		let caller_origin = T::Origin::from(RawOrigin::Root);
	}: {
		UnetConf::<T>::update_category(caller_origin, 1, Vec::from("2"),)?;
	}

	update_auction_close_delay {
		let caller_origin = T::Origin::from(RawOrigin::Root);
	}: {
		UnetConf::<T>::update_auction_close_delay(caller_origin, T::BlockNumber::default())?;
	}

	en_disable_whitelist {
		let caller_origin = T::Origin::from(RawOrigin::Root);
	}: {
		UnetConf::<T>::en_disable_whitelist(caller_origin)?;
	}
}
