use tonic::{transport::Server, Request, Response, Status};

use plugin::plugin_server::{Plugin, PluginServer};
use plugin::{PluginRequest, PluginResponse};

use plugin::simplify_server::{Simplify, SimplifyServer};
use plugin::{SimplifyRequest, SimplifyResponse};

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
            version: env!("CARGO_PKG_VERSION").to_string(),
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:5555".parse()?;
    let project_name = env!("CARGO_PKG_NAME");
    println!("{project_name} listening on {address}",);

    Server::builder()
        .add_service(PluginServer::new(PluginServicer::default()))
        .add_service(SimplifyServer::new(SimplifyServicer::default()))
        .serve(address)
        .await?;

    Ok(())
}
