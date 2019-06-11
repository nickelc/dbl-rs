use dbl::Client;
use tokio::runtime::Runtime;

fn main() {
    let token = match std::env::var("TOKEN") {
        Ok(token) => token,
        _ => panic!("missing token"),
    };

    let mut rt = Runtime::new().expect("failed rt");
    let client = Client::new(token).expect("failed client");

    let task = client.get(565_030_624_499_466_240);

    match rt.block_on(task) {
        Ok(bot) => println!("{:#?}", bot),
        Err(e) => eprintln!("{}", e),
    }
}
