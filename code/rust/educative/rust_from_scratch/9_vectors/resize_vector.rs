fn test(my_vec: &mut Vec<u32>)-> &mut Vec<u32>{
    // Write code here!
    my_vec.pop();
    print!("{} {} ", my_vec.len(), my_vec.len()/2);
    // I'm going to leave this here as a testament to my panicing
    // when confronted by compiler messages late at night, haha.
    //my_vec.remove(((my_vec.len() as u32 -1)/2) as u32 as usize);
    my_vec.remove((my_vec.len()/2 )-1);
    let mut total = 0;
    for ele in my_vec.iter(){
        total += ele
    }
    my_vec.push(total);
    my_vec
 }


 fn main(){
    let mut vec1:Vec <u32> = vec![1,5,7,9];
    let mut vec2:Vec <u32> = vec![1,2,3,1,2,6];
    print!("{:?}", test(&mut vec1));
    print!("{:?}", test(&mut vec2))
 }