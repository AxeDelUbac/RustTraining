fn main() {

    let input = "ccd";
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    for i in 0..len / 2
    {
        if chars[i] != chars[len - 1 - i] {
            println!("Not a palindrome");
        }
        else 
        {
            println!("It's a palindrome");
        }
    }
}
