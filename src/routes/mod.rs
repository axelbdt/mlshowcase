use crate::data::{Data, Sample};
use serde::Serialize;

pub mod data;
pub mod datasets;
pub mod kmeans;

#[derive(Serialize)]
pub struct APIResult {
    data: Vec<Point>,
}

impl APIResult {
    fn new(data: Data) -> Self {
        let data = data.iter().map(|s| (*s).into()).collect();
        APIResult { data }
    }
}

#[derive(Serialize)]
pub struct Point {
    x: f32,
    y: f32,
}

impl From<Sample> for Point {
    fn from(s: Sample) -> Self {
        Point { x: s[0], y: s[1] }
    }
}
