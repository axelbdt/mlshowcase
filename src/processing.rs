use crate::generation;
use crate::generation::Data;

use rand::prelude::*;

fn init_centroids(data: Data, k: u8) -> Data {
    let mut rng = StdRng::seed_from_u64(13);
    data.choose_multiple(&mut rng, k as usize).cloned().collect()
}

fn assign_cluster(data: Data, centroids: Data) {

}

pub fn kmeans() -> Data {
   let data = generation::generate_dataset(42);
   init_centroids(data, 3)
}


