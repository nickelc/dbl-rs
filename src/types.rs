use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Newtype for bot ids.
#[derive(Debug)]
pub struct BotId(pub u64);
/// Newtype for user ids.
#[derive(Debug)]
pub struct UserId(pub u64);
/// Newtype for guild ids.
#[derive(Debug)]
pub struct GuildId(pub u64);

/// Basic user information returned by [`Client::votes`](super::Client::votes).
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

/// Detailed user information returned by [`Client::user`](super::Client::user).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedUser {
    pub id: UserId,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    #[serde(rename = "defAvatar")]
    pub default_avatar: String,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub social: Social,
    pub color: Option<String>,
    pub supporter: bool,
    pub certified_dev: bool,
    #[serde(rename = "mod")]
    pub mod_: bool,
    pub web_mod: bool,
    pub admin: bool,
}

/// Social media accounts of the user.
#[derive(Debug, Deserialize)]
pub struct Social {
    pub github: String,
    pub instagram: String,
    pub reddit: String,
    pub twitter: String,
    pub youtube: String,
}

/// Information about a bot.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    pub id: BotId,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    #[serde(rename = "defAvatar")]
    pub default_avatar: String,
    pub clientid: String,
    pub lib: String,
    pub prefix: String,
    #[serde(rename = "shortdesc")]
    pub short_desc: String,
    #[serde(rename = "longdesc")]
    pub long_desc: Option<String>,
    pub tags: Vec<String>,
    pub website: Option<String>,
    pub support: Option<String>,
    pub github: Option<String>,
    pub owners: Vec<UserId>,
    pub guilds: Vec<GuildId>,
    pub invite: Option<String>,
    pub date: String,
    pub certified_bot: bool,
    pub vanity: Option<String>,
    pub shards: Vec<u64>,
    pub points: u64,
    pub monthly_points: u64,
}

/// Bot's sharding stats.
#[derive(Debug, Deserialize)]
pub struct Stats {
    pub server_count: Option<u64>,
    pub shards: Vec<u64>,
    pub shard_count: Option<u64>,
}

/// Used to update one or more sharding stats.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ShardStats {
    Cumulative {
        server_count: u64,
        shard_count: Option<u64>,
    },
    Shard {
        server_count: u64,
        shard_id: u64,
        shard_count: u64,
    },
    Shards {
        shards: Vec<u64>,
    },
}

/// Used for filtering the bot search.
pub struct Filter(pub(crate) HashMap<&'static str, String>);

impl Default for Filter {
    fn default() -> Filter {
        Filter::new()
    }
}

impl Filter {
    pub fn new() -> Filter {
        Filter(HashMap::with_capacity(4))
    }

    pub fn limit(mut self, mut limit: u16) -> Filter {
        if limit > 500 {
            limit = 500;
        }
        self.0.insert("limit", limit.to_string());
        self
    }

    pub fn offset(mut self, offset: u16) -> Filter {
        self.0.insert("offset", offset.to_string());
        self
    }

    pub fn sort<T: AsRef<str>>(mut self, field: T, ascending: bool) -> Filter {
        let mut buf = String::new();
        if !ascending {
            buf.push('-');
        }
        buf.push_str(field.as_ref());
        self.0.insert("sort", buf);
        self
    }

    /// Search string. Example: `lib:serenity mod`
    pub fn search<T: ToString>(mut self, search: T) -> Filter {
        self.0.insert("search", search.to_string());
        self
    }
}

/// Search result returned by [`Client::search`](super::Client::search).
#[derive(Debug, Deserialize)]
pub struct Listing {
    pub results: Vec<Bot>,
    pub limit: u64,
    pub offset: u64,
    pub count: u64,
    pub total: u64,
}

/// Vote received via webhook.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub bot: BotId,
    pub user: UserId,
    #[serde(rename = "type")]
    pub kind: WebhookType,
    pub is_weekend: bool,
    pub query: Option<String>,
}

/// Type of vote received via webhook.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WebhookType {
    Upvote,
    Test,
}

impl Webhook {
    pub fn is_test(&self) -> bool {
        std::matches!(self.kind, WebhookType::Test)
    }
}

impl ::std::ops::Index<usize> for Listing {
    type Output = Bot;

    fn index(&self, index: usize) -> &Self::Output {
        &self.results[index]
    }
}

impl IntoIterator for Listing {
    type Item = Bot;
    type IntoIter = ::std::vec::IntoIter<Bot>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

impl<'a> IntoIterator for &'a Listing {
    type Item = &'a Bot;
    type IntoIter = ::std::slice::Iter<'a, Bot>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.iter()
    }
}

#[derive(Deserialize)]
pub(crate) struct UserVoted {
    pub voted: u64,
}

#[derive(Deserialize)]
#[serde(rename = "kebab-case")]
pub(crate) struct Ratelimit {
    pub retry_after: u32,
}

macro_rules! impl_snowflake {
    ($($type:ty),*) => {
        $(
            impl $type {
                pub fn as_u64(&self) -> u64 {
                    self.0
                }
            }

            impl ::std::fmt::Display for $type {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl From<u64> for $type {
                fn from(v: u64) -> Self {
                    Self(v)
                }
            }

            impl<'de> ::serde::de::Deserialize<'de> for $type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'de> ::serde::de::Visitor<'de> for Visitor {
                        type Value = $type;

                        fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str("identifier")
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            v.parse::<u64>().map(Into::into).map_err(|_| {
                                E::custom(format!("invalid {}: value {}", stringify!(u64), v))
                            })
                        }
                    }

                    deserializer.deserialize_str(Visitor)
                }
            }

            impl ::serde::ser::Serialize for $type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(&self.0.to_string())
                }
            }
        )*
    };
}

impl_snowflake!(BotId, GuildId, UserId);
