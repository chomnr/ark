use axum::Router;

pub struct TestPartial {
    pub router: Router,
}

impl Default for TestPartial {
    fn default() -> Self {
        Self {
            router: Router::new(),
        }
    }
}