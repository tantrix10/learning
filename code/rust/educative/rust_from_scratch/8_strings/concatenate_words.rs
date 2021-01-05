fn test(my_str:String)-> String {
    let mut out = String::new();
    for word in my_str.split_whitespace(){
        if word.starts_with("c"){
            out = out +  word + " "
        }
    }
    out.trim().to_string()
}


fn main(){
    print!("{}",test(String::from("This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course")))
}