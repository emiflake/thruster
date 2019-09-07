use crate::algebra::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum ParseError {
    FileReadError,
    LineReadError,
    StrParseError,
    MalformedVertex(usize),
}

/// Just a bunch of triangles, basically.
pub struct Object {
    pub tris: Vec<(Vertex, Vertex, Vertex)>,
}

impl Default for Object {
    fn default() -> Self {
        Self { tris: Vec::new() }
    }
}

// TODO: remove
pub fn str_to_float(s: &str) -> Result<f64, ParseError> {
    s.parse::<f64>().map_err(|_| ParseError::StrParseError)
}

/// Parses a .obj file into an `Object`.
pub fn parse(path: String) -> Result<Object, ParseError> {
    let file = File::open(path).map_err(|_| ParseError::FileReadError)?;
    let reader = BufReader::new(file);

    let mut positions: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut uvs: Vec<Vec2> = Vec::new();
    let mut obj = Object::default();

    for (line_no, line) in reader.lines().enumerate() {
        let unwrapped_line = line.map_err(|_| ParseError::LineReadError)?;
        let line_bits: Vec<&str> = unwrapped_line.split_whitespace().collect();

        if line_bits.is_empty() {
            // Empty line; skip
            continue;
        }

        match line_bits[0usize] {
            "v" => {
                positions.push(Vec3::new(
                    str_to_float(line_bits[1usize])?,
                    str_to_float(line_bits[2usize])?,
                    str_to_float(line_bits[3usize])?,
                ));
            }
            "vn" => {
                normals.push(Vec3::new(
                    str_to_float(line_bits[1usize])?,
                    str_to_float(line_bits[2usize])?,
                    str_to_float(line_bits[3usize])?,
                ));
            }
            "vt" => {
                uvs.push(Vec2::new(
                    str_to_float(line_bits[1usize])?,
                    str_to_float(line_bits[2usize])?,
                ));
            }

            "f" => {
                let mut vertices = Vec::new();
                for vfinder in line_bits[1usize..].iter() {
                    let data: Vec<&str> = vfinder.split('/').collect();
                    if data.len() == 3 {
                        if let [pos, uv, norm] = data.as_slice() {
                            let uv = match uv.parse::<usize>() {
                                Ok(uvi) => uvs[uvi - 1],
                                Err(_) => Vec2::new(0.0, 0.0),
                            };
                            let vertex = Vertex {
                                origin: positions[pos
                                    .parse::<usize>()
                                    .map_err(|_| ParseError::MalformedVertex(line_no))?
                                    - 1],
                                uv,
                                normal: normals[norm
                                    .parse::<usize>()
                                    .map_err(|_| ParseError::MalformedVertex(line_no))?
                                    - 1],
                            };
                            vertices.push(vertex);
                        }
                    } else {
                        return Err(ParseError::MalformedVertex(line_no));
                    }
                }
                if vertices.len() != 3 {
                    unreachable!();
                }
                obj.tris.push((vertices[0], vertices[1], vertices[2]));
            }
            _ => {}
        }
    }

    Ok(obj)
}
