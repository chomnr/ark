/*
use super::{Sector, SectorPartial};

pub struct Iam {
    partials: Vec<Box<dyn SectorPartial<Self>>>
}

impl Sector for Iam {
    fn partials(self) -> Vec<Box<dyn super::SectorPartial<Self>>> {
        self.partials
    }
}

pub struct TestPartial;

impl SectorPartial<Iam> for TestPartial {
    fn routes(&self) -> Vec<axum::routing::MethodRouter> {
        todo!()
    }
}


pub struct Cool {
    sectors: Vec<Box<dyn Sector>>
}

impl Cool {
    pub fn test(self) {
        for n in 0..self.sectors.len(){
            let test = self.sectors.get(n).expect("Unable to unwrap sector").partials();
        }
    }
}
*/