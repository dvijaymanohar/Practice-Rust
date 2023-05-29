// fn main() {
//     let text = "  hello, world!  ";

//     let trimmed = text
//         .trim()
//         .to_lowercase()
//         .replace("world", "Rust")
//         .trim_start_matches(' ')
//         .trim_end_matches('!')
//         .to_owned();

//     println!("{}", trimmed);
// }

fn main() {
    let text = "  Hello, world!    ";

    let trimmed_text = text
        .trim()
        .to_lowercase()
        .replace("hello", "Rust")
        .to_owned();

    println!("Trimmed text: {}", trimmed_text);
}
