use std::{env, process};
use MulticallUniswapV3::fetcher::run;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("No RPC_URL environment variable.");
    if let Err(e) = run(&rpc_url).await {
        eprintln!("Программа завершилась с ошибкой: {}", e);
        process::exit(1);
    }
}
