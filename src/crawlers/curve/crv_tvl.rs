use scraper::{Html, Selector};
use std::error::Error;

pub async fn get_curve_tvl(
    url: &str,
    token_group: &str,
    token_symbols: &[&str],
) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);

    let mut tvl = None;
    let token_labels_selector = Selector::parse(".sc-7ne9eu-0").unwrap();

    let tvl_selector = Selector::parse(".sc-14f2b9o-0.hNlYBy").unwrap();
    for pool in
        document.select(&Selector::parse(".sc-1ddj6ya-0.sc-1l6ho57-2.bpcBii.hKRpvQ").unwrap())
    {
        let pool_token_labels = pool
            .select(&token_labels_selector)
            .map(|label| label.inner_html())
            .collect::<Vec<_>>();

        let pool_token_symbols = pool_token_labels
            .iter()
            .map(|label| label.split_whitespace().next().unwrap_or(""))
            .collect::<Vec<_>>();
        if pool_token_symbols
            .iter()
            .any(|symbol| token_symbols.contains(&symbol))
        {
            let pool_token_group = pool
                .select(&Selector::parse(".sc-1ddj6ya-0.sc-5wjvyc-1.hkpEJy").unwrap())
                .next()
                .map(|group| group.text().collect::<String>());
            if let Some(group) = pool_token_group {
                if group.to_lowercase() == token_group {
                    let tvl_label = pool.select(&tvl_selector).next();
                    tvl = tvl_label.map(|tvl| tvl.text().collect::<Vec<_>>().join(""));
                    break;
                }
            }
        }
    }

    match tvl {
        Some(tvl) => Ok(tvl),
        None => Err("Curve TVL not found".into()),
    }
}
