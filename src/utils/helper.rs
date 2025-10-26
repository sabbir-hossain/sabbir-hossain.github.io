use rand::Rng;

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn array_index(len: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..len)
}

#[cfg(target_arch = "wasm32")]
pub fn now() -> f64 {
    use js_sys::Date;
    let millis = Date::now();
    (millis / 1000.0) as f64
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now() -> f64 {
    use chrono::Utc;
    let now = Utc::now();
    now.timestamp_millis() as f64 / 1000.0 
}

#[cfg(target_arch = "wasm32")]
pub fn now_str() -> chrono::DateTime<chrono::Utc> {
    use js_sys::Date;
    let millis = Date::new_0().get_time();
    chrono::DateTime::from_timestamp((millis / 1000.0) as i64, 0).unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now_str() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

pub fn get_acceleration(force: f32, weight: f32) -> f32 {
    let acceleration = force / weight;
    acceleration
}
