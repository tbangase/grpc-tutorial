use serde::Deserialize;
use std::fs::File;
use crate::route_guide_rpc::route_guide;

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

#[allow(dead_code)]
pub fn load() -> Vec<route_guide::Feature> {
    let file = File::open("data/route_guide_db.json").expect("failed to open data file");

    let decoded: Vec<Feature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");

    decoded
        .into_iter()
        .map(|feature| route_guide::Feature {
            name: feature.name,
            location: Some(route_guide::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect()
}