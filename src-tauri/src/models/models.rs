use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{FromRow, Row};

use crate::models::{Candle, Product};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleModel {
    pub id: Option<i64>,
    pub symbol: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl FromRow<'_, SqliteRow> for CandleModel {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let symbol = row.try_get("symbol")?;
        let source = row.try_get("source")?;
        let timestamp: DateTime<Utc> = row.try_get("timestamp")?;
        let open = row.try_get("open")?;
        let high = row.try_get("high")?;
        let low = row.try_get("low")?;
        let close = row.try_get("close")?;
        let volume = row.try_get("volume")?;
        
        Ok(Self {
            id,
            symbol,
            source,
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        })
    }
}

impl From<Candle> for CandleModel {
    fn from(candle: Candle) -> Self {
        Self {
            id: None,
            symbol: candle.symbol.clone(),
            source: candle.source.clone(),
            timestamp: candle.timestamp,
            open: candle.open,
            high: candle.high,
            low: candle.low,
            close: candle.close,
            volume: candle.volume,
        }
    }
}

impl From<CandleModel> for Candle {
    fn from(model: CandleModel) -> Self {
        Self {
            symbol: model.symbol,
            source: model.source,
            timestamp: model.timestamp,
            open: model.open,
            high: model.high,
            low: model.low,
            close: model.close,
            volume: model.volume,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductModel {
    pub id: Option<i64>,
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub source: String,
}

impl FromRow<'_, SqliteRow> for ProductModel {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let symbol = row.try_get("symbol")?;
        let name = row.try_get("name")?;
        let asset_type = row.try_get("asset_type")?;
        let source = row.try_get("source")?;
        
        Ok(Self {
            id,
            symbol,
            name,
            asset_type,
            source,
        })
    }
}

impl From<Product> for ProductModel {
    fn from(product: Product) -> Self {
        Self {
            id: None,
            symbol: product.symbol,
            name: product.name,
            asset_type: product.asset_type,
            source: product.source,
        }
    }
}

impl From<ProductModel> for Product {
    fn from(model: ProductModel) -> Self {
        Self {
            symbol: model.symbol,
            name: model.name,
            asset_type: model.asset_type,
            source: model.source,
        }
    }
}