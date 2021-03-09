use secp256k1_sys::{secp256k1_context_no_precomp, secp256k1_ec_seckey_tweak_add};

#[no_mangle]
pub extern "C" fn private_add(seckey: *mut u8, tweak: *const u8) -> i32 {
    unsafe { secp256k1_ec_seckey_tweak_add(secp256k1_context_no_precomp, seckey, tweak) }
}
