use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::Connection;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn new_pool(database_url: String, is_test_environment: bool) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // MEMO: When tested, pool size should be set to 1 because of begin_test_transaction() method.
    let pool_size = if is_test_environment { 1 } else { 10 };
    let pool = diesel::r2d2::Pool::builder()
        .max_size(pool_size)
        .build(manager)?;

    if is_test_environment {
        start_test_transaction(&pool);
    }

    Ok(pool)
}

fn start_test_transaction(pool: &Pool<ConnectionManager<PgConnection>>) {
    pool.get().unwrap().begin_test_transaction().unwrap();
}
