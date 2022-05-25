use grpc_server::route_guide_rpc::route_guide;
use route_guide::{
    route_guide_client::RouteGuideClient,
    Point,
};
use tonic::Request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let mut client = RouteGuideClient::connect(
        "http://[::1]:10000"
    ).await?;
    tracing::info!("gRPC client Working...!");
    tracing::info!("Calling get feature...!");
    let response = client.get_feature(
        Request::new(Point {
            latitude: 409146138,
            longitude: -746188906,
        })
    ).await?;
    tracing::info!("\nResponse: {response:?}");

    Ok(())
}