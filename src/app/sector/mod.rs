use std::{collections::HashMap, sync::Arc};

use axum::routing::MethodRouter;
use dashmap::DashMap;

pub mod iam;

pub trait SectorPartial {
    fn routes(&self) -> Vec<MethodRouter>;
}

pub struct Sector {
    data: HashMap<String, Vec<Arc<dyn SectorPartial>>>,
}

impl Default for Sector {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl Sector {
    pub fn create_sector(
        &mut self,
        sector_name: &str,
        nest_path: &str,
        list: Vec<Arc<dyn SectorPartial>>,
    ) {
        println!("[ARC] registered {} partial sector(s) for {}", list.len(), sector_name);
        let name = String::with_capacity(sector_name.len());
        self.data.insert(name, list);
    }
}

/*
ArcServer arc = ArcServer::default

arc.create_sector("iam", vec![
    AuthPartial::default,
    SessionPartial::default
]);

*/
/*
pub fn test(){
    let test = Sector {
        iam: todo!(),
    };
    //test.iam.get(0).unwrap().routes();
}*/

/*
pub trait Sector {
    fn partial<T: SectorPartial<Self> + ?Sized>() {

    }
}

pub trait SectorPartial<T: Sector> {
    fn routes(&self) -> Vec<MethodRouter>;
}
*/
