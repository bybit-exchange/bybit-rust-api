//! End-to-end smoke verification of public REST endpoints against Bybit mainnet.
//!
//! Run with: `cargo run --example verify_public_apis`

use bybit_api::{BybitClient, Category, Interval, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Bybit public REST smoke test (mainnet)\n");
    let client = BybitClient::with_credentials("", "")?;
    let mut passed = 0;
    let mut failed = 0;

    macro_rules! check {
        ($name:expr, $fut:expr, $body:expr) => {{
            match $fut.await {
                Ok(v) => {
                    let detail: String = ($body)(&v);
                    println!("  PASS  {}: {}", $name, detail);
                    passed += 1;
                }
                Err(e) => {
                    println!("  FAIL  {}: {}", $name, e);
                    failed += 1;
                }
            }
        }};
    }

    check!(
        "get_server_time",
        client.get_server_time(),
        |t: &bybit_api::ServerTime| { format!("time_second={}", t.time_second) }
    );

    check!(
        "get_tickers(Linear, BTCUSDT)",
        client.get_tickers(Category::Linear, Some("BTCUSDT")),
        |t: &bybit_api::Tickers| match t.list.first() {
            Some(tk) => format!("last={}, mark={}", tk.last_price, tk.mark_price),
            None => "EMPTY".into(),
        }
    );

    check!(
        "get_orderbook(Linear, BTCUSDT, 25)",
        client.get_orderbook(Category::Linear, "BTCUSDT", Some(25)),
        |ob: &bybit_api::Orderbook| format!("{}x{} levels", ob.b.len(), ob.a.len())
    );

    check!(
        "get_klines(Linear, BTCUSDT, 1h, 5)",
        client.get_klines(Category::Linear, "BTCUSDT", Interval::H1, Some(5)),
        |k: &bybit_api::Klines| format!("{} bars", k.list.len())
    );

    check!(
        "get_recent_trades(Linear, BTCUSDT, 10)",
        client.get_recent_trades(Category::Linear, "BTCUSDT", Some(10)),
        |rt: &bybit_api::RecentTrades| format!("{} trades", rt.list.len())
    );

    check!(
        "get_instruments_info(Linear, BTCUSDT)",
        client.get_instruments_info(Category::Linear, Some("BTCUSDT")),
        |i: &bybit_api::InstrumentsInfo| match i.list.first() {
            Some(inst) => format!("symbol={}, status={}", inst.symbol, inst.status),
            None => "EMPTY".into(),
        }
    );

    check!(
        "get_funding_history(Linear, BTCUSDT, 3)",
        client.get_funding_history(Category::Linear, "BTCUSDT", Some(3)),
        |f: &bybit_api::FundingHistory| format!("{} records", f.list.len())
    );

    println!("\n=== SUMMARY ===");
    println!("Passed: {} / {}", passed, passed + failed);
    if failed > 0 {
        std::process::exit(1);
    }
    Ok(())
}
