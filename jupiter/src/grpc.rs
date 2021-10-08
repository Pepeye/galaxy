use dgraph_tonic::Client;
use neptune::ping::ping_service_server::PingService;
use neptune::ping::{Db, Pong, Ts};
use prost_types::Timestamp;
use std::time::SystemTime;
use tonic::{Request, Response, Status};
use tracing::instrument;

#[derive(Debug)]
pub struct Service {
    pub client: Client,
}

#[tonic::async_trait]
impl PingService for Service {
    #[instrument]
    async fn ping(&self, _req: Request<()>) -> Result<Response<Pong>, Status> {
        let version = self.client.check_version().await.expect("Version");
        let data = Some(Db {
            name: "dgraph".into(),
            version: version.tag,
        });
        let ts = Some(Ts {
            created: Some(Timestamp::from(SystemTime::now())),
            updated: Some(Timestamp::from(SystemTime::now())),
        });

        let resp = Pong {
            id: "someid".to_string(),
            msg: "pong".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            data,
            ts,
        };

        Ok(Response::new(resp))
    }
}
