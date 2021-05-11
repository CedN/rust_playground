fn main() {
    assert_fib(0, 0);
    assert_fib(1, 1);
    assert_fib(2, 1);
    assert_fib(3, 2);
    assert_fib(9, 34);
    assert_fib(18, 2584);
}

fn assert_fib(number: u32, expected_fib_result: u64) {
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
