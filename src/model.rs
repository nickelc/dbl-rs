use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BotId(pub u64);
#[derive(Debug)]
pub struct UserId(pub u64);
#[derive(Debug)]
pub struct GuildId(pub u64);

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

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

#[derive(Debug, Deserialize)]
pub struct Social {
    pub github: String,
    pub instagram: String,
    pub reddit: String,
    pub twitter: String,
    pub youtube: String,
}

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

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub server_count: Option<u64>,
    pub shards: Vec<u64>,
    pub shard_count: Option<u64>,
}

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

#[derive(Debug, Deserialize)]
pub struct Listing {
    pub results: Vec<Bot>,
    pub limit: u64,
    pub offset: u64,
    pub count: u64,
    pub total: u64,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WebhookType {
    Upvote,
    Test,
}

impl Webhook {
    pub fn is_test(&self) -> bool {
        match self.kind {
            WebhookType::Test => true,
            _ => false,
        }
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
