#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{utils::test_helper::*, Pallet as UnetOrder};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use unet_traits::time::MINUTES;

const SEED: u32 = 0;

benchmarks! {
	submit_order {
		let c in 1 .. 100;

		let alice = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		add_whitelist::<T>(&alice);
		add_whitelist::<T>(&bob);

		let _ = T::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = T::Currency::make_free_balance_be(&bob, balances!(60000));

		let class_id = peek_next_class_id::<T>();
		add_class::<T>(alice.clone());

		let mut tokens = Vec::new();
		add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::from_percent(5)));
		tokens.push((class_id, into!(0), into!(10)));
		for i in 0..(c-1) {
			add_token::<T>(alice.clone(), alice.clone(), class_id, into!(40), Some(PerU16::zero()));
			tokens.push((class_id, into!(i+1), into!(10)));
		}

		let auction_id = current_gid::<T>();
	}: {
		UnetOrder::<T>::submit_order(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			into!(ACCURACY), // deposit
			200, // price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			PerU16::zero(),
		)?;
	}

	submit_offer {
		let c in 1 .. 100;

		let alice = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		add_whitelist::<T>(&alice);
		add_whitelist::<T>(&bob);

		let _ = T::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = T::Currency::make_free_balance_be(&bob, balances!(60000));

		let class_id = peek_next_class_id::<T>();
		add_class::<T>(alice.clone());

		let mut tokens = Vec::new();
		add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::from_percent(5)));
		tokens.push((class_id, into!(0), into!(10)));
		for i in 0..(c-1) {
			add_token::<T>(alice.clone(), alice.clone(), class_id, into!(40), Some(PerU16::zero()));
			tokens.push((class_id, into!(i+1), into!(10)));
		}

		let auction_id = current_gid::<T>();
	}: {
		UnetOrder::<T>::submit_offer(
			RawOrigin::Signed(bob.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			200, // price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			PerU16::zero(),
		)?;
	}

	take_order {
		let alice = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		add_whitelist::<T>(&alice);
		add_whitelist::<T>(&bob);

		let _ = T::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = T::Currency::make_free_balance_be(&bob, balances!(60000));

		let class_id = peek_next_class_id::<T>();
		add_class::<T>(alice.clone());

		let mut tokens = Vec::new();
		add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::from_percent(5)));
		tokens.push((class_id, into!(0), into!(10)));

		let auction_id = current_gid::<T>();
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(alice.clone());

		UnetOrder::<T>::submit_order(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			into!(ACCURACY), // deposit
			200, // price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			PerU16::zero(),
		)?;
	}: {
		UnetOrder::<T>::take_order(
			RawOrigin::Signed(bob.clone()).into(),
			auction_id,
			recipient_lookup,
			None,
			None,
		)?;
	}

	take_offer {
		let alice = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		add_whitelist::<T>(&alice);
		add_whitelist::<T>(&bob);

		let _ = T::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = T::Currency::make_free_balance_be(&bob, balances!(60000));

		let class_id = peek_next_class_id::<T>();
		add_class::<T>(alice.clone());

		let mut tokens = Vec::new();
		add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::from_percent(5)));
		tokens.push((class_id, into!(0), into!(10)));

		let auction_id = current_gid::<T>();
		let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

		UnetOrder::<T>::submit_offer(
			RawOrigin::Signed(bob.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			200, // price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			PerU16::zero(),
		)?;
	}: {
		UnetOrder::<T>::take_offer(
			RawOrigin::Signed(alice.clone()).into(),
			auction_id,
			recipient_lookup,
			None,
			None,
		)?;
	}

	remove_order {
		let alice = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		add_whitelist::<T>(&alice);
		add_whitelist::<T>(&bob);

		let _ = T::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = T::Currency::make_free_balance_be(&bob, balances!(60000));

		let class_id = peek_next_class_id::<T>();
		add_class::<T>(alice.clone());

		let mut tokens = Vec::new();
		add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::from_percent(5)));
		tokens.push((class_id, into!(0), into!(10)));

		let auction_id = current_gid::<T>();

		UnetOrder::<T>::submit_order(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			into!(ACCURACY), // deposit
			200, // price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			PerU16::zero(),
		)?;
	}: {
		UnetOrder::<T>::remove_order(
			RawOrigin::Signed(alice.clone()).into(),
			auction_id,
		)?;
	}

	remove_offer {
		let alice = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		add_whitelist::<T>(&alice);
		add_whitelist::<T>(&bob);

		let _ = T::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = T::Currency::make_free_balance_be(&bob, balances!(60000));

		let class_id = peek_next_class_id::<T>();
		add_class::<T>(alice.clone());

		let mut tokens = Vec::new();
		add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::from_percent(5)));
		tokens.push((class_id, into!(0), into!(10)));

		let auction_id = current_gid::<T>();

		UnetOrder::<T>::submit_offer(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			200, // price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			PerU16::zero(),
		)?;
	}: {
		UnetOrder::<T>::remove_offer(
			RawOrigin::Signed(alice.clone()).into(),
			auction_id,
		)?;
	}
}
