use dbl::types::ShardStats;
use dbl::Client;
use tokio::runtime::Runtime;

fn main() {
    let token = match std::env::var("DBL_TOKEN") {
        Ok(token) => token,
        _ => panic!("missing token"),
    };

    let mut rt = Runtime::new().expect("failed rt");
    let client = Client::new(token).expect("failed client");

    let bot = 565_030_624_499_466_240;
    let stats = ShardStats::Cumulative {
        server_count: 1234,
        shard_count: None,
    };

    let task = client.update_stats(bot, stats);

    match rt.block_on(task) {
        Ok(_) => println!("Update successful"),
        Err(e) => eprintln!("{}", e),
    }
}
