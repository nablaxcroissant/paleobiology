use draw3d::geometry::{Draw, Geometry}; // Note that draw being in the geometry module is confusing
use draw3d::app::App;
use glam:f32:Vec3;

enum Command{
    DrawForward,
    Forward,
    Right,
    Left,
    Pop,
    Push,
}

struct Tree{
    gen_string: String, // L-system descriptor of the tree in its current state
    p1: f32,
    p2: f32,
    phi1: f32,
    phi2: f32,
    gam1: f32,
    gam2: f32,
}

impl Tree{
    fn new(
        start_string: String, 
        p1: f32, 
        p2: f32, 
        phi1: f32,
        phi2: f32,
        gam1: f32,
        gam2: f32
    ) -> Tree {
        Tree {
            gen_string: start_string,
            p1,
            p2,
            phi1,
            phi2,
            gam1,
            gam2,
        }
    }
}

struct Turtle{
    position: Vec3,
    angle: Vec3,
}

impl Turtle{
    //TODO: Make turtle move
}

impl Draw for Tree {
    fn draw(&self, app: &App) -> Geometry {
        //TODO: Parse gen string into commands

        //TODO: Run commands

        Geometry::new_line(app, vertices, indices)
    }
}