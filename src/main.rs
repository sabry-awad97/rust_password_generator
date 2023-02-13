use rand::seq::SliceRandom;

const ASCII_SYMBOL_START: u8 = 33;
const ASCII_SYMBOL_END: u8 = 47;
const ASCII_NUMBER_START: u8 = 48;
const ASCII_NUMBER_END: u8 = 57;
const ASCII_UPPERCASE_START: u8 = 65;
const ASCII_UPPERCASE_END: u8 = 90;
const ASCII_LOWERCASE_START: u8 = 97;
const ASCII_LOWERCASE_END: u8 = 122;

fn generate_chars(start: u8, end: u8) -> Vec<char> {
    (start..=end).map(|i| i as char).collect()
}

fn main() {
    let letters = generate_chars(ASCII_LOWERCASE_START, ASCII_LOWERCASE_END)
        .into_iter()
        .chain(generate_chars(ASCII_UPPERCASE_START, ASCII_UPPERCASE_END).into_iter())
        .collect::<Vec<char>>();

    let numbers = generate_chars(ASCII_NUMBER_START, ASCII_NUMBER_END);
    let symbols = generate_chars(ASCII_SYMBOL_START, ASCII_SYMBOL_END);

    println!("Welcome to the Rust Password Generator!");

    let mut rng = rand::thread_rng();
    println!("How many letters would you like in your password?");
    let nr_letters = read_input();

    println!("How many symbols would you like?");
    let nr_symbols = read_input();

    println!("How many numbers would you like?");
    let nr_numbers = read_input();

    let mut password = Vec::with_capacity(nr_letters + nr_symbols + nr_numbers);
    for _ in 0..nr_letters {
        password.push(*letters.choose(&mut rng).unwrap());
    }
    for _ in 0..nr_symbols {
        password.push(*symbols.choose(&mut rng).unwrap());
    }
    for _ in 0..nr_numbers {
        password.push(*numbers.choose(&mut rng).unwrap());
    }
    password.shuffle(&mut rng);

    println!("Your password is: {}", password.into_iter().collect::<String>());
}

fn read_input() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
