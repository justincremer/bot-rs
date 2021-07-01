use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Deserialize)]
pub struct Wallet {
    tokens: HashMap<String, String>,
}

impl Wallet {
    pub fn load(path: &str) -> Self {
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        return Wallet {
            tokens: toml::from_str(contents.as_str()).unwrap(),
        };
    }

    pub fn get(&self, k: &str) -> String {
        match self.tokens.get(&k.to_string()) {
            Some(v) => v.to_string(),
            None => panic!("The token {} does not exist", k),
        }
    }
}
