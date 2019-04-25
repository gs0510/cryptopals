extern crate base64;
extern crate hex;

pub fn hex_to_base64(hex: String) -> Result<String, hex::FromHexError> {
    // convert hex string to bytes
    let byte_array = hex::decode(hex)?;

    Ok(base64::encode(&byte_array))
}

pub fn hex_xor(hex1: String, hex2: String) -> Result<String, hex::FromHexError> {
    let byte_array1 = hex::decode(hex1)?;
    let byte_array2 = hex::decode(hex2)?;

    let mut byte_array = byte_array1
        .iter()
        .zip(byte_array2.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<_>>();

    Ok(hex::encode(byte_array))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(
            hex_to_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")),
            Ok(String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"))
        );
    }

     #[test]
    fn test_hex_xor() {
        assert_eq!(
            hex_xor(
                String::from("1c0111001f010100061a024b53535009181c"),
                String::from("686974207468652062756c6c277320657965")
            ),
            Ok(String::from("746865206b696420646f6e277420706c6179"))
        );
    }

}
