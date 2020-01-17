//! # dbl-rs
//!
//! Rust bindings for the [top.gg](https://top.gg) / discordbots.org API.
//! ## Usage
//!
//! Add this to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! dbl-rs = "0.1"
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use dbl::types::ShardStats;
//! use dbl::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let token = match std::env::var("DBL_TOKEN") {
//!         Ok(token) => token,
//!         _ => panic!("missing token"),
//!     };
//!
//!     let client = Client::new(token).expect("failed client");
//!
//!     let bot = 565_030_624_499_466_240;
//!     let stats = ShardStats::Cumulative {
//!         server_count: 1234,
//!         shard_count: None,
//!     };
//!
//!     match client.update_stats(bot, stats).await {
//!         Ok(_) => println!("Update successful"),
//!         Err(e) => eprintln!("{}", e),
//!     }
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/dbl-rs/0.1.1")]
#![deny(rust_2018_idioms)]

use futures_util::TryFutureExt;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client as ReqwestClient, Response};
use reqwest::{Method, StatusCode};
use url::Url;

macro_rules! api {
    ($e:expr) => {
        concat!("https://top.gg/api", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

mod error;
pub mod types;
pub mod widget;

pub use error::Error;

use types::*;

/// Endpoint interface to Discord Bot List API.
#[derive(Clone)]
pub struct Client {
    client: ReqwestClient,
    token: String,
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new(token: String) -> Result<Self, Error> {
        let client = ReqwestClient::builder().build().map_err(error::from)?;
        Ok(Client { client, token })
    }

    /// Constructs a new `Client` with a `reqwest` client.
    pub fn new_with_client(client: ReqwestClient, token: String) -> Self {
        Client { client, token }
    }

    /// Get information about a specific bot.
    pub async fn get<T>(&self, bot: T) -> Result<Bot, Error>
    where
        T: Into<BotId>,
    {
        let url = api!("/bots/{}", bot.into());
        get(self, url).await
    }

    /// Search for bots.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dbl::types::Filter;
    ///
    /// let filter = Filter::new().search("lib:serenity foobar");
    /// ```
    pub async fn search(&self, filter: &Filter) -> Result<Listing, Error> {
        let url = Url::parse_with_params(&api!("/bots"), &filter.0).map_err(Error::Url)?;
        get(self, url.to_string()).await
    }

    /// Get the stats of a bot.
    pub async fn stats<T>(&self, bot: T) -> Result<Stats, Error>
    where
        T: Into<BotId>,
    {
        let url = api!("/bots/{}/stats", bot.into());
        get(self, url).await
    }

    /// Update the stats of a bot.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dbl::types::ShardStats;
    ///
    /// let new_stats = ShardStats::Cumulative {
    ///     server_count: 1234,
    ///     shard_count: None,
    /// };
    /// ```
    pub async fn update_stats<T>(&self, bot: T, stats: ShardStats) -> Result<(), Error>
    where
        T: Into<BotId>,
    {
        let url = api!("/bots/{}/stats", bot.into());
        post(self, url, Some(stats)).await
    }

    /// Get the last 1000 votes for a bot.
    pub async fn votes<T>(&self, bot: T) -> Result<Vec<User>, Error>
    where
        T: Into<BotId>,
    {
        let url = api!("/bots/{}/votes", bot.into());
        get(self, url).await
    }

    /// Check if a user has voted for a bot in the past 24 hours.
    pub async fn has_voted<T, U>(&self, bot: T, user: U) -> Result<bool, Error>
    where
        T: Into<BotId>,
        U: Into<UserId>,
    {
        let bot = bot.into();
        let user = user.into();
        let url = api!("/bots/{}/check?userId={}", bot, user);
        let v: UserVoted = get(self, url).await?;
        Ok(v.voted > 0)
    }

    /// Get information about a user.
    pub async fn user<T>(&self, user: T) -> Result<DetailedUser, Error>
    where
        T: Into<UserId>,
    {
        let url = api!("/users/{}", user.into());
        get(self, url).await
    }
}

async fn request<T>(
    client: &Client,
    method: Method,
    url: String,
    data: Option<T>,
) -> Result<Response, Error>
where
    T: serde::Serialize + Sized,
{
    let mut req = client
        .client
        .request(method, &url)
        .header(AUTHORIZATION, &*client.token);

    if let Some(data) = data {
        req = req.json(&data);
    }

    let resp = req.send().map_err(error::from).await?;
    match resp.status() {
        StatusCode::TOO_MANY_REQUESTS => {
            let rl = resp.json::<Ratelimit>().map_err(error::from).await?;
            Err(error::ratelimit(rl.retry_after))
        }
        _ => resp.error_for_status().map_err(error::from),
    }
}

async fn get<T>(client: &Client, url: String) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + Sized,
{
    let resp = request(client, Method::GET, url, None::<()>).await?;
    resp.json().map_err(error::from).await
}

async fn post<T>(client: &Client, url: String, data: Option<T>) -> Result<(), Error>
where
    T: serde::Serialize + Sized,
{
    request(client, Method::POST, url, data).await?;
    Ok(())
}
