#![allow(dead_code)]
macro_rules! apis {
    ($($name:ident => $content:expr)*) => {
        $(
            pub const $name: &str = $content;
        )*
    }
}

apis! {
    BASE_URL => "https://api.lazerpay.engineering/api/v1"
    API_URL_INIT_TRANSACTION => "https://api.lazerpay.engineering/api/v1/transaction/initialize"
    API_URL_CONFIRM_TRANSACTION => "https://api.lazerpay.engineering/api/v1/transaction/verify"
    API_URL_GET_ACCEPTED_COINS => "https://api.lazerpay.engineering/api/v1/coins"
    API_URL_TRANSFER_FUNDS => "https://api.lazerpay.engineering/api/v1/transfer"
}