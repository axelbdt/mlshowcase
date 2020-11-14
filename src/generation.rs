use rand::prelude::*;
use rand_distr::{Distribution, StandardNormal};

fn cholesky_sqrt(matrix: [[f32;2];2]) -> [[f32;2];2] {
    
    // Cholesky decomposition
    let a = matrix[0][0].sqrt();
    let b = matrix[0][1] / a;
    let c = (matrix[1][1]).sqrt() - b * b;

    [[a, 0.], [b, c]]
}

pub fn generate_dataset(seed: u64) -> Vec<[f32;2]> {
    let mut rng = StdRng::seed_from_u64(seed);

    let sigma: [[f32; 2]; 2] = [[2.0, 1.0], [1.0, 3.0]];
    let mu: [f32; 2] = [6.0, 5.0];
    
    generate_bivariate_normal_sample(mu, sigma, 100, &mut rng)
}

pub fn generate_bivariate_normal_sample(mu: [f32;2], sigma: [[f32;2];2], nb_points: u64, rng: &mut StdRng) -> Vec<[f32;2]> {
    let sqrt_sigma = cholesky_sqrt(sigma);

    (0..nb_points).map(|_| {
        let x0: f32 = rng.sample(StandardNormal);
        let y0: f32 = rng.sample(StandardNormal);
        let x = x0 * sqrt_sigma[0][0] + mu[0];
        let y = x0 * sqrt_sigma[1][0] + y0 * sqrt_sigma[1][1] + mu[1];
        [x, y]
    }).collect()
}
