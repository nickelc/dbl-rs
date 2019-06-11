use dbl::types::Webhook;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::path;
use warp::{Filter, Rejection, Reply};

fn main() {
    let secret = "mywebhook";

    let filter = warp::header::<String>("authorization")
        .and_then(move |value| {
            if value == secret {
                Ok(())
            } else {
                Err(warp::reject::custom(Unauthorized))
            }
        })
        .untuple_one();
    let webhook = warp::post2()
        .and(path!("dbl" / "webhook"))
        .and(filter)
        .and(warp::body::json())
        .map(|hook: Webhook| {
            println!("{:?}", hook);
            warp::reply()
        })
        .recover(custom_error);

    warp::serve(webhook).run(([127, 0, 0, 1], 3030));
}

fn custom_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.find_cause::<BodyDeserializeError>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply(),
            StatusCode::BAD_REQUEST,
        ))
    } else if err.find_cause::<Unauthorized>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply(),
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        Err(err)
    }
}

#[derive(Debug)]
struct Unauthorized;

impl std::fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unauthorized")
    }
}

impl std::error::Error for Unauthorized {}
