// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();
    // ADDED &
    get_char(&data);
    // REMOVED &
    string_uppercase(data);
}

// Should not take ownership
// ADDED &
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// REMOVED &
fn string_uppercase(mut data: String) {
    // REMOVED &
    data = data.to_uppercase();

    println!("{}", data);
}
