//! Implementations for `nonfungibles` traits.

use super::*;
use frame_support::traits::tokens::nonfungibles::{Inspect, Transfer};
use sp_runtime::{DispatchError, DispatchResult};

impl<T: Config> Inspect<<T as frame_system::Config>::AccountId> for Pallet<T> {
	type ItemId = T::TokenId;
	type CollectionId = T::ClassId;

	/// Returns the owner of `item` of `collection`, or `None` if the item doesn't exist
	/// (or somehow has no owner).
	fn owner(
		class: &Self::CollectionId,
		instance: &Self::ItemId,
	) -> Option<<T as frame_system::Config>::AccountId> {
		unet_orml_nft::OwnersByToken::<T>::iter_key_prefix((class, instance))
			.next()
			.map(|a| a.into())
	}

	fn collection_owner(
		class: &Self::CollectionId,
	) -> Option<<T as frame_system::Config>::AccountId> {
		unet_orml_nft::Classes::<T>::get(class).map(|a| a.owner)
	}

	/// Returns the attribute value of `instance` of `class` corresponding to `key`.
	///
	/// When `key` is empty, we return the instance metadata value.
	///
	/// By default this is `None`; no attributes are defined.
	fn attribute(
		_collection: &Self::CollectionId,
		_item: &Self::ItemId,
		_key: &[u8],
	) -> Option<Vec<u8>> {
		if _key.is_empty() {
			// We make the empty key map to the instance metadata value.
			unet_orml_nft::Pallet::<T>::tokens(_collection, _item).map(|a| a.metadata.into())
		} else {
			return None;
		}
	}

	/// Returns the strongly-typed attribute value of `item` of `collection` corresponding to
	/// `key`.
	///
	/// By default this just attempts to use `attribute`.
	fn typed_attribute<K: Encode, V: Decode>(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
	) -> Option<V> {
		key.using_encoded(|d| Self::attribute(collection, item, d))
			.and_then(|v| V::decode(&mut &v[..]).ok())
	}

	/// Returns the attribute value of `collection` corresponding to `key`.
	///
	/// By default this is `None`; no attributes are defined.
	fn collection_attribute(_collection: &Self::CollectionId, _key: &[u8]) -> Option<Vec<u8>> {
		if _key.is_empty() {
			unet_orml_nft::Pallet::<T>::classes(_collection).map(|a| a.metadata.into())
		} else {
			return None;
		}
	}

	/// Returns the strongly-typed attribute value of `collection` corresponding to `key`.
	///
	/// By default this just attempts to use `collection_attribute`.
	fn typed_collection_attribute<K: Encode, V: Decode>(
		collection: &Self::CollectionId,
		key: &K,
	) -> Option<V> {
		key.using_encoded(|d| Self::collection_attribute(collection, d))
			.and_then(|v| V::decode(&mut &v[..]).ok())
	}

	/// Returns `true` if the asset `instance` of `class` may be transferred.
	///
	/// Default implementation is that all assets are transferable.
	fn can_transfer(_collection: &Self::CollectionId, _item: &Self::ItemId) -> bool {
		match unet_orml_nft::Pallet::<T>::classes(_collection) {
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
		let from = unet_orml_nft::OwnersByToken::<T>::iter_key_prefix((class, instance))
			.next()
			.ok_or(Error::<T>::TokenNotFound)?;
		Self::do_transfer(&from, &destination, *class, *instance, 1.into())?;
		Ok(())
	}
}
