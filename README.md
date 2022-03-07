# 💳 Lazerpay Rust SDK

This crate integrates the Lazerpay payment gateway for accepting cryptocurrency payments.

[![Crates.io][crate-src]][crate-href] [![License][license-src]][license-href] [![Contributors][contributors-src]][contributors-href]

## Usage

Add the following to your cargo dependencies:

```toml
lazerpay-rust-sdk = "0.1.2"
```

Next you get your test `LAZERPAY_PUBLIC_KEY` and `LAZERPAY_SECRET_KEY` from [Lazerpay dashboard](https://beta.lazerpay.finance/)

Add those to a `.env` file at the root of your project. You can check `.env.example` for an example.

With that out of the way, you can use the API to make payments.

```rust
use lazerpay_rust_sdk::{utils, Customer, Lazerpay, PaymentConfig};

#[tokio::main]
async fn main() {
    let api = Lazerpay::new();

    let mut payment = api.create_payment(PaymentConfig {
        customer: Customer::new("Enoch", "enochchejieh@gmail.com"),
        reference: utils::generate_unique_reference(),
        coin: "USDC".into(),
        amount: "1000".into(),
        currency: "USD".into(),
        accept_partial_payment: true,
    });

    dbg!(payment.send().await.unwrap());
    dbg!(payment.check_confirmation().await.unwrap());
}
```

And transfer funds using the transfer API.

```rust
use lazerpay_rust_sdk::{utils, Customer, Lazerpay, TransferConfig};

#[tokio::main]
async fn main() {
    let api = Lazerpay::new();

    let mut transfer = api.create_transfer(TransferConfig {
        recipient: "0x0000000000000000000000000000000000000000".into(),
        blockchain: "Binance Smart Chain".into(),
        coin: "USDC".into(),
        amount: 1000,
    });

    dbg!(transfer.send().await.unwrap());
}
```

Finally, you can use get information on supported coins and their rates.

```rust
use lazerpay_rust_sdk::Lazerpay;

#[tokio::main]
async fn main() {
    let api = Lazerpay::new();

    dbg!(api.get_rates("USD", "USDC").await.unwrap());
    dbg!(api.get_accepted_coins().await.unwrap());
}
```

and that's it.

### Examples

You could also try out the examples in the project. For example, to run the payment example, run:

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
