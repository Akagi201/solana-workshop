# solana-workshop

Codes and notes while learning solana

## Build

```sh
# https://github.com/solana-labs/solana/issues/34609
cargo update -p ahash@0.8.7 --precise 0.8.6
anchor build
anchor deploy --provider.wallet ~/.config/solana/id.json --provider.cluster devnet
```

## Nice Books

* <https://solanacookbook.com/#contributing>
* <https://www.solanazh.com/>

## Community

* <https://soldev.cn/>
