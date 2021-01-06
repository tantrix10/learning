struct Point {
	x: i32,
	y: i32
}
fn test(_point1: Point, _point2: Point)-> f32 {
    let x1 =  _point1.x as f32;
    let x2 =  _point2.x as f32;
    let y1 =  _point1.y as f32;
    let y2 =  _point2.y as f32;
    ((x1-x2).powi(2)+(y1-y2).powi(2)).sqrt()
}


fn main(){
    let p1 = Point{
        x: 1,
        y: 1
    };
    let p2 = Point{
        x: 2,
        y: 2
    };
    print!("{}", test(p1,p2))
}