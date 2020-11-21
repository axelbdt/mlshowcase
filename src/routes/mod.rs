pub mod data;
pub mod datasets;
pub mod kmeans;

/*
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
*/
