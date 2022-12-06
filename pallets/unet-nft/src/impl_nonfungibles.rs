//! Implementations for `nonfungibles` traits.

use super::*;
use frame_support::traits::tokens::{
	nonfungibles::{Inspect, Transfer},
};
use sp_runtime::DispatchResult;

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
			.ok_or(Error::<T>::TokenIdNotFound)?;
		Self::do_transfer(&from, &destination, *class, *instance, 1u64.into())?;
		Ok(())
	}
}

// For the definition of base metadata, please refer to the following document:
// 		https://github.com/unique_one-team/unique_one-spec/blob/master/standards/unique_one2.0.0/entities/metadata.md#schema-definition
#[derive(Deserialize, RuntimeDebug)]
struct UniqueOneBaseMetadata {
	// NFT name, required
	name: String,
	// General notes, abstracts, or summaries about the contents of an NFT.
	#[serde(default)]
	description: String,

	#[serde(default)]
	image: String,

	#[serde(default)]
	#[serde(rename = "externalUrl")]
	external_url: String,

	#[serde(default)]
	#[serde(rename = "alternativeImage")]
	alternative_image: String,

	#[serde(default)]
	#[serde(rename = "imageMimeType")]
	image_mime_type: String,

	#[serde(default)]
	#[serde(rename = "animationUrl")]
	animation_url: String,

	#[serde(default)]
	#[serde(rename = "alternativeAnimationUrl")]
	alternative_animation_url: String,

	#[serde(default)]
	#[serde(rename = "animationMimeType")]
	animation_mime_type: String,

	#[serde(default)]
	attributes: Vec<Attribute>
}

#[derive(Deserialize, RuntimeDebug)]
struct  Attribute {
	#[serde(default)]
	#[serde(rename = "traitType")]
	trait_type: String,

	#[serde(default)]
	value: String,
}

pub struct UniqueOneBaseMetadataConvertor<T>(sp_std::marker::PhantomData<T>);
impl<T> ConvertIntoNep171 for UniqueOneBaseMetadataConvertor<T>
where
	T: Config,
{
	type CollectionId = T::ClassId;
	type ItemId = T::TokenId;

	fn convert_into_nep171_metadata(
		collection: Self::CollectionId,
		item: Self::ItemId,
	) -> Option<Nep171TokenMetadata> {
		let mut data: Vec<u8> = Vec::new();
		if let Some(attribute) = <Pallet<T> as Inspect<T::AccountId>>::attribute(
			&collection,
			&item,
			&vec![],
		) {
			data.extend(attribute);
		}

		if data.is_empty() {
			return None;
		}

		// parse vec to unique_one base metadata
		let unique_one_metadata: UniqueOneBaseMetadata = match serde_json::from_slice(&data) {
			Ok(metadata) => metadata,
			Err(_) => {
				log!(warn, "data : {:?}", data);
				log!(warn, "Failed to parse data to unique_one base metadata");
				return None;
			},
		};
		log!(debug, "unique_one metadata is : {:?}", unique_one_metadata);

		// Need Check:
		// 		Can the name field be empty?
		let title = (unique_one_metadata.name.len() != 0).then_some(unique_one_metadata.name);
		let description =
			(unique_one_metadata.description.len() != 0).then_some(unique_one_metadata.description);
		let image = (unique_one_metadata.image.len() != 0).then_some(unique_one_metadata.image);

		let extra = json!({
			"externalUrl": unique_one_metadata.external_url,
			"alternativeImage": unique_one_metadata.alternative_image,
			"imageMimeType": unique_one_metadata.image_mime_type,
			"animationUrl": unique_one_metadata.animation_url,
			"alternativeAnimationUrl": unique_one_metadata.alternative_animation_url,
			"animationMimeType": unique_one_metadata.animation_mime_type,
		});

		// parse unique_one base metadata to nep171 format
		let metadata = Nep171TokenMetadata {
			title,
			description,
			media: image,
			media_hash: None,
			copies: None,
			issued_at: None,
			expires_at: None,
			starts_at: None,
			updated_at: None,
			extra: Some(extra.to_string()),
			reference: None,
			reference_hash: None,
		};
		log!(debug, "After, the Nep171 media data is {:?} ", metadata.clone());

		Some(metadata)
	}
}
