use rand::rngs::ThreadRng;
use rand::Rng;
use futures_util::stream;
use tonic::Request;

use grpc_server::route_guide_rpc::route_guide;
use route_guide::{
    route_guide_client::RouteGuideClient,
    Point
};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let mut client = RouteGuideClient::connect(
        "http://[::1]:10000"
    ).await?;

    let mut rng = rand::thread_rng();
    let point_count: i32 = rng.gen_range(2..100);

    let mut points = vec![];
    for _ in 0..=point_count {
        points.push(random_point(&mut rng));
    }

    tracing::info!("Traversing {} points.", points.len());

    // Research about Streaming, 
    // TODO: Modify to Streaming with asynchronously
    let request = 
        Request::new(stream::iter(points));

    match client.record_route(request).await {
        Ok(response) => println!("SUMMARY: {:?}", response.into_inner()),
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(())
}

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.gen_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.gen_range(0..360) - 180) * 10_000_000;
    Point {
        latitude,
        longitude,
    }
}