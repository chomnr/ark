use axum::Router;
use smallvec::{SmallVec, smallvec};

const ARC_SERVER_ADDRESS: &str = "0.0.0.0:3000";

pub struct ArcServer {
    address: String,
}

impl Default for ArcServer {
    fn default() -> Self {
        Self {
            address: ARC_SERVER_ADDRESS.to_owned()
        }
    }
}

impl ArcServer {
    pub async fn run(self) {
        let app = Router::new();
        let listener = tokio::net::TcpListener::bind(ARC_SERVER_ADDRESS).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

/*
use super::sector::{
    iam::{auth::AuthPartial, root::IamSector},
    sector::{Sector, SectorPartial},
};

pub const ARC_SERVER_ADDRESS: &str = "0.0.0.0:3000";

pub struct ArcServer {
    address: String,
    router: Router,
    sectors: Vec<Box<dyn Sector>>,
}

impl Default for ArcServer {
    fn default() -> Self {
        Self {
            address: ARC_SERVER_ADDRESS.to_owned(),
            router: Router::new(),
            sectors: vec![Box::new(IamSector::new())],
        }
    }
}

impl ArcServer {
    pub fn new(router: Router, address: String) -> Self {
        Self {
            router,
            address,
            sectors: vec![],
        }
    }

    pub async fn run(self) {
        for sector in self.sectors.iter() {

        }

        let listener = tokio::net::TcpListener::bind(self.address).await.unwrap();
        axum::serve(listener, self.router).await.unwrap();
    }

    pub fn attach_sector<T: Sector>(&mut self, sector: T) {
        println!("[ARC] attached a sector.");
        self.sectors.push(Box::new(sector))
    }
}
*/
