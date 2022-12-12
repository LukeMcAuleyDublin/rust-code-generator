const ALPHABET: &'static [&'static char] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
    'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
    'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '@', '#', '$', '%', '^',
    '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}', ';', ':', '\'', '"', ',',
    '.', '<', '>', '/', '?', '`', '~', ' ', '\t',
];

pub struct Generator {
    pub file_path: String,
    pub string_to_run: String,
    pub attempted_characters: Vec<char>,
    pub index_of_last_attempted: u32,
}

pub fn run(gen: Generator) -> Result<(), Box<dyn Error>> {
    println!("Running code...");
    Ok(())
}

impl Generator {

    pub fn build() -> Result<Generator, &'static str> {
        let file_path = String::from("file.py");
        let string_to_run = String::from(" ");
        let attempted_characters = Vec::new();
        let index_of_last_attempted = 0;

        Ok(Generator {
            file_path,
            string_to_run,
            attempted_characters,
            index_of_last_attempted,
        })
    }

    pub fn generate_code(gen: self) {
        for c in &ALPHABET {
            gen.attempted_c
        }
    }

}
