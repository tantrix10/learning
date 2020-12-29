fn test(n:i32){
    for i in 1..n+1{
        print!("\n");
        for _ in 1..i+1{
            print!("&")
        }
    };
    print!("\n");
}

fn main(){
    test(5);
    test(6);
}