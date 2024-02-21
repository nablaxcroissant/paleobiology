use draw3d::geometry::{Draw, Geometry}; // Note that draw being in the geometry module is confusing
use draw3d::app::App;
use draw3d::vertex::Vertex;
use glam::f32::{Vec3, Quat};
use std::f32::consts::PI;
use rand::prelude::*;

enum Command{
    DrawForward,    // Draw forward
    Forward,        // Move forward
    ZaxisPlus,      //Z rotation positive
    ZaxisMinus,     //Z rotation negative
    Pop,
    Push,
}

pub struct Tree{
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
        Tree {
            p1,
            p2,
            phi1,
            phi2,
            gam1,
            gam2,
        }
    }
}

impl Draw for Tree {
    fn draw(&self, app: &App) -> Geometry {
        let mut turtle_stack: Vec<Turtle> = Vec::new();
        let mut index_stack: Vec<u16> = Vec::new();

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut turtle = Turtle::new();
        let mut index: u16 = 0;

        let mut rng = rand::thread_rng();

        vertices.push(turtle.position_as_vertex());
        indices.push(index);
        turtle.forward(0.01);
        index += 1;
        vertices.push(turtle.position_as_vertex());
        indices.push(index);
        turtle_stack.push(turtle.clone());
        index_stack.push(index);

        for i in 0..10{
            vertices.push(turtle.position_as_vertex());
            indices.push(index);
            turtle.forward(0.01);
            index += 1;
            vertices.push(turtle.position_as_vertex());
            indices.push(index);

            if rng.gen() < self.p1 {
                turtle_stack.push(turtle.clone());
                index_stack.push(index);
            }
        }

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