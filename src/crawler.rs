use reqwest::blocking::Client;
use scraper::{Html, Selector};

pub fn get_dex_liquidity(url: &str, token_pair: &str) -> Result<String, Box<dyn std::error::Error>> {
    // send request
    let client = Client::new();
    let res = client.get(url).send()?;
    // read and parse response body
    let body = res.text()?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("table tr").unwrap();

    let mut liquidity = String::new();

    for element in fragment.select(&selector) {
        if let Some(pair) = element.text().next() {
            // If the pair is found, get the liquidity value from the third cell in the row
            // pair.contains("USDC") && pair.contains("WETH")
            if pair.contains(token_pair) && pair.contains(token_pair) {
                if let Some(liq) = element.text().nth(3) {
                    liquidity = liq.trim().to_owned();
                    break;
                }
            }
        }
    }

    Ok(liquidity)
}
