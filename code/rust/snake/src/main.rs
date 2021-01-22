extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;
use rand::Rng;

struct Dot{
    pub pos: Vec<f64>,
    pub vel: Vec<f64>,
}

impl Dot{
    pub fn create(x: f64, y: f64) -> Dot{
        let mut rng = rand::thread_rng();
        Dot{
            pos: vec![x, y],
            vel: vec![rng.gen(), rng.gen()]

        }
    }

    pub fn update(&mut self, time: f64){
        self.pos[0] += time*self.vel[0];
        self.pos[1] += time*self.vel[1];

        if self.pos[0] > 500.0{
            self.pos[0] = 500.0;
            self.vel[0] *= -1.0;
        }
        if self.pos[0] < 0.0{
            self.pos[0] = 0.0;
            self.vel[0] *= -1.0;
        }
        if self.pos[1] > 500.0{
            self.pos[1] = 500.0;
            self.vel[1] *= -1.0;
        }
        if self.pos[1] < 0.0{
            self.pos[1] = 0.0;
            self.vel[1] *= -1.0;
        }
    }
}

struct Game{
    pub scene: usize,
    pub dots: Vec<Dot>,
    pub time_step: f64
}

impl Game {
    pub fn new(param_scene: usize, time: f64) -> Game {
        Game {
            scene: param_scene,
            dots: vec![],
            time_step: time
        }
    }
    pub fn set_scene(&mut self, scene: usize) {
        self.scene = scene;
    }

    pub fn spawn_dots(&mut self, number: i32){
        let mut rng = rand::thread_rng();
        for _ in 1..number{
            self.dots.push(Dot::create(rng.gen(), rng.gen()))
        }
    }

    pub fn update(&mut self){
        for dot in self.dots.iter_mut(){
            dot.update(self.time_step);
        }
    }
}
fn main() {
    println!("Hello, world!");
    let opengl = OpenGL::V3_2;
    let mut game = Game::new(1, 1.0);
    game.spawn_dots(20);
    let mut window: PistonWindow = WindowSettings::new(
        "DOTS", [700, 700])
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();
    while let Some(e) = window.next(){
        match e{
           Input::Update(_)=> {
               game.update();
           } 
            Input::Render(_)=>{ 
                window.draw_2d(&e, |c, g| {
                    clear([0.0,0.0,0.0,1.0], g);
                    for f in game.dots.iter() {
                        let dot_shape = rectangle::square(0.0, 0.0, 10.0);
                        rectangle([1.0, 0.0, 0.0, 1.0], dot_shape, c.transform.trans(
                            f.pos[0], f.pos[1]), g);
                    };
                });
            }

            _ =>{}
    }
}
}


