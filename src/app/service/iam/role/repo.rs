use crate::app::{
    database::postgres::PostgresDatabase,
    service::
        iam::error::IamResult
    ,
};

use super::model::Role;

pub struct RoleRepo {
    pg: PostgresDatabase,
}

impl RoleRepo {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    pub fn create_role(role: Role) -> IamResult<bool> {
        //Event::<Role>::add("dasds", "dsad");
        //Event::add(EventTarget::Role, "adssd", "dsadsa");
        //assign an id
        //BridgeEvent::add(BridgeType::Role, "adssdadas", "111")
        //BridgeEvent::new().
        //let res = BridgeEvent::builder()
        //.bridge_type(BridgeType::Role)
        //.bridge_payload("asdadsdsa")
        //.send()
        // .await;
        /*
        BridgeEvent::builder()
            .bridge_type(BridgeType::Role)
            .bridge_payload("payload".to_string())
            .priority(BridgePriority::HIGH)
            .build()
            .send();
        */
        todo!()
    }
}
