#![cfg_attr(not(feature = "std"), no_std)]

polkadot_sdk::sp_api::decl_runtime_apis! {
    pub trait PalletMinimalTemplateApi {
        fn get_value() -> u32;
    }
}
