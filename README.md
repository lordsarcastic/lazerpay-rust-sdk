# 💳 Lazerpay Rust SDK

This crate integrates the Lazerpay payment gateway for accepting cryptocurrency payments.

[![Crates.io][crate-src]][crate-href] [![License][license-src]][license-href] [![Contributors][contributors-src]][contributors-href]

## Usage

Add the following to your `cargo.toml`

```toml
lazerpay-rust-sdk = "0.1.0"
```

Then you can easily import the crate.

```rust

use lazerpay_rust_sdk::Lazerpay;

```

Next you will need to create a Lazerpay instance with the help your `API_PUBLIC_KEY` and `API_SECRET_KEY` you can get that easily from your [Lazerpay dashboard](https://beta.lazerpay.finance/)

```rust
use serde_json::Value;
use lazerpay_rust_sdk::Lazerpay;

mod utils;

#[tokio::main]
async fn main() {
    let api = utils::get_api_keys().unwrap();
    let lazerpay: Lazerpay = Lazerpay::new(&api.public_key, &api.secret_key);

    let response = initialize_payment(
        lazerpay,
        "xxxxxxxxxxxxxxxxxxxxx".to_string(),
        "1000".to_string(),
        "xxxxx".to_string(),
        "xxxxx@gmail.com".to_string(),
        "USDC".to_string(),
        "USD".to_string(),
        api.public_key.to_string(),
        true
    ).await;

    println!("Response -> {:?}", response);
}

async fn initialize_payment (
    lazerpay: Lazerpay,
    reference: String,
    amount: String,
    customer_name: String,
    customer_email: String,
    coin: String,
    currency: String,
    api_public_key: String,
    accept_partial_payment: bool
    ) -> Value {
    lazerpay.payment.initialize_payment(
        reference,
        amount,
        customer_name,
        customer_email,
        coin,
        currency,
        api_public_key,
        accept_partial_payment
    ).await
}

async fn confirm_payment (
    lazerpay: Lazerpay,
    identifier: String,
    ) -> Value {
    lazerpay.payment.confirm_payment("xxxxxxxxxxxxxxxxxxxxx".to_string()).await
}

async fn get_accepted_coins (
    lazerpay: Lazerpay,
    ) -> Value {
    lazerpay.payment.get_accepted_coins().await
}

async fn get_rate (
    lazerpay: Lazerpay,
    currency: String,
    coin: String,
    ) -> Value {
    lazerpay.payment.get_rate(
        currency,
        coin,
    ).await
}

async fn transfer_funds (
    lazerpay: Lazerpay,
    amount: u32,
    recipient: String,
    coin: String,
    blockchain: String,
    api_public_key: String,
    api_secret_key: String,
    ) -> Value {
    lazerpay.payment.transfer_funds(
        amount,
        recipient,
        coin,
        api_public_key,
        api_secret_key,
    ).await
}
```

and that's it.

### Examples

To try out the examples, you can use the following command:

```sh
cargo run --example payment
```

If you need more reference on this crate feel free to check the [source code](https://github.com/Lord-sarcastic/lazerpay-rust-sdk/tree/master/src)

[crate-src]: https://img.shields.io/crates/v/lazerpay-rust-sdk
[crate-href]: https://crates.io/crates/lazerpay-rust-sdk
[license-src]: https://img.shields.io/badge/License-MIT-blue.svg
[license-href]: https://github.com/Lord-sarcastic/lazerpay-rust-sdk/blob/master/LICENSE
[contributors-src]: https://img.shields.io/github/contributors/Lord-sarcastic/lazerpay-rust-sdk
[contributors-href]: https://github.com/TribecaHQ/tribeca/graphs/contributors
