// util::math

pub fn i(p: f64) -> f64 {
    -p.ln()
}

pub fn entropy(information: Vec<f64>) -> f64 {
    let mut total: f64 = 0.0;
    for p in information.iter() {
        total += i(*p);
    }
    total
}