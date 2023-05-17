use tonic::{transport::Server, Request, Response, Status};

use plugin::plugin_server::{Plugin, PluginServer};
use plugin::{PluginRequest, PluginResponse};

use plugin::simplify_server::{Simplify, SimplifyServer};
use plugin::{SimplifyRequest, SimplifyResponse};

use plugin::postprocess_server::{Postprocess, PostprocessServer};
use plugin::{PostprocessRequest, PostprocessResponse};

mod plugin {
    tonic::include_proto!("cura.plugins.proto");
}

#[derive(Default)]
struct PluginServicer {}

#[tonic::async_trait]
impl Plugin for PluginServicer {
    async fn identify(
        &self,
        request: Request<PluginRequest>,
    ) -> Result<Response<PluginResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        Result::Ok(Response::new(PluginResponse {
            plugin_hash: "1234567890".to_string(),
            version: "0.0.1".to_string(),
        }))
    }
}

#[derive(Default)]
struct SimplifyServicer {}

#[tonic::async_trait]
impl Simplify for SimplifyServicer {
    async fn simplify(
        &self,
        request: Request<SimplifyRequest>,
    ) -> Result<Response<SimplifyResponse>, Status> {
        Result::Ok(Response::new(SimplifyResponse {
            polygons: request.into_inner().polygons,
        }))
    }
}

#[derive(Default)]
struct PostprocessServicer {}

#[tonic::async_trait]
impl Postprocess for PostprocessServicer {
    async fn postprocess(
        &self,
        request: Request<PostprocessRequest>,
    ) -> Result<Response<PostprocessResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        println!("gcode word: {}", request.into_inner().gcode_word);
        Result::Ok(Response::new(PostprocessResponse {
            gcode_word: "Hello World".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::]:5555".parse()?;
    let project_name = env!("CARGO_PKG_NAME");
    println!("{project_name} listening on {address}",);

    Server::builder()
        .add_service(PluginServer::new(PluginServicer::default()))
        .add_service(SimplifyServer::new(SimplifyServicer::default()))
        .add_service(PostprocessServer::new(PostprocessServicer::default()))
        .serve(address)
        .await?;

    Ok(())
}
