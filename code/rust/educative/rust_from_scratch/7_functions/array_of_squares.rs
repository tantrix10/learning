fn arr_square() ->[i32;5]{
    let mut out: [i32;5] = [1;5];
    for i in 0..5{
        let temp = i as i32;
        out[i] = (temp+1).pow(2);
    }
    return out
}

fn main(){
    let a: [i32; 5] = arr_square();
    print!("{:?}", a)
}