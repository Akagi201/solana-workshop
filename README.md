# solana-workshop

Codes and notes while learning solana

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
