#![allow(dead_code)] 
//declare a structure
struct Car {
   owner_age:i32,
}
struct Motorbike {
   owner_age:i32,
}
//declare a trait
trait Drive {
   fn can_drive(&self)->i32;
}
//implement the trait
impl Drive for Car{
   fn can_drive(&self)->i32{
       if self.owner_age<18{
           0
       }else{
        1
       }
   }
}
impl Drive for Motorbike{
   fn can_drive(&self)->i32{
      if self.owner_age<14{
           0
       }else{
        1
       }
   }
}

fn main(){
    print!("{}\n", Car{owner_age:12}.can_drive());
    print!("{}\n", Car{owner_age: 21}.can_drive());
    print!("{}\n", Motorbike{owner_age: 13}.can_drive());
    print!("{}\n", Motorbike{owner_age: 15}.can_drive())
}