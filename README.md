# Nibiru contracts

This project contains the contracts for the [email-wallet](https://github.com/hduoc2003/email-wallet).

## Installation

Before running the following steps, you need to install `rustup`, `cargo`, `python` and `docker`.

### Install Nibiru CLI

```bash
rustup target add wasm32-unknown-unknown
curl -s https://get.nibiru.fi/! | bash
```

### Setup Nibiru Testnet

```bash
nibid config chain-id nibiru-testnet-1 && \                                         
nibid config broadcast-mode sync && \
nibid config node "https://rpc.testnet-1.nibiru.fi:443" && \
nibid config keyring-backend test && \
nibid config output json
```

### Add Nibid wallet

```bash
nibid keys import-hex $WALLET_NAME $PRIVATE_KEY
```

### Compile

> [!NOTE]
> You can use the existing configs we have built, or run this step to redeploy the contracts

First, we need to optimize our generated wasm binary file using CosmWasm Rust Optimizer by running:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.0
```

Then, run the script:

```bash
cd script
python store_contracts.py
```

All the data generated during deployment will be stored in the build folder.
