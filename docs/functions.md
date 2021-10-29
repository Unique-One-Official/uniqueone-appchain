# Description of Unique One Network functionalities

Unique One Network Appchain runtime uses the following pallets to provide the necessary NFT related functions

## Pallet Ethereum & EVM
These modules enable the appchain with Ethereum Solidity smart contracts capability.

## Pallet Contract
This module provides functionality for the runtime to deploy and execute WebAssembly smart-contracts.

## Pallet UNet NFT
This UNet NFT module handles the functions of NFT class/collection creation, NFT token mint, NFT token burn and ownership transfer.
This mobule exposes the following extrinsic calls:

### Create NFT Class/Collection
```rust
pub fn create_class(origin: OriginFor<T>, metadata: NFTMetadata, name: Vec<u8>, description: Vec<u8>, royalty_rate: PerU16, properties: Properties, category_ids: Vec<GlobalId> ) -> DispatchResultWithPostInfo
```

### Destroy NFT Class/Collection
```rust
pub fn destroy_class(origin: OriginFor<T>, class_id: ClassIdOf<T>, dest: <T::Lookup as StaticLookup>::Source ) -> DispatchResultWithPostInfo
```

### Mint NFT Token
```rust
pub fn proxy_mint(origin: OriginFor<T>, to: <T::Lookup as StaticLookup>::Source, class_id: ClassIdOf<T>, metadata: NFTMetadata, quantity: TokenIdOf<T>, charge_royalty: Option<PerU16> ) -> DispatchResultWithPostInfo
```

### Transfer NFT Token to another account
```rust
pub fn transfer(origin: OriginFor<T>, to: <T::Lookup as StaticLookup>::Source, items: Vec<(ClassIdOf<T>, TokenIdOf<T>, TokenIdOf<T>)> ) -> DispatchResultWithPostInfo
```

### Burn NFT Token
```rust
pub fn burn(origin: OriginFor<T>, class_id: ClassIdOf<T>, token_id: TokenIdOf<T>, quantity: TokenIdOf<T> ) -> DispatchResultWithPostInfo
```

### Update NFT Token Royalty
```rust
pub fn update_token_royalty(origin: OriginFor<T>, class_id: ClassIdOf<T>, token_id: TokenIdOf<T>, charge_royalty: Option<PerU16> ) -> DispatchResultWithPostInfo
```

### Update NFT Token Royalty Beneficiary
```rust
pub fn update_token_royalty_beneficiary(origin: OriginFor<T>, class_id: ClassIdOf<T>, token_id: TokenIdOf<T>, to: <T::Lookup as StaticLookup>::Source) -> DispatchResultWithPostInfo
```
