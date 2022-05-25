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
use tokio::sync::mpsc;
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

// point が rect の中に入っているかどうか
fn in_range(point: &Point, rect: &Rectangle) -> bool {
    use std::cmp;

    let lo = rect.lo.as_ref().unwrap();
    let hi = rect.hi.as_ref().unwrap();

    let left = cmp::min(lo.longitude, hi.longitude);
    let right = cmp::max(lo.longitude, hi.longitude);
    let top = cmp::max(lo.latitude, hi.latitude);
    let bottom = cmp::min(lo.latitude, hi.latitude);

    point.longitude >= left
        && point.longitude <= right
        && point.latitude >= bottom
        && point.latitude <= top
}

/// Calculates the distance between two points using the "haversine" formula.
/// This code was taken from http://www.movable-type.co.uk/scripts/latlong.html.
#[allow(dead_code)]
fn calc_distance(p1: &Point, p2: &Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0; // meters

    let lat1 = p1.latitude as f64 / CORD_FACTOR;
    let lat2 = p2.latitude as f64 / CORD_FACTOR;
    let lng1 = p1.longitude as f64 / CORD_FACTOR;
    let lng2 = p2.longitude as f64 / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin() * (delta_lat / 2f64).sin()
        + (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin();

    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

    (R * c) as i32
}

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
        request: Request<Rectangle>
    ) -> Result<Response<Self::ListFeaturesStream> ,tonic::Status> {
        let (tx, rx) = mpsc::channel(4);
        let features = self.features().clone();

        tokio::spawn(async move {
            for feature in features.iter() {
                if in_range(feature.location.as_ref().unwrap(), request.get_ref()) {
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
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