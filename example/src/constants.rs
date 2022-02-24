#![allow(dead_code)]
macro_rules! apis {
    ($($name:ident => $content:expr)*) => {
        $(
            pub const $name: &str = $content;
        )*
    }
}

apis! {
    API_PUBLIC_KEY_ENV_VAR => "pk_live_GHRzvJIIobM0Yvpec61tquOToX8jae3SnQEDwrGeCUbY0IzPD6"
    API_SECRET_KEY_ENV_VAR => "sk_live_GZrPRI0EhqDgjWvKbbJf94lYfN8lBacwNXY2h9iHvClPqXTEQT"
}