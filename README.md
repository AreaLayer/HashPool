# Hash Pool ⚡🔑

[![Bitcoin-only](https://img.shields.io/badge/bitcoin-only-FF9900?logo=bitcoin)](https://twentyone.world)
[![LN](https://img.shields.io/badge/lightning-792EE5?logo=lightning)](https://mempool.space/lightning)
[![Rust](https://github.com/AreaLayer/HashPool/actions/workflows/rust.yml/badge.svg)](https://github.com/AreaLayer/HashPool/actions/workflows/rust.yml)
![Crates.io Total Downloads](https://img.shields.io/crates/d/Hash_Pool)
[![crates.io](https://img.shields.io/crates/v/Hash_Pool)](https://crates.io/crates/Hash_Pool)
[![Documentation](https://img.shields.io/static/v1?logo=read-the-docs&label=docs.rs&message=Hash_Pool&color=informational)](https://docs.rs/Hash_Pool/latest/mining/)


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
- [`lightning`](https://github.com/AreaLayer/HashPool/tree/main/src/lightning) - section about Lightning Network with BOLT11 and BOLT12
- [`stratumv2`](https://github.com/AreaLayer/HashPool/tree/main/src/stratumv2) - section about Stratum V2 protocol
- [`braidpool`](https://github.com/AreaLayer/HashPool/tree/main/src/braidpool) - section about BraidPool protocol
- [ `asic` ](https://github.com/AreaLayer/HashPool/tree/main/src/asic) - section about ASIC mining and hardware

## Run library

**Pre requesite**

- Rust version 1.60 or higher

To build your SDK application, navigate to the project directory in the terminal and run:

```cargo
cargo build
```

### Add this Cargo.toml in your application

```Cargo.toml
[pcakage]
name = "Hash Pool"
version = "1.0.16-beta"
```

## Roadmap

- [ ] Bitaxe (basic)
- [ ] Datum (basic template)