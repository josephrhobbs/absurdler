// util::math

pub fn i(p: f64) -> f64 {
    if p == 0.0 {
        // Otherwise this will break as 0.ln() is undefined
        return 0.0;
    }
    -p.ln()
}

pub fn entropy(information: Vec<f64>) -> f64 {
    let mut total: f64 = 0.0;
    for p in information.iter() {
        total += p*i(*p);
    }
    total
}