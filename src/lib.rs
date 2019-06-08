use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{self, Future};
use reqwest::header::AUTHORIZATION;
use reqwest::r#async::{Client as ReqwestClient, Response};
use reqwest::{Method, StatusCode, Url};

mod error;
pub mod model;

pub use error::Error;

use model::*;

type BoxFuture<T> = Box<dyn Future<Item = T, Error = Error> + Send>;

macro_rules! api {
    ($e:expr) => {
        concat!("https://discordbots.org/api", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

pub struct Client {
    client: Arc<ReqwestClient>,
    token: String,
}

impl Client {
    /// Constructs a new `Client`.
    ///
    /// # Panic
    ///
    /// This method panics if the `reqwest` client cannot initialized.
    pub fn new(token: String) -> Self {
        let client = Arc::new(ReqwestClient::new());
        Client { client, token }
    }

    /// Constructs a new `Client` with a `reqwest` client.
    pub fn new_with(client: Arc<ReqwestClient>, token: String) -> Self {
        Client { client, token }
    }

    /// Get information about a specific bot.
    pub fn get<T>(&self, bot: T) -> impl Future<Item = Bot, Error = Error>
    where
        T: Into<BotId>,
    {
        self.get2(&api!("/bots/{}", bot.into()))
            .and_then(|mut resp| resp.json().map_err(error::from))
    }

    /// Search for bots.
    pub fn search(&self, filter: &Filter) -> impl Future<Item = Listing, Error = Error> {
        let url = match Url::parse_with_params(&api!("/bots"), filter.0.iter()) {
            Ok(url) => url,
            Err(e) => return future::Either::A(future::err(Error::Url(e))),
        };
        future::Either::B(
            self.get2(&url.to_string())
                .and_then(|mut resp| resp.json().map_err(error::from)),
        )
    }

    /// Get the stats of a bot.
    pub fn stats<T>(&self, bot: T) -> impl Future<Item = Stats, Error = Error>
    where
        T: Into<BotId>,
    {
        self.get2(&api!("/bots/{}/stats", bot.into()))
            .and_then(|mut resp| resp.json().map_err(error::from))
    }

    /// Post the stats of a bot.
    pub fn post_stats<T>(&self, bot: T, stats: PostStats) -> impl Future<Item = (), Error = Error>
    where
        T: Into<BotId>,
    {
        self.post(&api!("/bots/{}/stats", bot.into()), Some(stats))
            .map(|_| ())
    }

    /// Get the last 1000 votes for a bot.
    pub fn votes<T>(&self, bot: T) -> impl Future<Item = Vec<User>, Error = Error>
    where
        T: Into<BotId>,
    {
        self.get2(&api!("/bots/{}/votes", bot.into()))
            .and_then(|mut resp| resp.json().map_err(error::from))
    }

    /// Check if a user has voted for a bot in the past 24 hours.
    pub fn has_voted<T, U>(&self, bot: T, user: U) -> impl Future<Item = bool, Error = Error>
    where
        T: Into<BotId>,
        U: Into<UserId>,
    {
        self.get2(&api!("/bots/{}/check?userId={}", bot.into(), user.into()))
            .and_then(|mut resp| resp.json::<UserVoted>().map_err(error::from))
            .map(|v| v.voted > 0)
    }

    /// Get information about a user.
    pub fn user<T>(&self, user: T) -> impl Future<Item = DetailedUser, Error = Error>
    where
        T: Into<UserId>,
    {
        self.get2(&api!("/users/{}", user.into()))
            .and_then(|mut resp| resp.json().map_err(error::from))
    }

    fn request<T>(
        &self,
        method: Method,
        url: &str,
        data: Option<T>,
    ) -> impl Future<Item = Response, Error = Error>
    where
        T: ::serde::ser::Serialize + Sized,
    {
        let mut req = self
            .client
            .request(method, url)
            .header(AUTHORIZATION, &*self.token);

        if let Some(data) = data {
            req = req.json(&data);
        }

        req.send()
            .map_err(error::from)
            .and_then(|mut resp| -> BoxFuture<Response> {
                match resp.status() {
                    StatusCode::TOO_MANY_REQUESTS => Box::new(
                        resp.json::<Ratelimit>()
                            .map_err(error::from)
                            .and_then(|r| Err(error::ratelimit(r.retry_after))),
                    ),
                    _ => Box::new(future::result(resp.error_for_status().map_err(error::from))),
                }
            })
    }

    fn get2(&self, url: &str) -> impl Future<Item = Response, Error = Error> {
        self.request(Method::GET, url, None::<()>)
    }

    fn post<T>(&self, url: &str, data: Option<T>) -> impl Future<Item = Response, Error = Error>
    where
        T: ::serde::Serialize + Sized,
    {
        self.request(Method::POST, url, data)
    }
}

/// Filter for searching bots.
pub struct Filter(HashMap<&'static str, String>);

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
