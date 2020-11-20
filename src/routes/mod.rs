use crate::data::{Data, Sample};
use crate::error::Error;
use serde::Serialize;

pub mod data;
pub mod kmeans;

#[derive(Serialize)]
pub enum APIResult {
    Data(Vec<Point>),
    Error(Error),
}

impl From<Error> for APIResult {
    fn from(e: Error) -> Self {
        APIResult::Error(e)
    }
}

impl From<Data> for APIResult {
    fn from(data: Data) -> Self {
        let points = data.into_iter().map(|s| s.into()).collect();
        APIResult::Data(points)
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
