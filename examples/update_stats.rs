use dbl::types::ShardStats;
use dbl::Client;

#[tokio::main]
async fn main() {
    let token = match std::env::var("DBL_TOKEN") {
        Ok(token) => token,
        _ => panic!("missing token"),
    };

    let client = Client::new(token).expect("failed client");

    let bot = 565_030_624_499_466_240;
    let stats = ShardStats::Cumulative {
        server_count: 1234,
        shard_count: None,
    };

    match client.update_stats(bot, stats).await {
        Ok(_) => println!("Update successful"),
        Err(e) => eprintln!("{}", e),
    }
}
