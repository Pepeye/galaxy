// crate dependencies
use crate::configuration::{Configuration, DatabaseSettings};
use crate::grpc::Service;
use neptune::ping::ping_service_server::PingServiceServer;
use tonic::transport::Server;

// dependencies
use anyhow::Result;
use dgraph_tonic::{Client, ClientError};
use std::net::TcpListener;

// module dependencies

pub struct Application {
    address: String,
    port: u16,
    service: Service,
}

impl Application {
    pub async fn build(configuration: Configuration) -> Result<Self, std::io::Error> {
        let client = get_client(&configuration.db)
            .await
            .expect("failed to connect to database");

        let address = format!("{}:{}", configuration.app.host, configuration.app.port);

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let service = Service { client };
        // let server = run(listener, client)?;

        Ok(Self {
            address,
            port,
            service,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), tonic::transport::Error> {
        let svc = PingServiceServer::new(self.service);
        Ok(Server::builder()
            .add_service(svc)
            .serve(self.address.parse().unwrap())
            .await?)
    }
}

pub async fn get_client(dbconf: &DatabaseSettings) -> Result<Client, ClientError> {
    let c = Client::new(format!("http://{}:{}", dbconf.host, dbconf.port))
        .expect("Faild to connect to dgraph");
    Ok(c)
}
