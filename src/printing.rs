use colored::Colorize;

// This code sucks! Can't handle any multiline inputs, and looks seriously clunky.
// There must be a better way to write this.
pub fn print_box_header(text: String) {
    let mut header: String = "╔".to_string();
    for _ in 0..text.len() + 2 {
        header.push_str("═");
    }
    header.push_str("╗");
    header.push_str("\n║ ");
    header.push_str(&text);
    header.push_str(" ║\n╚");
    for _ in 0..text.len() + 2 {
        header.push_str("═");
    }
    header.push_str("╝");
    println!("{}", header.blue().bold());
}
