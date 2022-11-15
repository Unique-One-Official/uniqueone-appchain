#[cfg(any(test, feature = "runtime-benchmarks"))]
pub mod test_helper {
	use crate::*;

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
}
