use axum::Router;

pub struct AuthPartial {
    pub router: Router,
}

impl Default for AuthPartial {
    fn default() -> Self {
        Self {
            router: Router::new(),
        }
    }
}

impl AuthPartial {

}

/*
impl Default for AuthPartial {
    fn default() -> Self {
        Self {
            nest_path: "/auth".to_string(),
            router: Router::new()
        }
    }
}

impl SectorPartial for AuthPartial {
    fn get_routes() -> Router {
        todo!()
    }
}*/
/*
pub struct AuthPartial {
    nest_path: String,
    router: Mutex<Router>
}

impl AuthPartial {
    pub fn new() -> Self {
        Self {
            nest_path: "/auth".to_string(),
            router: Mutex::new(Router::new()),
        }
    }
}

impl SectorPartial<IamSector> for AuthPartial {
    fn router(&self) -> &Mutex<Router> {
        &self.router
    }

    fn nest_path(&self) -> &String {
        &self.nest_path
    }
}
*/
