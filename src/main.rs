fn main() {
    for x in 1..101 {
        match x {
            x if x % 15 == 0 => println!("FizzBuzz"),
            x if x % 5 == 0  => println!("Buzz"),
            x if x % 3 == 0  => println!("Fizz"),
            _                => println!("{}", x),
        }
    }
}
