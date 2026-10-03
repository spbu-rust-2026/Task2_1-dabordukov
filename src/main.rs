fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(_text: &mut String) -> &str {
    let (start, end) = match _text.split_whitespace().next() {
        Some(word) => {
            let start = match _text.find(word) {
                Some(start) => start,
                None => 0,
            };

            (start, start + word.len())
        }
        None => (0, 0),
    };

    _text.push('!');

    return &_text[start..end];
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
