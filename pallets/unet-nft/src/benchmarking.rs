// #![cfg(feature = "runtime-benchmarks")]
//
// use super::*;
// use crate::{utils::test_helper::*, Pallet as UnetNft};
// use crate::*;
//
// use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller, whitelist_account};
// use frame_system::RawOrigin;
// use sp_std::vec::Vec;
// use unet_traits::time::MINUTES;
//
// const SEED: u32 = 0;
//
// benchmarks! {
// 	create_class {
// 		let caller: T::AccountId = whitelisted_caller();
// 		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
//
// 	}: { UnetNft::<T>::create_class(
// 			caller_origin,
// 			Vec::from("1"),
// 			Vec::from("1"),
// 			Vec::from("1"),
// 			PerU16::from_percent(5),
// 			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
// 			vec![0, 1],
// 		)?;
// 	}
//
// 	proxy_mint {
// 		let c in 1 .. 100;
//
// 		let alice: T::AccountId  = account("account", 0, SEED);
// 		let bob: T::AccountId = whitelisted_caller();
//
// 		// UnetNft::<T>::create_class(
// 		// 	RawOrigin::Signed(alice.clone()).into(),
// 		// 	Vec::from("1"),
// 		// 	Vec::from("1"),
// 		// 	Vec::from("1"),
// 		// 	PerU16::from_percent(5),
// 		// 	Properties(ClassProperty::Transferable | ClassProperty::Burnable),
// 		// 	vec![0, 1],
// 		// )?;
//
// 		let class_id = peek_next_class_id::<T>();
// 		add_class::<T>(alice.clone());
//
// 		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(alice.clone());
// 	}: {
// 		add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::zero()));
// 	}
//
// }
//
// impl_benchmark_test_suite!(UnetNft, crate::mock::new_test_ext(), crate::mock::Runtime,);
