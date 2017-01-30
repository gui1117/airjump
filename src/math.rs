#[inline]
pub fn angle(p: [f64; 2]) -> f64 {
    p[1].atan2(p[0])
}
#[inline]
pub fn norm(p: [f64; 2]) -> f64 {
    (p[0].powi(2) + p[1].powi(2)).sqrt()
}
#[inline]
pub fn mul(k: f64, p: [f64; 2]) -> [f64; 2] {
    [p[0]*k, p[1]*k]
}
#[inline]
pub fn normalize(p: [f64; 2]) -> [f64; 2] {
    mul(1./norm(p), p)
}
#[inline]
pub fn add(p1: [f64; 2], p2: [f64; 2]) -> [f64; 2] {
    [p1[0]+p2[0], p1[1]+p2[1]]
}
#[inline]
pub fn sub(p1: [f64; 2], p2: [f64; 2]) -> [f64; 2] {
    [p1[0]-p2[0], p1[1]-p2[1]]
}
#[inline]
pub fn into_polar(p: [f64; 2]) -> [f64; 2] {
    [norm(p), angle(p)]
}
#[inline]
pub fn from_polar(p: [f64; 2]) -> [f64; 2] {
    [p[0]*p[1].cos(), p[0]*p[1].sin()]
}
