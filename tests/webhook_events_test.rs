use axum::http::StatusCode;
use serde_json::json;
mod common;

#[tokio::test]
async fn health_check() {
    common::server()
        .await
        .get("/health")
        .await
        .assert_status_ok();
}

#[tokio::test]
async fn invalid_webhook_format() {
    let res = common::server()
        .await
        .post("/webhook")
        .json(&json!({"not": "an array"}))
        .await;
    res.assert_status(StatusCode::BAD_REQUEST);
    assert_eq!(res.json::<serde_json::Value>()["error"], "invalid format");
}

#[tokio::test]
async fn webhook_empty_array() {
    let res = common::server()
        .await
        .post("/webhook")
        .json(&json!([]))
        .await;
    res.assert_status_ok();
    assert_eq!(res.json::<serde_json::Value>()["saved events"], 0);
    assert_eq!(res.json::<serde_json::Value>()["skipped events"], 0);
}

#[tokio::test]
async fn webhook_swap_event() {
    let res = common::server()
        .await
        .post("/webhook")
        .json(&json!([{
            "type": "SWAP",
            "source": "JUPITER",
            "signature": "sig_swap_001",
            "timestamp": 1700000000,
            "transactionError": null,
            "feePayer": "wallet_abc",
            "tokenTransfers": [
                { "mint": "mintA", "tokenAmount": 5.0 },
                { "mint": "mintB", "tokenAmount": 10.0 }
            ]
        }]))
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert_eq!(body["saved events"], 1);
    assert_eq!(body["skipped events"], 0);
}

#[tokio::test]
async fn webhook_token_transfer_event() {
    let res = common::server()
        .await
        .post("/webhook")
        .json(&json!([{
            "signature": "sig_token_001",
            "timestamp": 1700000001,
            "transactionError": null,
            "tokenTransfers": [
                {
                    "fromUserAccount": "wallet_xyz",
                    "tokenAmount": 3.5,
                    "mint": "mintC"
                }
            ],
            "nativeTransfers": []
        }]))
        .await;
    res.assert_status_ok();
    assert_eq!(res.json::<serde_json::Value>()["saved events"], 1);
}

#[tokio::test]
async fn get_all_events() {
    let res = common::server().await.get("/events").await;
    res.assert_status_ok();
    assert!(res.json::<serde_json::Value>().is_array());
}

#[tokio::test]
async fn get_events_by_wallet() {
    common::server()
        .await
        .get("/events/some_wallet")
        .await
        .assert_status_ok();
}

#[tokio::test]
async fn get_events_by_token_mint() {
    common::server()
        .await
        .get("/events/mint/some_mint")
        .await
        .assert_status_ok();
}
