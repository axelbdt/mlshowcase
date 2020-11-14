extern crate nalgebra as na;

use crate::routes::Point;

use na::{Dynamic, Matrix, Matrix2, U2, Vector2, VecStorage};
use rand::prelude::*;
use rand_distr::StandardNormal;

// Half-dynamically sized and dynamically allocated matrix with
// two rows using 32-bit floats.
type Matrix2xXf32 = Matrix<f32, U2, Dynamic, VecStorage<f32, U2, Dynamic>>;

pub fn generate_dataset(seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);

    let sigma: Matrix2<f32> = Matrix2::new(2.0, 1.0 ,1.0, 3.0);
    let mu: Vector2<f32> = Vector2::new(6.0, 5.0);
    
    let dataset = generate_bivariate_normal_sample(mu, sigma, 100, &mut rng);

    dataset.column_iter().map( |c| Point {x: c[0], y: c[1]}).collect()
}

pub fn generate_bivariate_normal_sample(
    mu: Vector2<f32>,
    sigma: Matrix2<f32>,
    nb_points: u64,
    rng: &mut StdRng
    ) -> Matrix2xXf32 {
    let sqrt_sigma = sigma.cholesky().unwrap().l();

    let sample = Matrix2xXf32::from_distribution(nb_points as usize, &StandardNormal, rng);
    let data_zero_mean = sqrt_sigma * sample;

    let data: Vec<Vector2<f32>> = data_zero_mean.column_iter().map(|c| c + mu).collect();
    Matrix2xXf32::from_columns(&data)
}
