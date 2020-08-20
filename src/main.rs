#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate thiserror;

mod commands;
mod discord;
mod service;
mod storage;

use std::env;

fn main() -> anyhow::Result<()> {
    kankyo::load()?;
    env_logger::init();

    let token = env::var("GOSSIP_BOT_TOKEN")?;
    let prefix = env::var("GOSSIP_BOT_PREFIX")?;

    {
        let _ = storage::DatabaseWrapper::get();
    }

    discord::run(&token, &prefix)
}
