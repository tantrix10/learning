enum Days {
    Monday,Tuesday,Wednesday,Thursday,Friday,Saturday,Sunday
  }
  impl Days {
     fn is_weekend(&self)->i32{
        match self{
            Days::Saturday => return 1,
            Days::Sunday => return 1,
            _ => return 0
        }
     }
  }

fn main(){
    print!("{}\n", Days::Wednesday.is_weekend());
    print!("{}\n", Days::Sunday.is_weekend());
    print!("{}\n", Days::Monday.is_weekend());
}