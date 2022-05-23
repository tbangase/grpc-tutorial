pub mod route_guide {
    tonic::include_proto!("route_guide");
}

use route_guide::route_guide_server::RouteGuide;
use route_guide::{Feature, Point, Rectangle, RouteNote, RouteSummary};

use futures_core::Stream;
use std::{
    pin::Pin,
    sync::Arc,
};
use std::hash::{Hash, Hasher};
// use tokio::sync::mpsc;
use tonic::{Request, Response, Status, Streaming};
use tokio_stream::wrappers::ReceiverStream;

use getset::{Getters, Setters};
use derive_new::new;


impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.latitude.hash(state);
        self.longitude.hash(state);
    }
}

impl Eq for Point {}

#[derive(new, Debug, Getters, Setters)]
pub struct RouteGuideService {
    #[getset(get = "pub", set = "pub")]
    features: Arc<Vec<Feature>>
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(
        &self, 
        request: Request<Point>
    ) -> Result<Response<Feature>, Status> {
        let target_feature = self.features().iter().find(|feature| 
            feature.location.as_ref() == Some(request.get_ref())
        );

        if let Some(feature) = target_feature {
            Ok(Response::new(feature.clone()))
        } else {
            Ok(Response::new(Feature::default()))
        }
    }


    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        _request: Request<Rectangle>
    ) -> Result<Response<Self::ListFeaturesStream> ,tonic::Status> {
        unimplemented!()
    }


    async fn record_route(
        &self,
        _request: Request<Streaming<Point>>
    ) -> Result<Response<RouteSummary>, Status> {
        unimplemented!()
    }


    // Moveした時にChatStreamのアドレスが変わらないようにPinを使う
    // なんで??
    type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static >>;

    async fn route_chat(
        &self,
        _request: Request<Streaming<RouteNote>>
    ) ->  Result<Response<Self::RouteChatStream> ,Status> {
        unimplemented!()
    }

}