fn fib(n:i32)-> i32{
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}

fn main(){
    print!("{}\n", fib(3));
    print!("{}\n", fib(13));
    print!("{}\n", fib(30));
    // OVERFLOWWWWW
    // print!("{}\n", fib(50))
}