use std::{path::Path, fs::read_to_string, num::ParseFloatError, collections::HashMap};
use crate::{
    vec3::Point3,
    material::Material::{self}, color::Color, texture::Texture,
};
use anyhow::{Result, bail, anyhow};

#[derive(Debug, PartialEq)]
pub enum Face {
    Triangle(i32,i32,i32, Option<Material>),
    Quad(i32,i32,i32,i32, Option<Material>),
}

pub struct Obj {
    pub faces: Vec<Face>,
    pub vertices: Vec<Point3>,
}

impl Obj {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut faces = vec!();
        let mut vertices = vec!();
        let mut materials: HashMap<String, Material> = HashMap::new();
        let mut current_mat: Option<Material> = None;

        for line in read_to_string(&path)?.lines(){
            // Ignore comments
            if line.starts_with('#') { continue }
            // load materials
            if line.starts_with("mtllib ") {
                let filename = line.split_whitespace().nth(1)
                    .ok_or(anyhow!("No mtllib name found!"))?;
                let prefix = path.as_ref().parent()
                    .ok_or(anyhow!("No directory found"))?;
                materials = MtlLoader::load(&prefix.join(filename))?;
            }
            // use material
            if line.starts_with("usemtl ") {
                current_mat = Some(materials.get(&line[7..])
                    .ok_or(anyhow!("Invalid Material specified: {}", line))?.clone());
            }
            // read vertex
            if line.starts_with("v ") {
                let vertex: Point3 = parse_triplet(&line[2..])?;
                vertices.push(vertex);
            }
            if line.starts_with("f ") {
                let nums = line[2..].split(' ')
                    .map(|x| {
                        x.split('/').next().and_then(|x| x.parse::<i32>()
                                                     .ok().map(|x| x-1))
                    })
                    .collect::<Vec<Option<i32>>>();
                let face = match nums[..] {
                    [Some(x), Some(y), Some(z), Some(w), ..] => 
                        Face::Quad(x, y, z, w, current_mat.clone()),
                    [Some(x), Some(y), Some(z), ..] => 
                        Face::Triangle(x, y, z, current_mat.clone()),
                    _ => bail!("Face contains less than 2 vertices!"),
                };
                faces.push(face);
            }
        }

        Ok(Obj{ faces, vertices })
    }
}

#[allow(unused)]
struct MtlLoader {
    // ambient color
    ka: Color,
    // diffuse color
    kd: Color,
    // specular color
    ks: Color,
    // specular exponent
    ns: f32,
    // transparency, can also be "Tr"
    d: f32,
    // emissive color
    ke: Color,
}

impl MtlLoader {
    fn load<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Material>> {
        let mut map = HashMap::new();
        let mut loader = MtlLoader{..Default::default()};
        let mut name: Option<String> = None;

        for line in read_to_string(path)?.lines(){
            // Ignore comments
            if line.starts_with('#') { continue }
            // new material
            if line.starts_with("newmtl ") {
                if let Some(ref name) = name { map.insert(name.to_string(), loader.make_material()); }
                name = Some(line[7..].to_owned());
                loader = MtlLoader{..Default::default()};
            }
            if line.starts_with("Kd ") {
                let col: Color = parse_triplet(&line[3..])?;
                loader.kd = col;
            }
            if line.starts_with("Ns ") {
                let ns: f32 = 1.-(200./line[3..].parse::<f32>()?);
                loader.ns = ns;
            }
        }
        if let Some(ref name) = name { map.insert(name.to_string(), loader.make_material()); }

        Ok(map)
    }

    fn make_material(self) -> Material {
        self.kd.into()
    }
}

impl Default for MtlLoader {
    fn default() -> Self {
        Self {
            ka: Color::new(1., 1., 1.),
            kd: Color::new(0.5, 0.5, 0.5),
            ks: Color::new(0.5, 0.5, 0.5),
            ns: 10.0,
            d: 1.0,
            ke: Color::zeros(),
        }
    }
}

fn parse_triplet(from: &str) -> Result<Point3> {
    let [Ok(x), Ok(y), Ok(z), ..] = from.split(' ')
        .map(|x| x.parse()).collect::<Vec<Result<f32, _>>>()[..] else {
            bail!("Failed to parse vertex!")
        };
    Ok(Point3::new(x, y, z))
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

        let last_face = Face::Triangle(4, 0, 1, None);
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
