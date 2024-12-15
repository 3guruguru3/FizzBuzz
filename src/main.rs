mod core;
mod fizzbuzz;
mod spec;

// Rust的にmainで実装せなあかんかも
//
fn main() {
    println!("オブジェクト指向みたいな");
    fizzbuzz::fizzbuzz_oop::fizz_buzz(30);
    println!("普通の");
    fizzbuzz::fizzbuzz::fizz_buzz(30);
}
