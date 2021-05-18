
pub fn assert_fib(number: u32, expected_fib_result: u64) {
    let fib_result = fibonacci(number);
    assert_eq!(expected_fib_result, fib_result);
    println!("fib({})={} passed!", number, fib_result);
}

fn fibonacci(number: u32) -> u64 {
    if number == 0 {
        return 0;
    }
    else if number == 1 {
        return 1;
    }
    fibonacci(number - 1) + fibonacci(number - 2)
}
