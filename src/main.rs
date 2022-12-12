use std::io::Read;
use std::process::Command;

use rust_code_generator::Generator;

fn main() {
    let _alphabet = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
        '-', '_', '=', '+', '[', ']', '{', '}', ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?',
        '`', '~', ' ', '\t',
    ];
    let mut file = std::fs::File::open("file.py").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = Command::new("python")
        .arg("file.py")
        .status()
        .expect("Failed to run python script :(");
    println!("status: {}", result);

    Generator::generate();
}
