#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{utils::test_helper::*, Pallet as UnetAuction};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec::Vec;
use unet_traits::time::MINUTES;

const SEED: u32 = 0;

benchmarks! {
	submit_dutch_auction {
		let c in 1 .. MAX_TOKEN_PER_AUCTION;

		let alice = account("account", 0, SEED);
		let bob: T::AccountId = whitelisted_caller();

		add_whitelist::<T>(&alice);
		add_whitelist::<T>(&bob);

		let _ = T::Currency::make_free_balance_be(&alice, balances!(60000));
		let _ = T::Currency::make_free_balance_be(&bob, balances!(60000));

		let class_id = peek_next_class_id::<T>();
		add_class::<T>(alice.clone());

		let mut tokens = Vec::new();
		add_token::<T>(alice.clone(), bob.clone(), class_id, into!(20), Some(PerU16::from_percent(5)));
		tokens.push((class_id, into!(0), into!(10)));
		for i in 0..(c-1) {
			add_token::<T>(alice.clone(), bob.clone(), class_id, into!(40), Some(PerU16::zero()));
			tokens.push((class_id, into!(i+1), into!(10)));
		}

		let auction_id = current_gid::<T>();
	}: {
		UnetAuction::<T>::submit_dutch_auction(
			RawOrigin::Signed(bob.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			into!(ACCURACY), // deposit
			200, // min_price
			2000, // max_price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			true,
			PerU16::from_percent(50),
			PerU16::zero(),
		)?;
	}
	verify {
		assert_last_event::<T>(Event::<T>::CreatedDutchAuction(bob.clone(), auction_id).into());
	}

	submit_british_auction {
		let c in 1 .. MAX_TOKEN_PER_AUCTION;

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
			add_token::<T>(alice.clone(), alice.clone(), class_id, into!(20), Some(PerU16::zero()));
			tokens.push((class_id, into!(i+1), into!(10)));
		}

		let auction_id = current_gid::<T>();
	}: {
		UnetAuction::<T>::submit_british_auction(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			0, // hammer_price
			PerU16::from_percent(5), // min_raise
			into!(ACCURACY), // deposit
			into!(ACCURACY), // init_prince
			into!(200), // deadline
			false,
			tokens,
			PerU16::zero(),
		)?;
	}
	verify {
		assert_last_event::<T>(Event::<T>::CreatedBritishAuction(alice.clone(), auction_id).into());
	}

	bid_dutch_auction {
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

		UnetAuction::<T>::submit_dutch_auction(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			into!(ACCURACY), // deposit
			200, // min_price
			2000, // max_price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			true,
			PerU16::from_percent(50),
			PerU16::zero(),
		)?;
	}: {
		UnetAuction::<T>::bid_dutch_auction(
			RawOrigin::Signed(bob.clone()).into(),
			into!(ACCURACY),
			recipient_lookup,
			auction_id,
			None,
			None,
		)?;
	}

	bid_british_auction {
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

		UnetAuction::<T>::submit_british_auction(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			0, // hammer_price
			PerU16::from_percent(5), // min_raise
			into!(ACCURACY), // deposit
			into!(ACCURACY), // init_prince
			into!(200), // deadline
			false,
			tokens,
			PerU16::zero(),
		)?;
	}: {
		UnetAuction::<T>::bid_british_auction(
			RawOrigin::Signed(bob.clone()).into(),
			into!(ACCURACY),
			recipient_lookup,
			auction_id,
			None,
			None,
		)?;
	}

	redeem_dutch_auction {
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

		UnetAuction::<T>::submit_dutch_auction(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			into!(ACCURACY), // deposit
			200, // min_price
			2000, // max_price
			into!(10), // deadline
			tokens,
			true,
			PerU16::from_percent(50),
			PerU16::zero(),
		)?;

		UnetAuction::<T>::bid_dutch_auction(
			RawOrigin::Signed(bob.clone()).into(),
			into!(500),
			recipient_lookup.clone(),
			auction_id,
			None,
			None,
		)?;

		frame_system::Pallet::<T>::set_block_number(into!(500));
	}: {
		UnetAuction::<T>::redeem_dutch_auction(
			RawOrigin::Signed(alice.clone()).into(),
			recipient_lookup,
			auction_id,
		)?;
	}

	redeem_british_auction {
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

		UnetAuction::<T>::submit_british_auction(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			0, // hammer_price
			PerU16::from_percent(5), // min_raise
			into!(ACCURACY), // deposit
			into!(ACCURACY), // init_prince
			into!(10), // deadline
			false,
			tokens,
			PerU16::zero(),
		)?;

		UnetAuction::<T>::bid_british_auction(
			RawOrigin::Signed(bob.clone()).into(),
			into!(ACCURACY*2),
			recipient_lookup.clone(),
			auction_id,
			None,
			None,
		)?;

		frame_system::Pallet::<T>::set_block_number(into!(500));
	}: {
		UnetAuction::<T>::redeem_british_auction(
			RawOrigin::Signed(alice.clone()).into(),
			recipient_lookup,
			auction_id,
		)?;
	}

	remove_dutch_auction {
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

		UnetAuction::<T>::submit_dutch_auction(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			into!(ACCURACY), // deposit
			200, // min_price
			2000, // max_price
			into!((MINUTES as u64) * 120 + 1), // deadline
			tokens,
			true,
			PerU16::from_percent(50),
			PerU16::zero(),
		)?;
	}: {
		UnetAuction::<T>::remove_dutch_auction(
			RawOrigin::Signed(alice.clone()).into(),
			auction_id,
		)?;
	}

	remove_british_auction {
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

		UnetAuction::<T>::submit_british_auction(
			RawOrigin::Signed(alice.clone()).into(),
			into!(NATIVE_CURRENCY_ID),
			0, // hammer_price
			PerU16::from_percent(5), // min_raise
			into!(ACCURACY), // deposit
			into!(ACCURACY), // init_prince
			into!(200), // deadline
			false,
			tokens,
			PerU16::zero(),
		)?;
	}: {
		UnetAuction::<T>::remove_british_auction(
			RawOrigin::Signed(alice.clone()).into(),
			auction_id,
		)?;
	}
}

impl_benchmark_test_suite!(UnetAuction, crate::mock::new_test_ext(), crate::tests::Test,);
