extern crate nalgebra as na;

use na::base::{Matrix2, Vector2};
use rand::prelude::*;
use rand_distr::{Distribution, StandardNormal};


pub fn generate_dataset(seed: u64) -> Vec<[f32;2]> {
    let mut rng = StdRng::seed_from_u64(seed);

    let sigma: Matrix2<f32> = Matrix2::new(2.0, 1.0 ,1.0, 3.0);
    let mu: Vector2<f32> = Vector2::new(6.0, 5.0);
    
    generate_bivariate_normal_sample(mu, sigma, 100, &mut rng)
}

pub fn generate_bivariate_normal_sample(
    mu: Vector2<f32>,
    sigma: Matrix2<f32>,
    nb_points: u64,
    rng: &mut StdRng
    ) -> Vec<[f32;2]> {
    let sqrt_sigma = sigma.cholesky().unwrap().unpack();

    (0..nb_points).map(|_| {
        let x0: f32 = rng.sample(StandardNormal);
        let y0: f32 = rng.sample(StandardNormal);
        let v0 = Vector2::new(x0, y0);
        let v = sqrt_sigma * v0 + mu;
        [v[0], v[1]]
    }).collect()
}
