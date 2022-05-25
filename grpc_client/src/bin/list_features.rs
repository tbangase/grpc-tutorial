use tonic::Request;
use grpc_server::route_guide_rpc::route_guide;
use route_guide::{
    route_guide_client::RouteGuideClient,
    Rectangle, Point
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let mut client = RouteGuideClient::connect(
        "http://[::1]:10000"
    ).await?;
    tracing::info!("Now listing Features...!");
    let rectangle = Rectangle {
        lo: Some(Point {
            latitude: 400000000,
            longitude: -750000000,
        }),
        hi: Some(Point {
            latitude: 420000000,
            longitude: -730000000,
        }),
    };

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        tracing::info!("NOTE = {feature:?}")
    }

    Ok(())
}