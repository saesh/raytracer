use rand::Rng;

pub fn random_double() -> f32 {
    random_double_bounded(0.0, 1.0)
}

pub fn random_double_bounded(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min, max);
}