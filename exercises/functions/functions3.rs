// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    // ADDED
    call_me(33);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
