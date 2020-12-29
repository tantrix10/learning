fn test(n:i32)->i32{
    let three_test = n%3;
    let four_test = n%4;
    if three_test == 0 && four_test == 0{
        return 0
    }
    else if three_test == 0{
        return 1
    }
    else if four_test == 0{
        return 2
    }
    else {
        return -1
    }
}

fn main(){
    let a = test(12);
    let b = test(9);
    let c = test(16);
    let d = test(19);
    print!("Answers: {} {} {} {} \n", a,b,c,d)
}