use calculator::adder_server::{Adder, AdderServer};
use calculator::{Answer, Operands};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
pub struct Calc {}

#[tonic::async_trait]
impl Adder for Calc {
    async fn add(&self, req: Request<Operands>) -> Result<Response<Answer>, Status> {
        println!("Request: {:?}", req);

        let ops = req.into_inner();
        let reply = Answer {
            ans: ops.num1 + ops.num2,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5000".parse()?;
    let something = Calc::default();

    Server::builder()
        .add_service(AdderServer::new(something))
        .serve(addr)
        .await?;

    Ok(())
}
