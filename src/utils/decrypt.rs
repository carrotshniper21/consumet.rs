use openssl::symm::{decrypt, Cipher};

const KEY_SIZE: u8 = 48;

/// Decrypt Encoded Url Sources
/// # Parameters
/// * `encrypted_url` - The AES-256-CBC Encrypted url.
/// * `secret` - the decryption key as bytes.
/// ```
/// let decypted = decrypt::decrypt_url(encrypted_url, &key.into_bytes());
/// println!("{}", decrypted);
/// ```
pub fn decrypt_url(
    encrypted_url: String,
    secret: &Vec<u8>,
) -> anyhow::Result<String> {
    let raw_url =
        openssl::base64::decode_block(encrypted_url.as_str()).expect("Base64 decryption failed.");

    let salt_key = &raw_url[8..16].to_vec();

    fn md5(key: Vec<u8>, secret: &Vec<u8>, salt: &Vec<u8>) -> Vec<u8> {
        let mut vector: Vec<u8> = vec![];
        vector.extend(key);
        vector.extend(secret);
        vector.extend(salt);
        md5::compute(vector).to_vec()
    }

    let mut key = md5(vec![], secret, salt_key);
    let mut current_key: Vec<u8> = key.clone();

    while current_key.len() < (KEY_SIZE as usize) {
        key = md5(key, secret, salt_key);
        current_key.extend(&key);
    }

    let cipher = Cipher::aes_256_cbc();

    let decrypted_url = decrypt(
        cipher,
        &current_key[..32],
        Some(&current_key[32..]),
        &raw_url[16..],
    )?;

    Ok(String::from_utf8(decrypted_url).expect("Expected an UTF-8 read."))
}
