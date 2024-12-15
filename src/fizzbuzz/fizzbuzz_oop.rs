use crate::core::core::NumberConverter;
use crate::spec::spec::CyclicnumberRule;
use crate::spec::spec::PassThroughRule;

pub fn fizz_buzz(n: u32) {
    let fizz_rule: CyclicnumberRule = CyclicnumberRule {
        base: 3,
        replacement: String::from("Fizz"),
    };

    let buzz_rule: CyclicnumberRule = CyclicnumberRule {
        base: 5,
        replacement: String::from("Buzz"),
    };

    let pass_rule: PassThroughRule = PassThroughRule {};

    let fizzbuzz: NumberConverter = NumberConverter {
        rules: vec![
            Box::new(fizz_rule),
            Box::new(buzz_rule),
            Box::new(pass_rule),
        ],
    };

    // これはただVecを表示させてるだけ
    let fizzbuzz_results: Vec<String> = fizzbuzz.convert(n);
    for fizzbuzz_result in fizzbuzz_results.iter() {
        println!("{}", fizzbuzz_result);
    }
}
