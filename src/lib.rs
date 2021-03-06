extern crate base64;
extern crate hex;

use std::collections::HashMap;


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

pub fn xor_repeats(byte_array1: &[u8], byte2: &u8) -> Vec<u8> {
    byte_array1
        .iter()
        .zip(vec![byte2].iter().cycle())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<_>>()
}


pub fn get_score(xor: &Vec<u8>) -> u32 {
	let frequency: HashMap<u8, u32> = [
		(b'a', 8),
		(b'e', 12),
		(b'h', 6),
		(b'i', 7),
		(b'n', 7),
		(b'o', 8),
		(b't', 8),
		(b's', 6),
		(b'r', 6),
		(b'd', 4),
		(b'l', 4),
		(b'c', 3),
		(b'u', 3),
		(b' ', 12),
	]
	.iter().cloned().collect();

	// Check if there are only ASCII values
	if xor.len() == 0 || !xor.iter().all(|c| c < &127) {
		return 0;
	}

	xor.iter().map(|b| frequency.get(&b.to_ascii_lowercase()).unwrap_or(&0)).sum()
}

fn find_key(encrypted: &[u8]) -> u8 {
    let mut round_key = 0;
    let mut round_score = 0;
    let mut best = (0, 0);
    let mut xored = Vec::new();

    while round_key < 128 {

        round_score = 0;
        xored = xor_repeats(encrypted, &round_key);
        round_score += get_score(&xored);

        if best.1 < round_score {
            best.0 = round_key;
            best.1 = round_score;
        }
        round_key += 1;
    }

    best.0
}


fn break_xor_cipher(input: String) -> String {
    let byte_array = hex::decode(input).unwrap();
    let key = find_key(&byte_array);
    String::from_utf8(xor_repeats(&byte_array, &key)).unwrap()
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
    fn test_xor() {
        assert_eq!(
            break_xor_cipher(String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")),
            String::from("Cooking MC's like a pound of bacon")
        )
    }

}
