# solana-workshop

Codes and notes while learning solana

## Docs

* <https://docs.anza.xyz/cli/install>
* <https://www.anchor-lang.com/docs>
* <https://www.solanazh.com/>
* <https://solana.com/developers/cookbook>
* <https://github.com/helius-labs/solana-awesome>

## Solana Ecosystem

* <https://solanacompass.com/>

## Node.js Libraries

* <https://github.com/kevinheavey/solana-bankrun> - testing
* <https://github.com/kevinheavey/anchor-bankrun> - testing
* <https://github.com/metaDAOproject/spl-token-bankrun> - testing

## Install Solana CLI

```sh
sh -c "$(curl -sSfL https://release.anza.xyz/beta/install)"
# installed bins: ~/.local/share/solana/install/active_release/bin/
agave-install update # update solana cli
```

## Solana config file

```sh
solana config get
```

```sh
~/.config/solana/cli/config.yml
```

## Solana switch network

```sh
solana config set -um # mainnet-beta
solana config set -ud # devnet, use this for dev
solana config set -ul # localnet
solana config set -ut # testnet
solana config set --url https://api.testnet.solana.com
```

## Solana RPCs

* <https://solana.com/rpc>

## Install Anchor CLI

```sh
# install avm
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install master # latest means latest stable release
avm use master
```

## Solana playground

* <https://beta.solpg.io/>

## Build

```sh
# https://github.com/solana-labs/solana/issues/34609
cargo update -p ahash@0.8.7 --precise 0.8.6
anchor build
anchor deploy --provider.wallet ~/.config/solana/id.json --provider.cluster devnet
```

## Anchor commands

```sh
anchor keys sync # regenerate declare_id with target/deploy keypair
```

## RPC

* <https://github.com/rpcpool/yellowstone-grpc> - gRPC for Solana

## Nice Books

* <https://solanacookbook.com/#contributing>
* <https://www.solanazh.com/>

## Community

* <https://soldev.cn/>

## Testing

* <https://github.com/Ackee-Blockchain/trident>

## Tutorials

* <https://www.bilibili.com/video/BV1JkN8esEW8/>
* <https://www.youtube.com/watch?v=TQn9_grKFlQ>
* <https://turbin3.com/>
* <https://github.com/3uild-3thos/advanced-rust>

## Solana static analyzer

* <https://github.com/VulnPlanet/l3x>
* <https://github.com/auditware/radar>
* <https://github.com/sec3-product/x-ray>
* <https://github.com/honey-guard/solana-fender>

## Solana examples

* <https://github.com/solana-turbin3>
