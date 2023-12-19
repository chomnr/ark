pub struct ArcServer;

/*
const ARC_SERVER_ADDRESS: &str = "0.0.0.0:3000";

pub struct ArcServer {
    sector: Sector,
    router: Router,
}

impl Default for ArcServer {
    fn default() -> Self {
        ArcServer {
            sector: Sector::default(),
            router: Router::new(),
        }
    }
}

impl ArcServer {
    pub async fn run(self) {
        self.load_sector_routes().await;
        let listener = tokio::net::TcpListener::bind(ARC_SERVER_ADDRESS)
            .await
            .unwrap();
        println!("[ARC] listening on {}", ARC_SERVER_ADDRESS);
        axum::serve(listener, Router::new()).await.unwrap();
    }

    async fn load_sector_routes(mut self) {
        self.router = self.router.nest("/auth", self.sector.iam.ap.router);
    }
}
*/