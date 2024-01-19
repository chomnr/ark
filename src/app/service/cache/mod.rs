pub trait CacheDatabase {

}

pub trait Cache<DB: CacheDatabase> {

}


//Cache<CacheType(Local or Global)>
// global being database.

//impl Cache<RedisCache>
//impl Cache<LocalCache>