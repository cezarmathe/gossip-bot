//! Storage-related module

mod model;
pub mod prelude {
    pub use super::DatabaseWrapper;

    pub use super::model::*;
    pub use super::repository::*;
}
mod repository;
mod schema;

use std::mem::MaybeUninit;
use std::sync::Once;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;

embed_migrations!();

#[inline]
pub(crate) fn u64_to_vecu8(src: u64) -> Vec<u8> {
    src.to_le_bytes().to_vec()
}

#[inline]
pub(crate) fn vecu8_to_u64(src: &[u8]) -> u64 {
    if src.len() != 8 {
        log::error!("u64 as bytes vector does not have 8 elements, panicking");
        panic!("{}", "very bad storage error");
    }
    let arr: [u8; 8] = [
        src[0], src[1], src[2], src[3], src[4], src[5], src[6], src[7],
    ];
    u64::from_le_bytes(arr)
}

/// Wrapper for the database.
///
/// Maintains the database connection pool and provides a method for running closures that leverage
/// a connection obtained from the pool.
pub struct DatabaseWrapper {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseWrapper {
    /// Get a database handler.
    ///
    /// The database handler itself is initialized as a singleton.
    pub fn get() -> &'static DatabaseWrapper {
        static mut CONF: MaybeUninit<DatabaseWrapper> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            CONF.as_mut_ptr()
                .write(DatabaseWrapper::new().expect("Failed to create the database wrapper"));

            // run the pending database migrations
            CONF.as_ptr()
                .as_ref()
                .unwrap()
                .run(|conn| {
                    if let Err(e) = diesel_migrations::run_pending_migrations(conn) {
                        Err(anyhow!("{}", e))
                    } else {
                        Ok(())
                    }
                })
                .expect("Failed to run database migrations");
        });

        unsafe { &*CONF.as_ptr() }
    }

    /// Create a new database handler.
    fn new() -> anyhow::Result<Self> {
        Ok(Self {
            pool: Pool::new(ConnectionManager::new(
                std::env::var("GOSSIP_BOT_DB_URI")
                    .expect("Expected an environment variable with the database URI."),
            ))?,
        })
    }

    /// Run a closure that is provided with a database connection.
    #[inline]
    pub fn run<F, R>(&self, func: F) -> anyhow::Result<R>
    where
        F: FnOnce(&SqliteConnection) -> anyhow::Result<R>,
        R: Sized,
    {
        let conn = self.pool.get()?;
        func(&conn)
    }
}
