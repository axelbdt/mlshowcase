use na::Vector2;
use serde::Serialize;

pub type Sample = Vector2<f32>;
pub type Data = Vec<Sample>;

#[derive(Serialize)]
pub struct SampleJson {
    x: f32,
    y: f32,
}

impl From<Sample> for SampleJson {
    fn from(s: Sample) -> Self {
        SampleJson { x: s[0], y: s[1] }
    }
}

#[derive(Serialize)]
pub struct DataJson {
    data: Vec<SampleJson>,
}

impl From<Data> for DataJson {
    fn from(data: Data) -> Self {
        let data = data.iter().map(|s| (*s).into()).collect();
        DataJson { data }
    }
}
