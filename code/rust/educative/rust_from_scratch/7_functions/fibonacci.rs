fn fib(n:i32)->i32{
    match n{
        0 => 0,
        1 => 1,
        _ => fib(n-1)+fib(n-2)
    }
}

fn main(){
    print!("{}\n", fib(5));
    print!("{}\n", fib(7));
}