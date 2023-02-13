use rand::seq::SliceRandom;

const ASCII_SYMBOL_START: u8 = 33;
const ASCII_SYMBOL_END: u8 = 47;
const ASCII_NUMBER_START: u8 = 48;
const ASCII_NUMBER_END: u8 = 57;
const ASCII_UPPERCASE_START: u8 = 65;
const ASCII_UPPERCASE_END: u8 = 90;
const ASCII_LOWERCASE_START: u8 = 97;
const ASCII_LOWERCASE_END: u8 = 122;

enum CharacterType {
    LowerCaseLetter,
    UpperCaseLetter,
    Number,
    Symbol,
}

struct CharacterRange {
    start: u8,
    end: u8,
    character_type: CharacterType,
}

impl CharacterRange {
    fn new(start: u8, end: u8, character_type: CharacterType) -> Self {
        Self {
            start,
            end,
            character_type,
        }
    }

    fn generate_chars(&self) -> Vec<char> {
        (self.start..=self.end).map(|i| i as char).collect()
    }
}

struct PasswordGenerator {
    character_ranges: Vec<CharacterRange>,
}

impl PasswordGenerator {
    fn new() -> Self {
        let mut character_ranges = Vec::new();
        character_ranges.push(CharacterRange::new(
            ASCII_LOWERCASE_START,
            ASCII_LOWERCASE_END,
            CharacterType::LowerCaseLetter,
        ));
        character_ranges.push(CharacterRange::new(
            ASCII_UPPERCASE_START,
            ASCII_UPPERCASE_END,
            CharacterType::UpperCaseLetter,
        ));
        character_ranges.push(CharacterRange::new(
            ASCII_NUMBER_START,
            ASCII_NUMBER_END,
            CharacterType::Number,
        ));
        character_ranges.push(CharacterRange::new(
            ASCII_SYMBOL_START,
            ASCII_SYMBOL_END,
            CharacterType::Symbol,
        ));

        Self { character_ranges }
    }

    fn generate_password(
        &self,
        nr_lower_letters: usize,
        nr_upper_letters: usize,
        nr_symbols: usize,
        nr_numbers: usize,
    ) -> String {
        let mut rng = rand::thread_rng();
        let mut password =
            Vec::with_capacity(nr_lower_letters + nr_upper_letters + nr_symbols + nr_numbers);
        
        for range in &self.character_ranges {
            let chars = range.generate_chars();

            match range.character_type {
                CharacterType::LowerCaseLetter => {
                    for _ in 0..nr_lower_letters {
                        password.push(*chars.choose(&mut rng).unwrap());
                    }
                }

                CharacterType::UpperCaseLetter => {
                    for _ in 0..nr_upper_letters {
                        password.push(*chars.choose(&mut rng).unwrap());
                    }
                }

                CharacterType::Symbol => {
                    for _ in 0..nr_symbols {
                        password.push(*chars.choose(&mut rng).unwrap());
                    }
                }

                CharacterType::Number => {
                    for _ in 0..nr_numbers {
                        password.push(*chars.choose(&mut rng).unwrap());
                    }
                }
            }
        }

        password.shuffle(&mut rng);
        password.into_iter().collect::<String>()
    }
}

fn read_input() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    println!("Welcome to the Rust Password Generator!");

    println!("How many lower letters would you like in your password?");
    let nr_lower_letters = read_input();

    println!("How many upper letters would you like?");
    let nr_upper_letters = read_input();

    println!("How many symbols would you like?");
    let nr_symbols = read_input();

    println!("How many numbers would you like?");
    let nr_numbers = read_input();

    let password_generator = PasswordGenerator::new();
    let password = password_generator.generate_password(
        nr_lower_letters,
        nr_upper_letters,
        nr_symbols,
        nr_numbers,
    );
    println!("Your password is: {}", password);
}
