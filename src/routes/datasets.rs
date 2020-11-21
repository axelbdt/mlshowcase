use rocket_contrib::json::JsonValue;

/// Get list of ids of available datasets (placeholder value for now)
#[get("/datasets/ids")]
pub fn get_ids() -> JsonValue {
    json!({
        "datasets": [
            {"id": 1},
            {"id": 2}]
    })
}
