// gRPC server implementation

//First import the various types tonic generated for us based on our protocol buffer definitions

// Bring the types defined in the Tonic library in to the scope.
use tonic::{transport::Server, Request, Response, Status};

// Bring the generated types into scope
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};

// Module called payments to include the types tonic has generated for us based on the payments.proto file
pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait] //This attribute was added because the send_payment is going to be asyn
impl Bitcoin for BitcoinService {
    // This method takes a Request and returns the result of type gRPC response and status in case of an error
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main] // main to use the tokio async runtime
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:60051".parse()?;

    // Create new instance of BitcoinService
    let btc_service = BitcoinService::default();

    //Instantiate a new grpc server which uses bitcoin service
    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}
