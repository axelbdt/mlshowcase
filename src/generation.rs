use na::{Dynamic, Matrix, Matrix2, VecStorage, Vector2, U2};
use rand::prelude::*;
use rand_distr::StandardNormal;

// Half-dynamically sized and dynamically allocated matrix with
// two rows using 32-bit floats.
type Matrix2xXf32 = Matrix<f32, U2, Dynamic, VecStorage<f32, U2, Dynamic>>;

pub type Sample = Vector2<f32>;
pub type Data = Vec<Sample>;

pub fn generate_dataset(seed: u64) -> Data {
    let mut rng = StdRng::seed_from_u64(seed);

    let sigma: Matrix2<f32> = Matrix2::new(2.0, 1.0, 1.0, 3.0);
    let mu: Vector2<f32> = Vector2::new(6.0, 5.0);

    generate_bivariate_normal_sample(mu, sigma, 100, &mut rng)
}

pub fn generate_bivariate_normal_sample(
    mu: Vector2<f32>,
    sigma: Matrix2<f32>,
    nb_points: u64,
    rng: &mut StdRng,
) -> Data {
    let sqrt_sigma = sigma.cholesky().unwrap().l();

    let sample = Matrix2xXf32::from_distribution(nb_points as usize, &StandardNormal, rng);
    let data_zero_mean = sqrt_sigma * sample;

    data_zero_mean.column_iter().map(|c| c + mu).collect()
}
