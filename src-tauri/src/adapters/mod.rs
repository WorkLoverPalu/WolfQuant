pub mod fund;
pub mod crypto;
pub mod stock;
pub mod gold;

use crate::market::MarketAdapter;

pub fn get_adapter(asset_type: &str, source: &str) -> Result<Box<dyn MarketAdapter>, String> {
    match (asset_type, source) {
        ("fund", "tiantian") => Ok(Box::new(fund::tiantian::TiantianFundAdapter::new())),
        ("fund", "sina") => Ok(Box::new(fund::sina::SinaFundAdapter::new())),
        ("crypto", "binance") => Ok(Box::new(crypto::binance::BinanceAdapter::new(None, None))),
        ("crypto", "okex") => Ok(Box::new(crypto::okex::OkexAdapter::new(None, None))),
        _ => Err(format!("Unsupported asset type: {} or source: {}", asset_type, source)),
    }
}