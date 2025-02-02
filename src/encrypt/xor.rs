use crate::prelude::*;
use crate::Encrypter;

use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Xor;

impl FromStr for Xor {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        if input.to_uppercase() == "XOR" {
            Ok(Xor)
        } else {
            Err(Error::InvalidMethodError(input.to_string()))
        }
    }
}

impl Encrypter for Xor {
    fn encrypt(self, context_word: &[u8], service_word: &[u8]) -> Result<Box<[u8]>> {
        let mut service_word = service_word.to_vec();
        let service_word_iterator = service_word.clone();
        let mut service_word_iterator = service_word_iterator.iter().cycle();
        while context_word.len() > service_word.len() {
            if let Some(c) = service_word_iterator.next() {
                service_word.push(*c);
            } else {
                return Err(Error::InvalidCharacterError);
            }
        }

        let mut scrambled_word: Vec<u8> = Vec::new();

        for (i, character) in context_word.iter().enumerate() {
            let new_character = character ^ service_word[i];
            scrambled_word.push(new_character);
        }

        Ok(Box::from(scrambled_word))
    }
}

// #[async_trait]
// impl Method for Xor {
//     async fn encrypt(&self, uw: Arc<str>, vw: Arc<str>) -> Result<String> {
//         let mut binary_vw_word = String::new();
//         let mut binary_uw_word = String::new();
//         let mut new_pass = String::new();

//         for i in vw.bytes() {
//             binary_vw_word.push_str(&format!("0{:b}", i));
//         }
//         for i in uw.bytes() {
//             binary_uw_word.push_str(&format!("0{:b}", i));
//         }

//         let mut x = 0;
//         let mut binary_pass = String::new();
//         for i in 0..binary_uw_word.len() {
//             if x == 8 {
//                 binary_pass += " ";
//                 x = 0
//             }
//             let uw_val = &(binary_uw_word.as_bytes()[{
//                 if i > binary_uw_word.len() - 1 {
//                     i - (binary_uw_word.len() * (i / binary_uw_word.len()))
//                 } else {
//                     i
//                 }
//             }] as char)
//                 .to_string()
//                 .parse::<u8>()
//                 .unwrap();
//             let vw_val = &(binary_vw_word.as_bytes()[i] as char)
//                 .to_string()
//                 .parse()
//                 .unwrap();
//             binary_pass.push_str(&format!("{:b}", uw_val ^ vw_val));
//             x += 1;
//         }

//         let binary_vec: Vec<&str> = binary_pass.split(' ').collect();
//         for i in binary_vec {
//             let number = usize::from_str_radix(i, 2).unwrap();
//             let val = match ALPHABET.get(number) {
//                 Some(v) => v,
//                 None => continue,
//             };
//             new_pass.push(*val);
//         }

//         Ok(new_pass)
//     }
// }
