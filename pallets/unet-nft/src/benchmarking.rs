#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::*;
use crate::{utils::test_helper::*, Pallet as UnetNft};

use frame_benchmarking::{
	account, benchmarks, impl_benchmark_test_suite, whitelist_account, whitelisted_caller,
};
use frame_system::RawOrigin;
use sp_std::vec::Vec;
const SEED: u32 = 0;

benchmarks! {
	create_class {
		let alice: T::AccountId = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&bob, balances!(60000));

		add_whitelist::<T>(&alice);
	}: { UnetNft::<T>::create_class(
			RawOrigin::Signed(alice.clone()).into(),
			Vec::from("1"),
			Vec::from("1"),
			Vec::from("1"),
			PerU16::from_percent(5),
			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
			vec![0, 1],
		)?;
	}

	proxy_mint {
		let alice: T::AccountId = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&bob, balances!(60000));

		add_whitelist::<T>(&alice);

		let class_id = unet_orml_nft::NextClassId::<T>::get();
		UnetNft::<T>::create_class(
			RawOrigin::Signed(alice.clone()).into(),
			Vec::from("1"),
			Vec::from("1"),
			Vec::from("1"),
			PerU16::from_percent(5),
			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
			vec![0, 1],
		)?;
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(alice.clone());
	}: {
		UnetNft::<T>::proxy_mint(
			RawOrigin::Signed(alice.clone()).into(),
			recipient_lookup,
			class_id,
			Vec::from("1"),
			into!(1),
			Some(PerU16::from_percent(5)),
		)?;
	}

	transfer {
		let c in 1 .. 100;
		let alice: T::AccountId = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&bob, balances!(60000));

		add_whitelist::<T>(&alice);

		let class_id = unet_orml_nft::NextClassId::<T>::get();
		UnetNft::<T>::create_class(
			RawOrigin::Signed(alice.clone()).into(),
			Vec::from("1"),
			Vec::from("1"),
			Vec::from("1"),
			PerU16::from_percent(5),
			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
			vec![0, 1],
		)?;

		let owner: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(alice.clone());
		let to: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

		UnetNft::<T>::proxy_mint(
			RawOrigin::Signed(alice.clone()).into(),
			owner.clone(),
			class_id,
			Vec::from("1"),
			into!(1),
			Some(PerU16::from_percent(5)),
		)?;

		let mut tokens = Vec::new();
		tokens.push((class_id, into!(0), into!(1)));
		for i in 0..(c-1) {
			UnetNft::<T>::proxy_mint(
				RawOrigin::Signed(alice.clone()).into(),
				owner.clone(),
				class_id,
				Vec::from("1"),
				into!(1),
				Some(PerU16::from_percent(5)),
			)?;
			tokens.push((class_id, into!(i+1), into!(1)));
		}
	}: {
		UnetNft::<T>::transfer(
			RawOrigin::Signed(alice.clone()).into(),
			to,
			tokens,
		)?;
	}

	burn {
		let alice: T::AccountId = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&bob, balances!(60000));

		add_whitelist::<T>(&alice);

		let class_id = unet_orml_nft::NextClassId::<T>::get();
		UnetNft::<T>::create_class(
			RawOrigin::Signed(alice.clone()).into(),
			Vec::from("1"),
			Vec::from("1"),
			Vec::from("1"),
			PerU16::from_percent(5),
			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
			vec![0, 1],
		)?;

		let owner: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(alice.clone());

		UnetNft::<T>::proxy_mint(
			RawOrigin::Signed(alice.clone()).into(),
			owner,
			class_id,
			Vec::from("1"),
			into!(1),
			Some(PerU16::from_percent(5)),
		)?;
	}: {
		UnetNft::<T>::burn(
			RawOrigin::Signed(alice.clone()).into(),
			class_id,
			into!(0),
			into!(1),
		)?;
	}

	update_token_royalty {
		let alice: T::AccountId = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&bob, balances!(60000));

		add_whitelist::<T>(&alice);

		let class_id = unet_orml_nft::NextClassId::<T>::get();
		UnetNft::<T>::create_class(
			RawOrigin::Signed(alice.clone()).into(),
			Vec::from("1"),
			Vec::from("1"),
			Vec::from("1"),
			PerU16::from_percent(5),
			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
			vec![0, 1],
		)?;

		let owner: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(alice.clone());

		UnetNft::<T>::proxy_mint(
			RawOrigin::Signed(alice.clone()).into(),
			owner,
			class_id,
			Vec::from("1"),
			into!(1),
			Some(PerU16::from_percent(5)),
		)?;
	}: {
		UnetNft::<T>::update_token_royalty(
			RawOrigin::Signed(alice.clone()).into(),
			class_id,
			into!(0),
			Some(PerU16::from_percent(5)),
		)?;
	}

	update_token_royalty_beneficiary {
		let alice: T::AccountId = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = <T as pallet::Config>::Currency::make_free_balance_be(&bob, balances!(60000));

		add_whitelist::<T>(&alice);

		let class_id = unet_orml_nft::NextClassId::<T>::get();
		UnetNft::<T>::create_class(
			RawOrigin::Signed(alice.clone()).into(),
			Vec::from("1"),
			Vec::from("1"),
			Vec::from("1"),
			PerU16::from_percent(5),
			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
			vec![0, 1],
		)?;

		let owner: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(alice.clone());
		let to: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

		UnetNft::<T>::proxy_mint(
			RawOrigin::Signed(alice.clone()).into(),
			owner,
			class_id,
			Vec::from("1"),
			into!(1),
			Some(PerU16::from_percent(5)),
		)?;
	}: {
		UnetNft::<T>::update_token_royalty_beneficiary(
			RawOrigin::Signed(alice.clone()).into(),
			class_id,
			into!(0),
			to,
		)?;
	}
}

impl_benchmark_test_suite!(UnetNft, crate::mock::new_test_ext(), crate::mock::Runtime,);
