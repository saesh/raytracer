use std::convert::TryFrom;
use std::sync::Arc;

use crate::objects::triangle::Triangle;
use crate::materials::Material;
use crate::structures::vec3::Vec3;
use crate::hitable::Hitable;

// TODO: support more than one model
pub fn load_file(path: &str, material: Arc<dyn Material>) -> Option<Vec<Box<dyn Hitable>>> {

    let (models, _) = tobj::load_obj(path, false).expect("Failed to load file");
    println!("Loaded {}, found {} models", path, models.len());
    
    assert!(models.len() == 1);
    
    let mut triangles: Vec<Box<dyn Hitable>> = Vec::new();

    for (_, m) in models.iter().enumerate() {
        
        let mesh = &m.mesh;

        assert!(mesh.positions.len() % 3 == 0);

        let mut next_face = 0;

        for f in 0..mesh.num_face_indices.len() {
            let end = next_face + mesh.num_face_indices[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
           
            let v1_idx = usize::try_from(*face_indices[0]).unwrap();
            let v2_idx = usize::try_from(*face_indices[1]).unwrap();
            let v3_idx = usize::try_from(*face_indices[2]).unwrap();
            
            let p1x = mesh.positions[3 * v1_idx];
            let p1y = mesh.positions[3 * v1_idx + 1];
            let p1z = mesh.positions[3 * v1_idx + 2];
            let p0 = Vec3::new(p1x, p1y, p1z);

            let p2x = mesh.positions[3 * v2_idx];
            let p2y = mesh.positions[3 * v2_idx + 1];
            let p2z = mesh.positions[3 * v2_idx + 2];
            let p1 = Vec3::new(p2x, p2y, p2z);

            let p3x = mesh.positions[3 * v3_idx];
            let p3y = mesh.positions[3 * v3_idx + 1];
            let p3z = mesh.positions[3 * v3_idx + 2];
            let p2 = Vec3::new(p3x, p3y, p3z);

            triangles.push(Box::new(Triangle::new(p0, p1, p2, material.clone())));

            next_face = end;
        }
    }

    if triangles.is_empty(){
        return None;
    }

    Some(triangles)
}