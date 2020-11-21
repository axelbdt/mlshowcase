use crate::error::Error;
use crate::models::data::Data;
use crate::models::dataset::{Dataset, Gaussian};

use na::{Dynamic, Matrix, VecStorage, U2};
use rand::prelude::*;
use rand_distr::StandardNormal;

// Half-dynamically sized and dynamically allocated matrix with
// two rows using 32-bit floats.
type Matrix2xXf32 = Matrix<f32, U2, Dynamic, VecStorage<f32, U2, Dynamic>>;

pub fn generate_dataset(dataset: &Dataset) -> Result<Data, Error> {
    //let dataset = mock_dataset();
    let mut rng = StdRng::seed_from_u64(dataset.seed as u64);

    let mut data = Vec::new();
    for gaussian in dataset.gmm.iter() {
        let dataset = generate_bivariate_normal_sample(&gaussian, 100, &mut rng)?;
        data.extend(dataset)
    }
    Ok(data)
}

pub fn generate_bivariate_normal_sample(
    gaussian: &Gaussian,
    nb_points: usize,
    rng: &mut StdRng,
) -> Result<Data, Error> {
    let sigma = gaussian.cov;
    let mu = gaussian.mean;

    let sqrt_sigma = sigma.cholesky().ok_or(Error::InvalidSigma)?.l();

    let sample = Matrix2xXf32::from_distribution(nb_points, &StandardNormal, rng);

    let data_zero_mean = sqrt_sigma * sample;

    Ok(data_zero_mean.column_iter().map(|c| c + mu).collect())
}
