use tonic::{transport::Server, Request, Response, Status};

use curaengine_grpc_defintions::slots::simplify::v0::{
    simplify_service_server::{SimplifyService, SimplifyServiceServer},
    SimplifyServiceModifyRequest, SimplifyServiceModifyResponse,
};

use curaengine_grpc_defintions::slots::postprocess::v0::{
    postprocess_service_server::{PostprocessService, PostprocessServiceServer},
    PostprocessServiceModifyRequest, PostprocessServiceModifyResponse,
};

#[derive(Default)]
struct SimplifyServicer {}

#[tonic::async_trait]
impl SimplifyService for SimplifyServicer {
    async fn modify(
        &self,
        request: Request<SimplifyServiceModifyRequest>,
    ) -> Result<Response<SimplifyServiceModifyResponse>, Status> {
        Result::Ok(Response::new(SimplifyServiceModifyResponse {
            polygons: request.into_inner().polygons,
        }))
    }
}

#[derive(Default)]
struct PostprocessServicer {}

#[tonic::async_trait]
impl PostprocessService for PostprocessServicer {
    async fn modify(
        &self,
        request: Request<PostprocessServiceModifyRequest>,
    ) -> Result<Response<PostprocessServiceModifyResponse>, Status> {
        Result::Ok(Response::new(PostprocessServiceModifyResponse {
            gcode_word: request.into_inner().gcode_word,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::]:5555".parse()?;
    let project_name = env!("CARGO_PKG_NAME");
    println!("{project_name} listening on {address}",);

    Server::builder()
        .add_service(SimplifyServiceServer::new(SimplifyServicer::default()))
        .add_service(PostprocessServiceServer::new(PostprocessServicer::default()))
        .serve(address)
        .await?;

    Ok(())
}
