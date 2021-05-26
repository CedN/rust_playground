use std::io;

mod fibo;
use fibo::assert_fib;

mod secret_number;
use secret_number::play;

mod word_count;
use word_count::word_count;

fn main() {
    println!("Select an item:");
    println!("1 => Fibonacci tests");
    println!("2 => Play to guess a number");
    println!("3 => Count word of file in argument");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Err during IO reading");
    let choice: u32 = buffer.trim().parse().expect("The choice is not a number");
    match choice {
        1 => test_fibonacci(),
        2 => play(),
        3 => word_count(),
        _ => println!("Unknown choice")
    };
}

fn test_fibonacci() {
    assert_fib(0, 0);
    assert_fib(1, 1);
    assert_fib(2, 1);
    assert_fib(3, 2);
    assert_fib(9, 34);
    assert_fib(18, 2584);
}
