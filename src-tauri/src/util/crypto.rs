extern crate crypto;

use crypto::{sha3::Sha3, digest::Digest};

/// 用于hash加盐
const SALT_LEFT: &str = "mlk$&UYBJ'/][KN6514dL";
const SALT_RIGHT: &str = "651#$GFD(&asd15][`";

/// 用于密码加密
pub fn password_crypto(pass: &String) -> String {
    let mut hasher = Sha3::sha3_512();
    let all = format!("{}{}{}", SALT_LEFT, pass, SALT_RIGHT);
    hasher.input_str(&all);
    hasher.result_str()
}


#[test]
fn test() {
    let pass = password_crypto(&"11111111".to_string());
    println!("{}", pass);
    let pass = password_crypto(&"22222222".to_string());
    println!("{}", pass);
}