# solana-workshop

Codes and notes while learning solana

## Docs

* <https://docs.anza.xyz/cli/install>
* <https://www.anchor-lang.com/docs>
* <https://www.solanazh.com/>
* <https://solana.com/developers/cookbook>
* <https://github.com/helius-labs/solana-awesome>

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
avm install latest
avm use latest
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
