
use std::io;
use regex::Regex;

fn main() {
    let re = Regex::new(r"[lr]").unwrap();

    loop {
        let mut input_text = String::new();

        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");

        if input_text.trim().to_lowercase() == "exit" {
            break;
        } else {
            input_text = input_text.trim().to_string();

            let translated_msg = re.replace_all(&input_text, "w");

            print!("{} :3\n", translated_msg);
        }
    }
}
