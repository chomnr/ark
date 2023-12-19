use axum::Router;

use crate::app::sector::sector::DEFAULT_NEST_ROUTE;

use super::{auth::AuthPartial, test::TestPartial};

pub struct IAMSector {
    ap: AuthPartial,
    tp: TestPartial
}

impl Default for IAMSector {
    fn default() -> Self {
        Self { 
            ap: AuthPartial::default(),
            tp: TestPartial::default(),
        }
    }
}

impl IAMSector {
    pub fn routes(self) -> Router {
        Router::new()
            .nest(DEFAULT_NEST_ROUTE, self.ap.router)
            .nest(DEFAULT_NEST_ROUTE, self.tp.router)
    }
}

//IAMSector -> AuthPartial | SessionPartial | AccessPartial | .... ...


//Iam::new(sss, ss, ss, ss, )

/*
pub struct IamSector {
    sector_name: String,
    partials: SmallVec<[dyn SectorPartial; 8]>,
}

impl Default for IamSector {
    fn default() -> Self {
        Self {
            sector_name: "IAM".to_string(),
            partials: smallvec![]
        }
    }
}

impl Sector for IamSector {
    fn get_sector_name(self) -> String {
        self.sector_name
    }

    fn get_partials(self) -> SmallVec<[dyn SectorPartial; 8]> {
        self.partials
    }
}
*/

/*
pub struct IamSector {
    partials: SmallVec<[Box<dyn SectorPartial<Self>>; 8]>,
}

impl Default for IamSector {
    fn default() -> Self {
        Self {
            partials: smallvec![Box::new(AuthPartial::new()) as Box<dyn SectorPartial<Self>>],
        }
    }
}

impl IamSector {
    pub fn new() -> Self {
        let mut instance = Self {
            partials: SmallVec::new(),
        };
        // sector auth partial
        instance.register_partial(AuthPartial::new());
        println!(
            "[ARC] registered {} partial sector(s) for IAM",
            &instance.partials.len()
        );
        instance
    }

    fn register_partial<T: SectorPartial<Self> + 'static>(&mut self, partial: T) {
        self.partials.push(Box::new(partial))
    }
}

impl Clone for IamSector {
    fn clone(&self) -> Self {
        let partials = self.partials.iter().map(|p| p.dyn_clone()).collect();
        Self { partials }
    }
}


impl Sector for IamSector {
    fn partials(&self) -> SmallVec<[Box<dyn SectorPartial<Self>>; 8]>
    where
        Self: Sized + 'static {
        todo!()
    }
}

*/
//let mut router = Router::new(); // Make `router` mutable
/*
for partial in self.partials.iter() {
    router.nest(&partial.nest_path(), partial.router().get_mut().unwrap().to_owned());
    //router.nest(&partial.nest_path(), partial.router()); // Modify the original router
}*/
//self.partials
