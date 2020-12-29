fn test(_a:i32){
    let output:&str;
    if _a%2==0{
        output="even";
    }
    else{
        output="odd";
    };
    print!("Number {} is {}\n", _a, output);
}

fn main(){
    test(3);
    test(4);
}