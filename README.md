<p align="center">
  <img src="https://avatars0.githubusercontent.com/u/41934032?s=400&v=4" alt="Crypto.com Chain" width="400">
</p>

# Crypto.com Chain
[![Build Status](https://travis-ci.org/crypto-com/chain.svg?branch=master)](https://travis-ci.org/crypto-com/chain)
[![codecov](https://codecov.io/gh/crypto-com/chain/branch/master/graph/badge.svg)](https://codecov.io/gh/crypto-com/chain)
[![riot.im](https://img.shields.io/badge/riot.im-JOIN%20CHAT-green.svg)](https://riot.im/app/#/room/#cro:matrix.org)

This repository contains the pre-alpha version prototype implementation of Crypto.com Chain. The current repository consists of the following Rust sub-crates:

* *chain-abci*: the Tendermint ABCI application that currently does the transaction validation etc.

* *chain-core*: the library that contains the main type definitions and any utility code (such as serialization), so that it can be used in *chain-abci* and other applications.

* *dev-utils*: currently a minimal development tool for generating genesis.json

* *signer-[cli|core|rpc]*: work-in-progress infrastructure for client code, such as wallet implementations and third party integrations. 

## Contributing
Please abide by the [Code of Conduct](CODE_OF_CONDUCT.md) in all interactions,
and the [contributing guidelines](CONTRIBUTING.md) when submitting code.

## License

[Apache 2.0](./LICENSE)

## Prerequisites 

Tendermint: https://github.com/tendermint/tendermint/releases

Rust tooling (cargo + cmake): https://rustup.rs

(TODO: In the future, the build tooling may be migrated to Bazel / Nix etc. for reproducible builds.)

## How to build it
```
cargo build
```

## How to run the test suite
```
cargo test
```

To measure code coverage, one can run
```
cargo tarpaulin
```

This only works on x86_64 processors running Linux. On different platforms, 

```
docker run --security-opt seccomp=unconfined -v "$PWD:/volume" xd009642/tarpaulin
```

## How to run a full node
1. generate initial state -- currently, a very naive way is in dev-utils. In the end, you should get two pieces of data:
```
"app_hash": "<APP_HASH_HEX_BYTES>",
"app_state": {"distribution":[{"address":"0x<ETH_ADDRESS_HEX_BYTES>","amount":10000000000000000000 }]}
```

"app_hash" is the initial application hash -- currently, it's computed as a root of a merkle tree of transaction IDs. In the example above, there'll be only one TX:
```
(no inputs) => 1 output
```
where the output is saying "0x<ETH_ADDRESS_HEX_BYTES>" could redeem the full supply.

"app_state" is the initial application state that will be passed to the application. Currently, it only contains the initial distribution that maps Ethereum-style addresses to the number of tokens they own.

2. initialize a Tendermint node:
```
tendermint init
```

If you previously initialized a Tendermint node, you may need to run before it:
```
tendermint unsafe_reset_all
```

3. configure a Tendermint node:

One thing to change would be the change `genesis.json` to have `app_hash` and `app_state` obtained in step 1. Also, make sure the `chain_id` ends with two hex digits (e.g. test-chain-mafL4t-AA).

4. run CRO ABCI process, e.g.:
```
chain-node -g "<APP_HASH_HEX_BYTES>" -c <CHAIN_ID>
```

The string passed to `-g` is the genesis `app_hash` obtained in step 1 and configured in the previous step. The string passed to `-c` is the `chain_id` that ends with two hex digits (e.g. test-chain-mafL4t-AA).

If you need backtraces or logging, set the environment variables before it:
```
RUST_BACKTRACE=1 RUST_LOG=info 
```

5. run a Tendermint process:
in a different terminal:

```
tendermint node
```

## How to run a basic lite node
```
tendermint lite
```

## How to send TX, query, etc.

See Tendermint RPC documentation: https://tendermint.com/rpc/#introduction

Currently, there's a rough command-line application in `signer/app` for testing purposes.

Its workflow is the following:

1. Generate a new keypair: `signer address generate -n <name>` (you can view hex-encoded addresses of corresponding key with `signer address get -n <name>`)

2. Generate a signed TX with: 
```
signer transaction generate -n <name> -c <chain_id>
```

After entering all the required data, this will print out a hex-encoded TXID (blake2s hash) and a hex-encoded RLP-serialized TX structure / content. You can decode it and transform it as necessary, e.g. into base64.

3. You can then send a TX using the `broadcast_tx` Tendermint RPC command (e.g. JSONRPC over HTTP or websockets). 
For example, for the [URI/HTTP option](https://tendermint.com/rpc/#uri-http), it can look like this:

```
http://localhost:26657/broadcast_tx_sync?tx=0xa200a30081a200982018f318bc1844182918821848188a18a51835186118f51877189c031868186618f8188a185b18cb185d18b418da18a518db18d6183f18e2185018f200184701000181a20082009418431835186b18fe18ec011858183d18fe1878187e183d181f18a3181918ae06189c181e189801185002a10018aa0181820183820298201861189718f318a518bb184918ea00187205187318ad18b6184318cd182a1841183518dc1825181a0b18ac181d184418f718a118c418680018f701829820188718d90c186d183518ee189218410f186c189118d418c418d518c5184e189a189418f7185d181a1820182b18b1181e18e618b51018e3184c1851186d982018220d18b71859183718af187018b818c8185f18731893188618e0181918e918fc18b5185718c3188c182b188b185e1882184618d7189d187218f4183d181980
```

## Useful links

* [Project Website](http://crypto.com/chain)
* Community chatrooms (non-technical): [Discord](https://discord.gg/nsp9JTC) [Telegram](https://t.me/CryptoComOfficial)
* Developer community chatroom (technical): [![riot.im](https://img.shields.io/badge/riot.im-JOIN%20CHAT-green.svg)](https://riot.im/app/#/room/#cro:matrix.org)
