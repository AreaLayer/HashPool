# Hash Pool ⚡🔑

Efficient software for Miners and Pools

**Contributions are welcome**

⚠️ **Beta Software and WIP**

⚠️**Use small amount**

## About

Bitcoin remains a frontrunner, and its underlying technology continues to fascinate enthusiasts and experts alike. One key aspect that drives the operation of the Bitcoin network is the concept of mining, a process crucial to its security and functionality.

At the heart of mining lies the concept of "hash." A hash is a cryptographic function that converts an input of any size into a fixed-size string of characters. In Bitcoin mining, miners compete to solve complex mathematical puzzles using their computational power to find a specific hash that meets certain criteria. This process is known as "proof of work" and is essential for validating transactions and adding them to the blockchain.

Mining, however, is no longer a solo endeavor for most participants. This is where Software Development Kits (SDKs) as Hash Pool come into play. SDKs are sets of tools and libraries that developers can use to create applications for a specific platform. In the context of Bitcoin mining, SDKs allow developers to integrate mining capabilities into their applications or systems without having to reinvent the wheel.

Incorporating an SDK into a mining operation empowers developers to streamline the process, making it more efficient and accessible. This integration can range from creating user-friendly mining apps to implementing advanced mining strategies.

In conclusion, the fusion of SDKs as Hash Pools has brought significant advancements to the world of Bitcoin mining. These technologies not only simplify the mining process but also enhance its overall efficiency and accessibility.

## Archecture

- [`data`](https://github.com/AreaLayer/HashPool/tree/main/src/data) - section about Data on Bitcoin Network
- [`mining`](https://github.com/AreaLayer/HashPool/tree/main/src/mining) - section about Mining involving Pool, PoW and SHA256
- [`onchain`](https://github.com/AreaLayer/HashPool/tree/main/src/on-chain) - section about On-chain involving API, Block and Hash

## Run library

**Pre requesite**

- Cargo
- Rust Bitcoin
- BDK / LDK

To build your SDK application, navigate to the project directory in the terminal and run:

```cargo
cargo build
```

### Add this Cargo.toml in your application

```Cargo.toml
[pcakage]

name = "Hash Pool"
version = "1.0.0"

[package BDK]
bdk = "1.0.0-alha.9"

[package rust bitcoin]
rust-bitcoin = "0.13.0"
```

## Roadmap

- [x] Add new features (WIP)
- [x] Clean docs
- [x] Stratum V2 (WIP)
- [x] BraidPool (WIP)
- [ ] BOLT12
- [ ] Hardware support
- [ ] Testnet4
- [ ] Mutinynet
- [ ] Lightning support
- [ ] Add Cargo.lock
- [x] RBF
