use super::schema::gossipers;
use super::schema::gossips;

use chrono::prelude::*;

#[derive(Identifiable, Queryable)]
pub struct Gossip {
    id: i32,
    msg: String,
    added: NaiveDateTime,
    guild_id: Vec<u8>,
}

impl Gossip {
    /// Get the message.
    #[inline]
    pub fn message(&self) -> &str {
        self.msg.as_str()
    }

    /// Get the time at which this gossip was added.
    #[inline]
    pub fn added(&self) -> &NaiveDateTime {
        &self.added
    }

    /// Get the guild id.
    #[inline]
    pub fn guild_id(&self) -> u64 {
        super::vecu8_to_u64(&self.guild_id)
    }
}

#[derive(Insertable)]
#[table_name = "gossips"]
pub struct NewGossip<'a> {
    msg: &'a str,
    guild_id: Vec<u8>,
}

impl<'a> NewGossip<'a> {
    #[inline]
    /// Create a new gossip.
    pub fn new<S, U>(message: S, guild_id: u64) -> Self
    where
        S: Into<&'a str>,
        U: Into<u64>,
    {
        NewGossip::new_impl(message.into(), guild_id.into())
    }

    fn new_impl(message: &'a str, guild_id: u64) -> Self {
        NewGossip {
            msg: message,
            guild_id: super::u64_to_vecu8(guild_id),
        }
    }

    /// Get the message.
    #[inline]
    pub fn message(&self) -> &'a str {
        self.msg
    }

    /// Get the guild id.
    #[inline]
    pub fn guild_id(&self) -> u64 {
        super::vecu8_to_u64(&self.guild_id)
    }
}

#[derive(Identifiable, Queryable)]
pub struct Gossiper {
    id: i32,
    discord_id: Vec<u8>,
    /// The preferred guild is only used by a select group of commands and only when talking to the
    /// bot via a private message.
    preferred_guild: Option<Vec<u8>>,
}

impl Gossiper {
    #[inline]
    pub fn discord_id(&self) -> u64 {
        super::vecu8_to_u64(&self.discord_id)
    }

    #[inline]
    pub fn preferred_guild(&self) -> Option<u64> {
        match &self.preferred_guild {
            Some(value) => Some(super::vecu8_to_u64(value)),
            None => None,
        }
    }
}

#[derive(Insertable)]
#[table_name = "gossipers"]
pub struct NewGossiper {
    discord_id: Vec<u8>,
    preferred_guild: Option<Vec<u8>>,
}

impl NewGossiper {
    /// Create a new gossiper.
    #[inline]
    pub fn new<V, O>(discord_id: V, preferred_guild: O) -> Self
    where
        V: Into<u64>,
        O: Into<Option<u64>>,
    {
        Self::new_impl(discord_id.into(), preferred_guild.into())
    }

    fn new_impl(discord_id: u64, preferred_guild: Option<u64>) -> Self {
        Self {
            discord_id: super::u64_to_vecu8(discord_id),
            preferred_guild: match preferred_guild {
                Some(value) => Some(super::u64_to_vecu8(value)),
                None => None,
            },
        }
    }

    #[inline]
    pub fn discord_id(&self) -> u64 {
        super::vecu8_to_u64(&self.discord_id)
    }

    #[inline]
    pub fn prefered_guild(&self) -> Option<u64> {
        match &self.preferred_guild {
            Some(value) => Some(super::vecu8_to_u64(value)),
            None => None,
        }
    }
}
