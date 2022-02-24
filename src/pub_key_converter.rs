use crate::base_types::ComprPubKey

pub fn string_from_compr_pub_key(pub_key : ComprPubKey) -> String {

    let mut result = String::new();
    for el in pub_key {
        let el1 : u8 = (el << 4) >> 4;
        let el2 : u8 = el >> 4;
        result.push((el1 + b"a".get(0).unwrap()) as char);
        result.push((el2 + b"a".get(0).unwrap()) as char);
    }
    result

}
pub fn comp_pub_key_from_string(string_pub_key : String) -> ComprPubKey {

    let mut result : [u8;32] = [0;32];
    for i in 0..32 {
        let el1 = string_pub_key.as_bytes()[2*i] - b"a".get(0).unwrap();
        let el2 = string_pub_key.as_bytes()[2*i+1] - b"a".get(0).unwrap();
        result[i] = el1 + (el2 << 4);
    }

    result as ComprPubKey
}