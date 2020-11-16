use crate::generation;
use crate::generation::{Data, Sample};

use std::collections::HashMap;

use rand::prelude::*;

fn init_centroids(data: &Data, k: usize) -> Data {
    let mut rng = StdRng::seed_from_u64(13);
    data.choose_multiple(&mut rng, k as usize)
        .cloned()
        .collect()
}

fn update_centroids(data: &Data, centroids: &Data) -> Option<Vec<Sample>> {
    if centroids.len() == 0 {
        return None;
    }
    let assignments: HashMap<usize, (usize, Sample)> = centroids
        .iter()
        .enumerate()
        .map(|(i, _)| (i, (0, Sample::new(0., 0.))))
        .collect();
    let assignments = data.iter().fold(assignments, |mut assignments, &p| {
        let closest_centroid = centroids
            .iter()
            .enumerate()
            .fold((0, f32::INFINITY), |(i_min, dist_min), (i, c)| {
                let dist = (p - c).norm();
                if dist < dist_min {
                    (i, dist)
                } else {
                    (i_min, dist_min)
                }
            })
            .0;
        let (count, sum) = assignments.get_mut(&closest_centroid).expect("Closest centroid should be initialized");
        *count += 1;
        *sum += p;
        assignments
    });

    Some(
        assignments
            .iter()
            .map(|(_, &(count, sum))| sum / (count as f32))
            .collect(),
    )
}

pub fn kmeans(k: usize) -> Data {
    let data = generation::generate_dataset(42);
    let centroids = init_centroids(&data, k);
    let centroids = update_centroids(&data, &centroids).expect("k should be 3 for now");
    centroids
}
