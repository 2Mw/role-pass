extern crate crypto;

use hex::{ToHex};
use crypto::{sha3::Sha3, digest::Digest, chacha20::ChaCha20, symmetriccipher::{Encryptor, Decryptor}, buffer::{RefReadBuffer, RefWriteBuffer, BufferResult, WriteBuffer, ReadBuffer}};

/// 用于hash加盐
const SALT_LEFT: &str = "mlk$&UYBJ'/][KN6514dL";
const SALT_RIGHT: &str = "651#$GFD(&asd15][`";
// Chacha 20
const NONCE: &[u8] = "abcd[]8(".as_bytes();

/// 用于密码加密
pub fn password_crypto(pass: &String) -> String {
    let mut hasher = Sha3::sha3_512();
    let all = format!("{}{}{}", SALT_LEFT, pass, SALT_RIGHT);
    hasher.input_str(&all);
    hasher.result_str()
}

pub fn encrypt(key: &String, data: &String) -> String {
    let key = &key.as_str()[0..32].to_string();
    let mut final_result = Vec::<u8>::new();
    let key = key.as_bytes();
    let mut encoder = ChaCha20::new(key, NONCE);
    // 
    let mut read_buffer = RefReadBuffer::new(data.as_bytes());
    let mut buf = [0; 4096];
    let mut write_buf = RefWriteBuffer::new(&mut buf);
    loop {
        let result = encoder.encrypt(&mut read_buffer, &mut write_buf, true).unwrap();

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(write_buf.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        };
    }
    
    final_result.encode_hex()
}

pub fn decrypt(key: &String, payload: &String) -> String {
    let key = &key.as_str()[0..32].to_string();
    let mut final_result = Vec::<u8>::new();
    let key = key.as_bytes();
    let mut decoder = ChaCha20::new(key, NONCE);
    // 
    let p = hex::decode(payload).unwrap();
    let p = p.as_slice();
    let mut read_buffer = RefReadBuffer::new(p);
    let mut buf = [0; 4096];
    let mut write_buf = RefWriteBuffer::new(&mut buf);
    loop {
        let result = decoder.decrypt(&mut read_buffer, &mut write_buf, true).unwrap();

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(write_buf.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        };
    }
    let s: String = final_result.encode_hex();
    let res = hex::decode(s).unwrap();
    String::from_utf8(res).unwrap()
}


#[test]
fn test() {
    let pass = password_crypto(&"11111111".to_string());
    println!("{}", pass);
    let pass = password_crypto(&"22222222".to_string());
    println!("{}", pass);
}

#[test]
fn test_en() {
    let key = "123456781234567knasdlkasjd902873912/'].fds'f;sd8".to_string();
    // let key = &key[0..32].to_string();
    let o = "香辣脆皮鸡🐕~~@！#!xixi".to_string();
    let a = encrypt(&key, &o);
    let b = decrypt(&key, &a);
    println!("{}\n{}", a, b);
    assert_eq!(o, b);
}