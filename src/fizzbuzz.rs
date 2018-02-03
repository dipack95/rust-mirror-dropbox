fn is_divisible_by(numerator: i32, denominator: i32) -> boolean {
    return numerator % denominator == 0;
}

fn fizzbuzz(num: u32) {
    if is_divisible_by(num, 15) {
        println!("FizzBuzz");
    } else if is_divisible_by(num, 5) {
        println!("Fizz");
    } else if is_divisible_by(num, 3) {
        println!("Buzz");
    }
}

fn fizzbuzz_until(num: u32) {
    for n in 0..num {
        fizzbuzz(n);
    }
}

fn main() {
    let max_fizzbuzz: u32 = 100;
    fizzbuzz_until(max_fizzbuzz);
}