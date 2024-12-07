#![allow(unused)]

use std::thread;

fn words(content: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let mut last = 0;
    let mut last_char = 0;
    for (i, byte) in content.bytes().enumerate() {
        if byte == b' ' {
            words.push(&content[last..i]);
            last = i + 1;
        }
        last_char = byte;
    }
    if last_char != b' ' {
        words.push(&content[last..]);
    }
    words
}

fn main() {
    let words = words("String in Main Thread");
    for word in words {
        println!("Word: {:?}", word);
    }
}
