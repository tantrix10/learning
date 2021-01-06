fn my_area(base: i32, height: i32)->f32{
    (0.5)*(base as f32)*(height as f32)
}


mod shapes{
    pub fn triangle_area(base : i32, height : i32) {
        // call the root function 'my_area' and print the return value
        print!("{}\n",super::my_area(base, height))
    }
}

fn main(){
    shapes::triangle_area(3,4);
    shapes::triangle_area(7,5)
}