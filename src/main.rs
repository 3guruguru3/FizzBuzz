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
    println!("別の");
    fizzbuzz::fizzbuzz2::fizz_buzz2(30);
    fizzbuzz::fizzbuzz_power::fizzbuzz_power();
}
