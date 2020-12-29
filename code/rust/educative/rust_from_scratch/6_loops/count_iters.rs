fn test(mut x:i32){
    let mut count:i32 = 0;
    while x>=0{
        count += 1;
        x-=3
    }
    print!("{}\n", count)   
}

fn main(){
    test(21);
    test(32);
}