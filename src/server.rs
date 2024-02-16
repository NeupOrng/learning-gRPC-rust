use tonic::{transport::Server, Request, Response, Status};

// use hello_world::greeter_server::{Greeter, GreeterServer};
// use hello_world::{HelloReply, HelloRequest};

// pub mod hello_world {
//     tonic::include_proto!("helloworld");
// }

use profile::user_profile_server::{UserProfile, UserProfileServer};
use profile::{GetProfileByIdRequest, GetProfileByIdResponse};

pub mod profile {
    tonic::include_proto!("profile");
}

// #[derive(Debug, Default)]
// pub struct MyGreeter {}

#[derive(Debug, Default)]
pub struct MyUserProfile {}

// #[tonic::async_trait]
// impl Greeter for MyGreeter {
//     async fn say_hello(
//         &self,
//         request: Request<HelloRequest>,
//     ) -> Result<Response<HelloReply>, Status> {
//         println!("Got a request: {:?}", request);

//         let reply = hello_world::HelloReply {
//             message: format!("Hello {}!", request.into_inner().name),
//         };

//         Ok(Response::new(reply))
//     }
// }

#[tonic::async_trait]
impl UserProfile for MyUserProfile {
    async fn get_profile_by_id(&self,request: Request<GetProfileByIdRequest>) -> Result<Response<GetProfileByIdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = profile::GetProfileByIdResponse {
            id: request.into_inner().id,
            name: "Orng".to_string(),
            status_id: 1,
            created_at: "12/12/2024".to_string(),
            updated_at: "12/12/2024".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    // let greeter = MyGreeter::default();
    let user_profile = MyUserProfile::default();

    Server::builder()
        // .add_service(GreeterServer::new(greeter))
        .add_service(UserProfileServer::new(user_profile))
        .serve(addr)
        .await?;

    Ok(())
}