use crate::generation::{Data, Sample};

use std::collections::HashMap;

use rand::prelude::*;

fn init_centroids(data: &Data, k: usize) -> Data {
    let mut rng = StdRng::seed_from_u64(13);
    data.choose_multiple(&mut rng, k as usize)
        .cloned()
        .collect()
}

fn update_centroids(
    data: &Data,
    centroids: &Data,
    assignments: &Vec<usize>,
) -> (Data, Vec<usize>, bool) {
    if centroids.len() == 0 {
        return (Data::new(), Vec::new(), false);
    }
    let mut acc_centroids: HashMap<usize, (usize, Sample)> = centroids
        .iter()
        .enumerate()
        .map(|(i, _)| (i, (0, Sample::new(0., 0.))))
        .collect();
    let mut new_assignments: Vec<usize> = Vec::new();
    let mut updated = false;
    for (pos, sample) in data.iter().enumerate() {
        updated = false;
        let pos_closest_centroid = centroids
            .iter()
            .enumerate()
            .fold((0, f32::INFINITY), |(i_min, dist_min), (i, centroid)| {
                let dist = (sample - centroid).norm();
                if dist < dist_min {
                    (i, dist)
                } else {
                    (i_min, dist_min)
                }
            })
            .0;
        if assignments[pos] != pos_closest_centroid {
            updated = true;
        }
        new_assignments.push(pos_closest_centroid);

        let (count, sum) = acc_centroids
            .get_mut(&pos_closest_centroid)
            .expect("Closest centroid should be initialized");
        *count += 1;
        *sum += sample;
    }

    (
        acc_centroids
            .iter()
            .map(|(_, &(count, sum))| sum / (count as f32))
            .collect(),
        new_assignments,
        updated,
    )
}

pub fn kmeans(k: usize, data: &Data) -> Data {
    if k == 0 {
        return Data::new();
    }
    let mut centroids = init_centroids(&data, k);
    let mut assignments = vec![centroids.len(); data.len()];
    for _ in 0..100 {
        let (new_centroids, new_assignments, updated) =
            update_centroids(&data, &centroids, &assignments);
        centroids = new_centroids;
        assignments = new_assignments;
        if updated == false {
            break;
        }
    }
    centroids
}
