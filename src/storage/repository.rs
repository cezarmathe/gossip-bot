//! Various model repository for easily interacting with the database.

use super::model::Gossip;
use super::model::Gossiper;
use super::model::NewGossip;
use super::model::NewGossiper;
use super::DatabaseWrapper;

use diesel::prelude::*;

pub struct GossipRepository;

impl GossipRepository {
    /// Save a new gossip.
    pub fn save(gossip: NewGossip) -> anyhow::Result<()> {
        use super::schema::gossips::dsl::*;

        DatabaseWrapper::get().run(move |conn| {
            match diesel::insert_into(gossips).values(gossip).execute(conn) {
                Ok(val) => {
                    if val == 0 {
                        Err(anyhow!("no gossip was added"))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(anyhow!("{}", e)),
            }
        })
    }

    /// Find all gossips for a guild id.
    pub fn find_all_by_guild_id(guild_id_filter: u64) -> anyhow::Result<Vec<Gossip>> {
        use super::schema::gossips::dsl::*;

        let guild_id_vec = super::u64_to_vecu8(guild_id_filter);

        DatabaseWrapper::get().run(|conn| {
            match gossips
                .filter(guild_id.eq(guild_id_vec))
                .load::<Gossip>(conn)
            {
                Ok(value) => Ok(value),
                Err(e) => Err(anyhow!("{}", e)),
            }
        })
    }

    /// Find the latest gossip by guild id.
    pub fn find_by_guild_id_ord_added_desc(guild_id_filter: u64) -> anyhow::Result<Option<Gossip>> {
        use super::schema::gossips::dsl::*;

        let guild_id_vec = super::u64_to_vecu8(guild_id_filter);

        DatabaseWrapper::get().run(|conn| {
            match gossips
                .filter(guild_id.eq(guild_id_vec))
                .order(added.desc())
                .first::<Gossip>(conn)
                .optional()
            {
                Ok(value) => Ok(value),
                Err(e) => Err(anyhow!("{}", e)),
            }
        })
    }
}

pub struct GossiperRepository;

impl GossiperRepository {
    /// Save a new gossiper.
    pub fn save(gossiper: NewGossiper) -> anyhow::Result<()> {
        use super::schema::gossipers::dsl::*;

        DatabaseWrapper::get().run(move |conn| {
            match diesel::insert_into(gossipers)
                .values(gossiper)
                .execute(conn)
            {
                Ok(val) => {
                    if val == 0 {
                        Err(anyhow!("no gossiper was added"))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(anyhow!("{}", e)),
            }
        })
    }

    /// Find a gossiper by discord id.
    pub fn find_by_discord_id(discord_id_filter: u64) -> anyhow::Result<Option<Gossiper>> {
        use super::schema::gossipers::dsl::*;

        let discord_id_vec = super::u64_to_vecu8(discord_id_filter);

        DatabaseWrapper::get().run(|conn| {
            match gossipers
                .filter(discord_id.eq(discord_id_vec))
                .first::<Gossiper>(conn)
                .optional()
            {
                Ok(value) => Ok(value),
                Err(e) => Err(anyhow!("{}", e)),
            }
        })
    }

    /// Update a gossiper's guild id by discord id.
    pub fn update_preferred_guild_by_discord_id(
        discord_id_filter: u64,
        guild_id: u64,
    ) -> anyhow::Result<()> {
        use super::schema::gossipers::dsl::*;

        let gossiper = match Self::find_by_discord_id(discord_id_filter)? {
            Some(value) => value,
            None => bail!("there is no gossiper to update"),
        };

        DatabaseWrapper::get().run(|conn| {
            match diesel::update(&gossiper)
                .set(preferred_guild.eq(Some(super::u64_to_vecu8(guild_id))))
                .execute(conn)
            {
                Ok(value) => {
                    if value != 0 {
                        Ok(())
                    } else {
                        Err(anyhow!("no gossiper was updated"))
                    }
                }
                Err(e) => Err(anyhow!("{}", e)),
            }
        })
    }
}
