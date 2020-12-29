fn test(a: i32, operator: char, b: i32){
    match operator{
        '+' => print!("{}\n",a+b),
        '-' => print!("{}\n", a-b),
        '*' => print!("{}\n", a*b),
        '%' => print!("{}\n", a%b),
        '/' => if b != 0{
            print!("{}\n", a/b)
        } 
        else{
            print!("Division by 0 is undefined\n")
        },
        _ => print!("invalid operator\n")
        

    };
}

fn main(){
    test(3,'+',2);
    test(1,'/',0);
    test(4,'*',3);
    test(3, 'b', 3);
}