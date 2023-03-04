mod crawlers {
    pub mod uniswap {
        pub mod liquidity;
    }
}
use dotenv::dotenv;
use std::env;
use tokio::runtime::Runtime;

use crate::crawlers::uniswap::liquidity::get_uniswap_liquidity;


fn main() {
    dotenv().ok();
    let uniswap_url = env::var("UNISWAP_URL").unwrap();
    println!("UNIURL: {}", uniswap_url);
    let token_pair = "DAI/USDC";
    let rt = Runtime::new().unwrap();
    let uniswap_tvl = rt.block_on(get_uniswap_liquidity(&uniswap_url, token_pair));
    print!("uniTVL: {:?}", uniswap_tvl);

}
