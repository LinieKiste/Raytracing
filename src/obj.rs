use std::{fs::read_to_string, num::ParseFloatError};
use crate::{triangle::Triangle, vec3::Point3};
use anyhow::{Result, bail};

#[derive(Debug, PartialEq)]
struct Face {
    indices: [i32; 3],
}

pub struct Obj {
    faces: Vec<Face>,
    vertices: Vec<Point3>,
    pub triangles: Vec<Triangle>,
}

impl Obj {
    pub fn new(path: &str) -> Result<Self> {
        let mut faces = vec!();
        let mut vertices = vec!();
        for line in read_to_string(path).unwrap().lines(){
            // Ignore comments
            if line.starts_with('#') { continue }
            // read vertex
            if line.starts_with("v ") {
                let [Ok(x), Ok(y), Ok(z), ..] = line[2..].split(' ')
                    .map(|x| x.parse()).collect::<Vec<Result<f32, ParseFloatError>>>()[..] else {
                        bail!("Failed to parse vertex!")
                    };
                let vertex = Point3::new(x, y, z);
                vertices.push(vertex);
            }
            if line.starts_with("f ") {
                let [Some(x), Some(y), Some(z), ..] = line[2..].split(' ')
                    .map(|x| {
                        x.split('/').next().and_then(|x| x.parse::<i32>()
                                                     .ok().map(|x| x-1))
                    })
                    .collect::<Vec<Option<i32>>>()[..] else {
                        bail!("Failed to parse face!")
                    };
                let face = Face{ indices: [x, y, z] };
                faces.push(face);
            }
        }

        let mut triangles = vec!();
        for face in &faces {
            let [i0, i1, i2] = face.indices;
            let v0 = vertices[i0 as usize];
            let v1 = vertices[i1 as usize];
            let v2 = vertices[i2 as usize];

            let triangle = Triangle::new(v0, v1, v2, None);
            triangles.push(triangle);
        }

        Ok(Obj{ faces, vertices, triangles })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use anyhow::Result;
    use tempdir::TempDir;
    use super::*;

    #[test]
    fn create_obj() -> Result<()> {
        let tmp_dir = TempDir::new("testing")?;
        let file_path = tmp_dir.path().join("cube.obj");
        let mut tmp_file = File::create(file_path.clone())?;
        write!(tmp_file, "{}", CUBE_DATA)?;

        let cube = Obj::new(file_path.to_str().unwrap())?;

        assert_eq!(cube.vertices.len(), 8, "Cube must consist of 8 vertices");
        assert_eq!(cube.faces.len(), 12, "Cube must consist of 12 triangles");

        let last_face = Face{ indices: [4, 0, 1]};
        assert_eq!(cube.faces.last(), Some(last_face).as_ref());
        let last_vertex = Point3::new(-1., -1., 1.);
        assert_eq!(cube.vertices.last(), Some(last_vertex).as_ref());

        drop(tmp_dir);
        Ok(())
    }

    const CUBE_DATA: &str = "# Blender 3.6.5
# www.blender.org
o Cube
v 1.000000 1.000000 -1.000000
v 1.000000 -1.000000 -1.000000
v 1.000000 1.000000 1.000000
v 1.000000 -1.000000 1.000000
v -1.000000 1.000000 -1.000000
v -1.000000 -1.000000 -1.000000
v -1.000000 1.000000 1.000000
v -1.000000 -1.000000 1.000000
vn -0.0000 1.0000 -0.0000
vn -0.0000 -0.0000 1.0000
vn -1.0000 -0.0000 -0.0000
vn -0.0000 -1.0000 -0.0000
vn 1.0000 -0.0000 -0.0000
vn -0.0000 -0.0000 -1.0000
vt 0.875000 0.500000
vt 0.625000 0.750000
vt 0.625000 0.500000
vt 0.375000 1.000000
vt 0.375000 0.750000
vt 0.625000 0.000000
vt 0.375000 0.250000
vt 0.375000 0.000000
vt 0.375000 0.500000
vt 0.125000 0.750000
vt 0.125000 0.500000
vt 0.625000 0.250000
vt 0.875000 0.750000
vt 0.625000 1.000000
s 0
f 5/1/1 3/2/1 1/3/1
f 3/2/2 8/4/2 4/5/2
f 7/6/3 6/7/3 8/8/3
f 2/9/4 8/10/4 6/11/4
f 1/3/5 4/5/5 2/9/5
f 5/12/6 2/9/6 6/7/6
f 5/1/1 7/13/1 3/2/1
f 3/2/2 7/14/2 8/4/2
f 7/6/3 5/12/3 6/7/3
f 2/9/4 4/5/4 8/10/4
f 1/3/5 3/2/5 4/5/5
f 5/12/6 1/3/6 2/9/6
";
}
