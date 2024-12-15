pub fn fizz_buzz(n: u32) {
    for i in 1..n + 1 {
        if i % 15 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}
