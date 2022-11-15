#[cfg(any(test, feature = "runtime-benchmarks"))]
pub mod test_helper {
	use crate::*;
	use frame_support::assert_ok;
	use sp_std::{vec, vec::Vec};

	#[macro_export]
	macro_rules! balances {
		($amount: expr) => {
			unet_traits::constants_types::ACCURACY.saturating_mul($amount).saturated_into()
		};
	}

	#[macro_export]
	macro_rules! into {
		($amount: expr) => {
			($amount as u128).saturated_into()
		};
	}

	pub fn add_whitelist<Runtime>(who: &Runtime::AccountId)
	where
		Runtime: crate::Config,
	{
		Runtime::ExtraConfig::do_add_whitelist(who);
	}

	pub fn add_class<Runtime>(who: Runtime::AccountId)
	where
		Runtime: crate::Config,
	{
		let cate_id1 = current_gid::<Runtime>();
		add_category::<Runtime>();
		let cate_id2 = current_gid::<Runtime>();
		add_category::<Runtime>();
		assert_ok!(Runtime::NFT::create_class(
			&who,
			Vec::from("1"),
			Vec::from("1"),
			Vec::from("1"),
			PerU16::from_percent(5),
			Properties(ClassProperty::Transferable | ClassProperty::Burnable),
			vec![cate_id1, cate_id2],
		));
	}

	pub fn last_event<Runtime>() -> Runtime::RuntimeEvent
	where
		Runtime: frame_system::Config,
	{
		frame_system::Pallet::<Runtime>::events().pop().expect("RuntimeEvent expected").event
	}

	pub fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
		frame_system::Pallet::<T>::assert_last_event(generic_event.into());
	}

	pub fn add_token<Runtime>(
		who: Runtime::AccountId,
		to: Runtime::AccountId,
		class_id: Runtime::ClassId,
		quantity: Runtime::TokenId,
		charge_royalty: Option<PerU16>,
	) where
		Runtime: crate::Config,
	{
		assert_ok!(Runtime::NFT::proxy_mint(
			&who,
			&to,
			class_id,
			Vec::from("1"),
			quantity,
			charge_royalty
		));
	}

	pub fn add_category<Runtime: Config>() {
		assert_ok!(Runtime::ExtraConfig::do_create_category(Vec::from("1")));
	}

	pub fn current_gid<Runtime: Config>() -> GlobalId {
		Runtime::ExtraConfig::peek_next_gid()
	}

	pub fn peek_next_class_id<Runtime: Config>() -> Runtime::ClassId {
		Runtime::NFT::peek_next_class_id()
	}
}
