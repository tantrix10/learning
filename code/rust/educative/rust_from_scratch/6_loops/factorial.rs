fn test(n:i32){
    let mut out:i32 = 1;
    if n < 0{
        print!("0\n")
    }
    else{
        for i in 1..n+1{
            out *= i;
        } 
        print!("{}\n", out);
    }
}

fn main(){
    test(2);
    test(3);
    test(4);
    test(0);
    test(-1);
    test(-5);
}