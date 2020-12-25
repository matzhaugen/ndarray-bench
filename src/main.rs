extern crate blas;
#[macro_use(azip)]
extern crate ndarray;
extern crate openblas_src;
use ndarray::Axis;
use ndarray::{Array2, ArrayView2};
use rand;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;

pub fn resample<'a>(data: &'a ArrayView2<f64>, n: &usize) -> Array2<f64> {
    // let resampled = data.sample_axis(Axis(0), *n, SamplingStrategy::WithReplacement); // for v ndarray = 0.14
    let between = Uniform::from(0..*n);
    let mut rng = rand::thread_rng();
    let mut indeces = Vec::with_capacity(*n);
    for _i in 0..*n {
        indeces.push(between.sample(&mut rng));
    }

    let resampled = data.select(Axis(1), &indeces[..]);

    resampled
}

fn main() {
    let p: usize = 1000;
    let mat1 = Array2::<f64>::ones((p, p));
    let mut mat2 = mat1.clone();
    let mut start = Instant::now();
    azip!((m1 in &mat1, m2 in &mut mat2) *m2 = -m1);
    let mut duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    start = Instant::now();
    let _mat3 = mat1.dot(&mat2);
    duration = start.elapsed();
    println!("Time elapsed in dot product is: {:?}", duration);
    start = Instant::now();
    let _mat3 = resample(&mat1.view(), &p).dot(&mat2);
    duration = start.elapsed();
    println!("Time elapsed in dot product is: {:?}", duration);
}
