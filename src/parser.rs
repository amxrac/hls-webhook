use crate::models::trigger_events::TriggerType;
use crate::schema::trigger_events_schema::NewTriggerEvent;
use chrono::{DateTime, Utc};
use serde_json::Value;

pub fn parse_event(tx: &Value) -> Option<NewTriggerEvent> {
    let tx_type = tx.get("type").and_then(|i| i.as_str());
    let source = tx.get("source").and_then(|i| i.as_str());
    let signature = tx.get("signature").and_then(|i| i.as_str())?.to_string();
    let ts = tx.get("timestamp").and_then(|i| i.as_i64())?;
    let timestamp = DateTime::<Utc>::from_timestamp(ts, 0)?;

    if tx.get("transactionError").map_or(false, |i| !i.is_null()) {
        return None;
    }

    if tx_type == Some("SWAP") || source == Some("JUPITER") {
        return parse_swap(tx, signature, timestamp);
    }

    let token_transfers = tx.get("tokenTransfers").and_then(|i| i.as_array())?;
}

pub fn parse_swap(
    tx: &Value,
    signature: String,
    timestamp: DateTime<Utc>,
) -> Option<NewTriggerEvent> {
    let wallet = tx.get("feePayer").and_then(|i| i.as_str())?.to_string();
    let token_transfers = tx.get("tokenTransfers").and_then(|i| i.as_array())?;
    if token_transfers.is_empty() {
        return None;
    }
    // TODO: test w actual swap event and maybe add input mints
    // let input = token_transfers.first()?;
    let output = token_transfers.last()?;

    // let input_mint = input["mint"].as_str().map(|s| s.to_string());
    // let input_value = input["tokenAmount"].as_f64().unwrap_or(0.0);

    let output_mint = output["mint"].as_str().map(|s| s.to_string());
    let output_value = output["tokenAmount"].as_f64().unwrap_or(0.0);

    Some(NewTriggerEvent {
        trigger_type: TriggerType::Swap,
        wallet,
        value: output_value,
        token_mint: output_mint,
        timestamp,
        tx_signature: signature,
    })
}
