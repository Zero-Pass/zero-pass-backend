use crate::{CipherError, CipherResult};

pub fn b64(vw: &str) -> CipherResult {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut binary_word = "".to_string();
    let mut new_pass = String::new();

    for i in vw.chars() {
        if !alphabet.contains(i) {
            return Err(CipherError::InvalidCharacterError);
        }
    }

    for i in &mut (*vw).bytes() {
        binary_word += &format!("0{:b}", i);
    }

    let mut padding = "".to_string();
    while binary_word.len() % 6 != 0 {
        binary_word += "00";
        padding += "=";
    }

    let mut x = 0;
    let mut new_binary = String::new();
    for i in binary_word.chars() {
        if x == 6 {
            new_binary += " ";
            x = 0
        }
        new_binary += &i.to_string();
        x += 1;
    }

    let binary_vec: Vec<&str> = new_binary.split(' ').collect();
    for i in binary_vec {
        let number = usize::from_str_radix(i, 2).unwrap();
        new_pass += &(alphabet.as_bytes()[number] as char).to_string();
    }

    new_pass += &padding;
    Ok(new_pass)
}
