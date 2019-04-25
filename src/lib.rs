extern crate base64;
extern crate hex;
use std::str;

pub fn hex_to_base64(hex: String) -> Result<String, hex::FromHexError> {
    // convert hex string to bytes
    let byte_array = hex::decode(hex)?;

    Ok(base64::encode(&byte_array))
}

pub fn hex_xor(hex1: String, hex2: String) -> Result<String, hex::FromHexError> {
    let byte_array1 = hex::decode(hex1)?;
    let byte_array2 = hex::decode(hex2)?;

    let byte_array = byte_array1
        .iter()
        .zip(byte_array2.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<_>>();

    Ok(hex::encode(byte_array))
}

pub fn xor_repeats(byte_array1: &Vec<u8>, byte2: &u8) -> Vec<u8> {
    byte_array1
        .iter()
        .zip(vec![byte2].iter().cycle())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<_>>()
}

pub fn xor_cipher(hex1: String, byte2: &u8) -> Result<String, hex::FromHexError> {
    let byte_array1 = hex::decode(hex1)?;

    let result = xor_repeats(&byte_array1, byte2);

    let s = match str::from_utf8(&result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    Ok(s.to_string())
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

    #[test]
    fn test_xor_cipher() {
        let s = match xor_cipher(
            String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"),
            &0b0000_0101,
        ) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("result: {}", s);
    }

}
