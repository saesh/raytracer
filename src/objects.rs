pub mod sphere;
pub mod moving_sphere;
pub mod triangle;

use std::cmp::Ordering;

use crate::structures::ray::Ray;
use crate::materials::HitRecord;
use crate::structures::vec3::Vec3;
use crate::random::random_int_closed;

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aaab>;
}

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit_record: Option<HitRecord> = None;

        for object in self.iter() {
            match object.hit(ray, t_min, closest_so_far) {
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    closest_hit_record = Some(hit_record);
                },
                None => {}
            }
        }
        
        closest_hit_record
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aaab> {
        if self.len() == 0 {
            return None;
        }

        let mut output_box: Aaab = Aaab { min: Vec3::ZERO, max: Vec3::ZERO };
        let mut first_box = true;
        
        for object in self.iter() {
            match object.bounding_box(t0, t1) {
                Some(temp_box) => {
                    if first_box {
                        output_box = temp_box;
                    } else {
                        output_box = surrounding_box(output_box, temp_box);
                        first_box = false;
                    }
                },
                None => {
                    return None;
                },
            }
        }

        Some(output_box)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Aaab {
    min: Vec3,
    max: Vec3,
}

impl Aaab {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0..3 {

            let inv_d = 1.0 / ray.direction.get(a);
            let mut t0 = (self.min.get(a) - ray.origin.get(a)) * inv_d;
            let mut t1 = (self.max.get(a) - ray.origin.get(a)) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            
            if t_max <= t_min {
                return false;
            }
        }
        
        return true;
    }
}

pub fn surrounding_box(box0: Aaab, box1: Aaab) -> Aaab {
    let small = Vec3 {
        x: box0.min.x.min(box1.min.x),
        y: box0.min.y.min(box1.min.y),
        z: box0.min.z.min(box1.min.z),
    };

    let big = Vec3 {
        x: box0.max.x.max(box1.max.x),
        y: box0.max.y.max(box1.max.y),
        z: box0.max.z.max(box1.max.z),
    };

    Aaab {
        min: small,
        max: big,
    }
}

pub struct BvhTree {
    nodes: Vec<BvhNode>,
    root: NodeId,
}

struct BvhNode {
    left: Option<NodeId>,
    right: Option<NodeId>,
    aabb: Option<Aaab>,
    hitable: Option<Box<dyn Hitable>>,
}

#[derive(Copy, Clone, Debug)]
struct NodeId {
    index: usize,
}

impl BvhTree {
    pub fn new(objects: Vec<Box<dyn Hitable>>, time0: f32, time1: f32) -> impl Hitable + 'static {
        let mut tree = BvhTree { nodes: Vec::new(), root: NodeId { index: 0} };
    
        tree.root = tree.build(&objects, time0, time1);

        tree
    }

    fn build(&mut self, mut objects: &Vec<Box<dyn Hitable>>, time0: f32, time1: f32) -> NodeId {
        let axis = random_int_closed(0 , 2);
        let object_span = objects.len();

        let left;
        let right;

        if object_span == 1 {
            left = self.new_leaf(objects[0], time0, time1);
            right = self.new_leaf(objects[0], time0, time1);
        } else if object_span == 2 {
            let ordering = compare(objects.get(0).unwrap(), objects.get(1).unwrap(), axis);
            if ordering == Ordering::Less {
                left = self.new_leaf(objects[0], time0, time1);
                right = self.new_leaf(objects[1], time0, time1);
            } else {
                left = self.new_leaf(objects[0], time0, time1);
                right = self.new_leaf(objects[1], time0, time1);
            }
        } else {
            match axis {
                0 => objects.sort_by(|a, b| box_x_compare(a, b)),
                1 => objects.sort_by(|a, b| box_y_compare(a, b)),
                2 => objects.sort_by(|a, b| box_z_compare(a, b)),
                _ => eprintln!("Unexpected axis"),
            }

            let mid = object_span / 2;    
            let lt = objects[0..mid];
            let (left_hitables, right_hitables) = objects.split_at(mid);
            left = self.build(&objects[0..mid], time0, time1);
            right = self.build(right_hitables, time0, time1);
        }

        let box_left = self.nodes[left.index].aabb;
        let box_right = self.nodes[right.index].aabb;
         
        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in bvh_node constructor.");
        }

        self.new_node(surrounding_box(box_left.unwrap(), box_right.unwrap()), Some(left), Some(right))
    }

    fn new_leaf(&mut self, hitable: Box<dyn Hitable>, time0: f32, time1: f32) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left: None,
            right: None,
            aabb: hitable.bounding_box(time0, time1),
            hitable: Some(hitable),
        });

        NodeId { index: next_index }
    }

    fn new_node(&mut self, aabb: Aaab, left: Option<NodeId>, right: Option<NodeId>) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(BvhNode {
            left,
            right,
            aabb: Some(aabb),
            hitable: None,
        });

        NodeId { index: next_index }
    }

    fn hit(&self, id: NodeId, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let node = &self.nodes[id.index];

        if node.aabb.is_some() && !node.aabb.unwrap().hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.nodes[node.left.unwrap().index].hitable.unwrap().hit(ray, t_min, t_max);
        let right_hit = self.nodes[node.right.unwrap().index].hitable.unwrap().hit(ray, t_min, if left_hit.is_some() { left_hit.unwrap().t } else { t_max });

        if left_hit.is_some() && right_hit.is_some() {
            let left_t = left_hit.unwrap().t;
            let right_t = right_hit.unwrap().t;
            if left_t <= right_t {
                return left_hit;
            } else {
                return right_hit;
            }
        }

        if left_hit.is_some() && !right_hit.is_some() {
            return left_hit;
        }

        if !left_hit.is_some() && right_hit.is_some() {
            return right_hit;
        }

        None
    }
}

impl Hitable for BvhTree {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit(self.root, ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aaab> {
        self.nodes[self.root.index].aabb
    }
}

fn compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>, axis: u32) -> Ordering {
    match axis {
        0 => box_x_compare(a, b),
        1 => box_y_compare(a, b),
        2 => box_z_compare(a, b),
        _ => panic!("Unexpected axis"),
    }
}

fn box_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>, axis: u32) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in bvh_node constructor.");
    }

    if let Some(cmp) = box_a.unwrap().min.get(axis as u8).partial_cmp(&box_b.unwrap().min.get(axis as u8)) {
        return cmp;
    } else {
        panic!("Comparison failed")
    };
}

fn box_x_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    return box_compare(a, b, 0);
}

fn box_y_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    return box_compare(a, b, 1);
}

fn box_z_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    return box_compare(a, b, 2);
}