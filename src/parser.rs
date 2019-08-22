use std::ops;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub triangles: Vec<Triangle3>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            position: Vec3::ORIGIN,
            rotation: Vec3::ORIGIN,
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            triangles: Vec::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[allow(dead_code)]
    pub const ORIGIN: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Vertex3 {
    pub pos: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

pub type Triangle3 = (Vertex3, Vertex3, Vertex3);

use std::fs;

fn str_to_vec3(s: &str) -> Option<Vec3> {
    let bits: Vec<&str> = s.trim().split(" ").collect();
    if bits.len() != 3 {
        None
    } else {
        Some(Vec3 {
            x: bits[0].parse().unwrap(),
            y: bits[1].parse().unwrap(),
            z: bits[2].parse().unwrap(),
        })
    }
}

fn str_to_vec2(s: &str) -> Option<Vec2> {
    let bits: Vec<&str> = s.split(" ").collect();
    if bits.len() < 2 {
        None
    } else {
        Some(Vec2 {
            x: bits[0].parse().unwrap(),
            y: bits[1].parse().unwrap(),
        })
    }
}

type VertexDescription = (usize, usize, usize);
type TriangleDescription = (VertexDescription, VertexDescription, VertexDescription);

fn str_to_vertex_description(s: &str) -> Option<VertexDescription> {
    let bits: Vec<&str> = s.split("/").collect();
    if bits.len() != 3 {
        None
    } else {
        Some((
            bits[0].parse::<usize>().unwrap() - 1,
            bits[1].parse::<usize>().unwrap() - 1,
            bits[2].parse::<usize>().unwrap() - 1,
        ))
    }
}

fn str_to_triangle_description(s: &str) -> Option<TriangleDescription> {
    let bits: Vec<&str> = s.trim().split(" ").collect();
    if bits.len() != 3 {
        None
    } else {
        Some((
            str_to_vertex_description(bits[0]).expect("Failed to parse vertex"),
            str_to_vertex_description(bits[1]).expect("Failed to parse vertex"),
            str_to_vertex_description(bits[2]).expect("Failed to parse vertex"),
        ))
    }
}

pub fn parse(path: String) -> Object {
    let object_string = fs::read_to_string(path).expect("Cannot read .obj file!");
    let lines = object_string.lines();
    let mut positions: Vec<Vec3> = vec![];
    let mut normals: Vec<Vec3> = vec![];
    let mut uvs: Vec<Vec2> = vec![];
    let mut triangle_descriptions: Vec<TriangleDescription> = vec![];

    for (i, line) in lines.collect::<Vec<&str>>().iter().enumerate() {
        if line.len() < 2 {
            continue;
        }

        match &line[0..2] {
            "v " => match str_to_vec3(&line[2..]) {
                Some(vec3) => positions.push(vec3),
                None => panic!("Malformed 'v' value at {}!", i),
            },
            "vn" => match str_to_vec3(&line[3..]) {
                Some(vec3) => normals.push(vec3),
                None => panic!("Malformed 'vn' value at {}!", i),
            },
            "vt" => match str_to_vec2(&line[3..]) {
                Some(vec2) => uvs.push(vec2),
                None => panic!("Malformed 'vt' value at {}!", i),
            },
            "f " => match str_to_triangle_description(&line[2..]) {
                Some(td) => triangle_descriptions.push(td),
                None => panic!("Malformed 'f' value at {}!", i),
            },
            _ => {}
        }
    }

    let triangles: Vec<Triangle3> = triangle_descriptions
        .iter()
        .map(|&td| {
            let v1: Vertex3 = Vertex3 {
                pos: positions[(td.0).0],
                uv: uvs[(td.0).1],
                normal: normals[(td.0).2],
            };
            let v2: Vertex3 = Vertex3 {
                pos: positions[(td.1).0],
                uv: uvs[(td.1).1],
                normal: normals[(td.1).2],
            };
            let v3: Vertex3 = Vertex3 {
                pos: positions[(td.2).0],
                uv: uvs[(td.2).1],
                normal: normals[(td.2).2],
            };
            (v1, v2, v3)
        })
        .collect();

    let mut obj = Object::new();
    obj.triangles = triangles;
    obj
}
