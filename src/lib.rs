extern crate hex;
extern crate base64;

pub fn hex_to_base64(hex :String) -> Result<String, hex::FromHexError> {
    // convert hex string to bytes
    let byte_array = hex::decode(hex)?;

    Ok(base64::encode(&byte_array))
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

}