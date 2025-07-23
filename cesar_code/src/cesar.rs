pub fn encrypt(input: &str, offset: u8) -> String {
    let mut output_word_in = String::new();
    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        let shifted_c = ((c as u8 - b'a' + offset) % 26 + b'a') as char;
        output_word_in.push(shifted_c);
    }
    output_word_in
}

pub fn decrypt(input: &str, offset: u8) -> String {
    let mut output_word_in = String::new();
    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        let shifted_c = ((c as u8 - b'a' - offset) % 26 + b'a') as char;
        output_word_in.push(shifted_c);
    }
    output_word_in
}
