use std::convert::TryFrom;
use std::sync::Arc;

use crate::objects::triangle::Triangle;
use crate::structures::color::Color;
use crate::materials::Metal;
use crate::structures::vec3::Vec3;

// TODO: load any file
// TODO: support more than one model
// TODO: improve material assignment
pub fn load_teapot() -> Option<Vec<Triangle>> {
    let path = "files/teapot.obj";

    let (models, _) = tobj::load_obj(&path, false).expect("Failed to load file");
    eprintln!("Loaded {}, found {} models", path, models.len());
    assert!(models.len() == 1);
    
    let mut triangles: Vec<Triangle> = Vec::new();

    for (_, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        assert!(mesh.positions.len() % 3 == 0);

        let mut next_face = 0;

        for f in 0..mesh.num_face_indices.len() {
            let end = next_face + mesh.num_face_indices[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
           
            let v1_idx = usize::try_from(face_indices[0].clone()).unwrap();
            let v2_idx = usize::try_from(face_indices[1].clone()).unwrap();
            let v3_idx = usize::try_from(face_indices[2].clone()).unwrap();
            
            let p1x = mesh.positions[3 * v1_idx];
            let p1y = mesh.positions[3 * v1_idx + 1];
            let p1z = mesh.positions[3 * v1_idx + 2];
            let v0 = Vec3::new(p1x, p1y, p1z);

            let p2x = mesh.positions[3 * v2_idx];
            let p2y = mesh.positions[3 * v2_idx + 1];
            let p2z = mesh.positions[3 * v2_idx + 2];
            let v1 = Vec3::new(p2x, p2y, p2z);

            let p3x = mesh.positions[3 * v3_idx];
            let p3y = mesh.positions[3 * v3_idx + 1];
            let p3z = mesh.positions[3 * v3_idx + 2];
            let v2 = Vec3::new(p3x, p3y, p3z);

            triangles.push(Triangle::new(v0, v1, v2, Arc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.5))));

            next_face = end;
        }
    }

    if triangles.len() == 0 {
        return None;
    }

    return Some(triangles);
}