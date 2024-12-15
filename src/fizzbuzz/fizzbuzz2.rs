pub fn fizz_buzz2(n: u32) {
    let mut san: u32 = 3;
    let mut go: u32 = 5;
    for i in 1..n + 1 {
        san -= 1;
        go -= 1;
        if san == 0 && go == 0 {
            san = 3;
            go = 5;
            println!("FizzBuzz")
        } else if san == 0 {
            san = 3;
            println!("Fizz")
        } else if go == 0 {
            go = 5;
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}
