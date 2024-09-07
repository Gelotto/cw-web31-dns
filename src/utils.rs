use bech32::decode;

pub fn is_bech32_address(address: &String) -> bool {
    decode(address).is_ok()
}
