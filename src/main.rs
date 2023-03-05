mod crawlers {
    pub mod curve {
        pub mod crv_tvl;
    }
    pub mod uniswap {
        pub mod uniswap_tvl;
    }
}
use dotenv::dotenv;
use std::env;

use crate::crawlers::curve::crv_tvl::get_curve_tvl;
use crate::crawlers::uniswap::uniswap_tvl::get_uniswap_liquidity;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let uniswap_url = env::var("UNISWAP_URL").unwrap();
    let token_pair = "DAI/USDC";
    let curve_url = env::var("CURVE_URL").unwrap();

    let token_group = "compound";
    let token_labels = ["USDC", "DAI"];

    let uniswap_tvl = get_uniswap_liquidity(&uniswap_url, token_pair).await;
    let token_pairs = vec!["USDC", "DAI"];

    let curve_tvl = get_curve_tvl(&curve_url, token_group, &token_pairs).await;
    println!("TVL for {:?} in {} pool: {:?}", token_labels, token_group, curve_tvl);
    println!("UNI_TVL: {:?}", uniswap_tvl);
}