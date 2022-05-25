use anyhow::Result;
use tonic::transport::Server;
use grpc_server::{
    data,
    route_guide_rpc::RouteGuideService,
    route_guide_rpc::route_guide::route_guide_server::RouteGuideServer
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();

    let addr = "[::1]:10000".parse().unwrap();

    let route_guide = RouteGuideService::new(
        Arc::new(data::load())
    );

    let svc = RouteGuideServer::new(route_guide);

    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
