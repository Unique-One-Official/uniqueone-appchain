//! Implementations for `nonfungibles` traits.

use super::*;
use frame_support::traits::tokens::nonfungibles::{Inspect, Transfer};
use sp_runtime::DispatchResult;

impl<T: Config> Inspect<<T as frame_system::Config>::AccountId> for Pallet<T> {
	type ItemId = T::TokenId;
	type CollectionId = T::ClassId;

	fn owner(
		class: &Self::CollectionId,
		instance: &Self::ItemId,
	) -> Option<<T as frame_system::Config>::AccountId> {
		unet_orml_nft::Pallet::<T>::tokens(class, instance).map(|a| a.owner)
	}

	fn class_owner(class: &Self::CollectionId) -> Option<<T as frame_system::Config>::AccountId> {
		unet_orml_nft::Pallet::<T>::classes(class).map(|a| a.owner)
	}

	/// Returns the attribute value of `instance` of `class` corresponding to `key`.
	///
	/// When `key` is empty, we return the instance metadata value.
	///
	/// By default this is `None`; no attributes are defined.
	fn attribute(
		class: &Self::CollectionId,
		instance: &Self::ItemId,
		key: &[u8],
	) -> Option<Vec<u8>> {
		if key.is_empty() {
			// We make the empty key map to the instance metadata value.
			unet_orml_nft::Pallet::<T>::tokens(class, instance).map(|a| a.metadata.into())
		} else {
			return None;
		}
	}

	/// Returns the attribute value of `instance` of `class` corresponding to `key`.
	///
	/// When `key` is empty, we return the instance metadata value.
	///
	/// By default this is `None`; no attributes are defined.
	fn class_attribute(class: &Self::CollectionId, key: &[u8]) -> Option<Vec<u8>> {
		if key.is_empty() {
			// We make the empty key map to the instance metadata value.
			unet_orml_nft::Pallet::<T>::classes(class).map(|a| a.metadata.into())
		} else {
			return None;
		}
	}

	/// Returns `true` if the asset `instance` of `class` may be transferred.
	///
	/// Default implementation is that all assets are transferable.
	fn can_transfer(class: &Self::CollectionId, instance: &Self::ItemId) -> bool {
		match unet_orml_nft::Pallet::<T>::classes(class) {
			Some(class) => class.data.properties.0.contains(ClassProperty::Transferable),
			_ => false,
		}
	}
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
	fn transfer(
		class: &Self::CollectionId,
		instance: &Self::ItemId,
		destination: &T::AccountId,
	) -> DispatchResult {
		let from = unet_orml_nft::Pallet::<T>::tokens(class, instance)
			.map(|a| a.owner)
			.ok_or(Error::<T>::TokenNotFound)?;
		Self::do_transfer(&from, &destination, (*class, *instance))?;
		Ok(())
	}
}