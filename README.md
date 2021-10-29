<div align="center">
<img src="https://avatars.githubusercontent.com/u/73292446?s=200&v=4">
</div>

<div align="Center">
    <h1>Unique One Network</h1>
    <h2>The Next Generation NFT decentralised application chain</h2>

    Unique One Network is a Substrate-based, next-generation decentralised NFT application-specific blockchain leveraging Web3.0 interoperability to augment the NFT Evolution.

    <br>
    <br>

[![Substrate version](https://img.shields.io/badge/Substrate-3.0.0-brightgreen?logo=Parity%20Substrate)](https://substrate.dev/)
[![Medium](https://img.shields.io/badge/Medium-UniqueOneNetwork-brightgreen?logo=medium)](https://medium.com/@uniqueone)

</div>

---

Unique One Network is a Substrate-based, next-generation decentralised NFT application-specific blockchain leveraging Web3.0 interoperability to augment the NFT Evolution.

With a history of successfully building out multiple fully operational NFT marketplaces across diverse sectors and chains such as Eth, xDai, BSC, and Polygon, Unique One Network is now developing its mothership chain on Substrate as an Appchain in the Octopus Network.

The Unique One Network NFT Galaxy Appchain will extend Unique One modular builds to B2B marketplaces by offering a variety of NFT-as-service modules to enfranchise Web3.0 teams with the support, community, and NFT network effects they need to scale.

- [Explanation about Unique One Network Node functionalities](./docs/functions.md)

## Getting Started

Follow these steps to get started with the Node

### Build

```bash
cargo build
```

### Run

```bash
./target/debug/uniqueone-appchain \
--base-path .local \
--dev \
--alice \
--enable-offchain-indexing true
```