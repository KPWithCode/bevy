use scraper::{Html, Selector};

pub async fn get_uniswap_liquidity(url: &str, token_pair: &str) -> Result<String, Box<dyn std::error::Error>> {
    
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let pair_selector = Selector::parse(".tw-pair-selector input").unwrap();
    let mut token_one: Option<&str> = None;
    let mut token_two: Option<&str> = None;
    for input in document.select(&pair_selector) {
        if input.value().attr("placeholder") == Some(token_pair) {
            let value = input.value().attr("value").unwrap();
            let tokens: Vec<&str> = value.split("-").collect();
            token_one = tokens.get(0).cloned();
            token_two = tokens.get(1).cloned();
            break;
        }
    }
    if let (Some(token_one), Some(token_b)) = (token_one, token_two) {
        let tvl_selector = Selector::parse(".s-liquidity__tvl-value").unwrap();
        for tvl in document.select(&tvl_selector) {
            if tvl.inner_html().contains(token_one) && tvl.inner_html().contains(token_b) {
                return Ok(tvl.inner_html());
            }
        }
    }
    Err("TVL not found".into())
}
