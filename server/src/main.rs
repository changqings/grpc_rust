use hello::hello_service_server::HelloService;
use hello::hello_service_server::HelloServiceServer;
use hello::HelloReply;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct Hello {}

#[tonic::async_trait]
impl HelloService for Hello {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloReply>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let r = request.into_inner();
        let reply = HelloReply {
            name: r.name,
            message: format!("{} from rs server", r.message),
        };

        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8081".parse()?;
    let svc = Hello::default();

    tonic::transport::Server::builder()
        .add_service(HelloServiceServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
