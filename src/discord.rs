use crate::commands::*;

use std::collections::HashSet;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        log::info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        log::info!("Resumed");
    }
}

#[group]
#[commands(all, gossip, setguild)]
struct General;

/// Run the discord bot.
pub fn run(token: &str, prefix: &str) -> anyhow::Result<()> {
    let mut client = Client::new(&token, Handler)?;

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix(&prefix))
            .group(&GENERAL_GROUP),
    );

    client.start()?;

    Ok(())
}
