mod crawler;
use dotenv::dotenv;
use std::env;

use crawler::get_dex_liquidity;


fn main() {
    dotenv().ok();
    let pancakeswap_url= env::var("PANCAKESWAP_URL").unwrap();
    let uniswap_url = env::var("UNISWAP_URL").unwrap();
    let pancakeswap_liquidity = get_dex_liquidity(&pancakeswap_url, "USDT/BUSD").unwrap();
    let uniswap_liquidity = get_dex_liquidity(&uniswap_url, "USDT/BUSD").unwrap();

    // Get the liquidity of the USDT/BUSD pair on PancakeSwap
    let pancakeswap_liquidity = get_dex_liquidity(&pancakeswap_liquidity, "USDT/BUSD").unwrap();
    println!("uni: {}", uniswap_liquidity);
    println!("ps: {}", pancakeswap_liquidity);

}
