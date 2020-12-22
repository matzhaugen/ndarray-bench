#[macro_use(azip)]
extern crate ndarray;
use ndarray::Array2;
use std::time::Instant;
fn main() {
    let p: usize = 100;
    let mat1 = Array2::<f32>::ones((p, p));
    let mut mat2 = mat1.clone();
    let start = Instant::now();
    azip!((m1 in &mat1, m2 in &mut mat2) *m2 = -m1);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
