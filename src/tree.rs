use draw3d::geometry::{Draw, Geometry}; // Note that draw being in the geometry module is confusing
use draw3d::app::App;
use draw3d::vertex::Vertex;
use glam::f32::{Vec3, Quat};
use std::f32::consts::PI;

enum Command{
    DrawForward,    // Draw forward
    Forward,        // Move forward
    ZaxisPlus,      //Z rotation positive
    ZaxisMinus,     //Z rotation negative
    Pop,
    Push,
}

pub struct Tree{
    pub gen_string: String, // L-system descriptor of the tree in its current state
    p1: f32,
    p2: f32,
    phi1: f32,
    phi2: f32,
    gam1: f32,
    gam2: f32,
}

impl Tree{
    pub fn new(
        p1: f32, 
        p2: f32, 
        phi1: f32,
        phi2: f32,
        gam1: f32,
        gam2: f32
    ) -> Tree {
        let start_string = String::from("X");
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

    pub fn iterate(&mut self) {
        let mut new = self.gen_string.clone();
        let mut count = 0;
        for (i, c) in self.gen_string.chars().enumerate(){
            match c {
                'A' => {
                    new.insert_str(i+count, "[wxA]yzB");
                    count += 8;
                }
                'B' => {
                    new.insert_str(i+count, "[wxA]yzB");
                    count += 8;
                }
                'F' => {
                    new.replace_range(i+count..i+count+1, "FF");
                    count += 1;
                }
                'X' => {
                    new.replace_range(i+count..i+count+1, "F[wX]F[yX]wX");
                    count += 11;
                }
                _ => ()
            }
        }
        self.gen_string = new;
    }
}

impl Draw for Tree {
    fn draw(&self, app: &App) -> Geometry {
        //TODO: Parse gen string into commands
        let mut turtle_stack: Vec<Turtle> = Vec::new();
        let mut index_stack = Vec::new();

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut turtle = Turtle::new();
        let mut index: u16 = 0;
                    
        for char in self.gen_string.chars() {
            match char {
                'A' => {
                    vertices.push(turtle.position_as_vertex());
                    indices.push(index);
                    index = vertices.len() as u16;
                    turtle.forward(0.2);
                    vertices.push(turtle.position_as_vertex());
                    indices.push(index);
                    index += 1;
                },
                'B' => {
                    vertices.push(turtle.position_as_vertex());
                    indices.push(index);
                    index = vertices.len() as u16;
                    turtle.forward(0.2);
                    vertices.push(turtle.position_as_vertex());
                    indices.push(index);
                    index += 1;
                },
                'F' => {
                    vertices.push(turtle.position_as_vertex());
                    indices.push(index);
                    index = vertices.len() as u16;
                    turtle.forward(0.004);
                    vertices.push(turtle.position_as_vertex());
                    indices.push(index);
                    index += 1;
                }
                '[' => {
                    turtle_stack.push(turtle.clone());
                    index_stack.push(index.clone());
                }
                ']' => {
                    turtle = turtle_stack.pop().unwrap();
                    index = index_stack.pop().unwrap();
                }
                'w' => {
                    turtle.rot_z(0.4);
                }
                'y' => {
                    turtle.rot_z(-0.4);
                }
                _ => {}
            }
        }

        //TODO: Run commands

        Geometry::new_line(app, &vertices, &indices)
    }
}

#[derive(Clone)]
struct Turtle{
    position: Vec3,
    direction: Quat,
}

impl Turtle{
    //TODO: Make turtle move
    fn new() -> Turtle {
        let position = -Vec3::Y;
        let direction = Quat::IDENTITY;
        Turtle {
            position,
            direction,
        }
    }

    fn position(&self) -> Vec3 {
        self.position
    }

    fn position_as_vertex(&self) -> Vertex {
        let p = self.position;
        Vertex::new([p.x, p.y, p.z], [0., 0.5, 0.])
    }

    fn forward(&mut self, d: f32) {
        let up = Vec3::Y;
        let forward = d*self.direction.mul_vec3(up);
        self.position += forward;
    }

    fn rot_z(&mut self, a: f32) {
        let rot = Quat::from_rotation_z(a);
        self.direction*=rot;
    }
}