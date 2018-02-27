
extern crate i3blocks;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


const BTCTURK_URL: &'static str = "https://www.btcturk.com/api/ticker";

#[derive(Serialize, Deserialize)]
struct BtcTurkData {
    pair: String,
    high: f32,
    last: f32,
    timestamp: f32,
    bid: f32,
    volume: f32,
    low: f32,
    ask: f32,
    open: f32,
    average: f32,
    daily: f32,
    #[serde(rename = "dailyPercent")]
    daily_percent: f32,
}


fn main() {
    i3blocks::Script::new().on_refresh(|_| {
        reqwest::get(BTCTURK_URL).ok()
            .and_then(|mut r| r.text().ok())
            .and_then(|t| parse_btcturk_data(&t))
            .map(|btc| {
                if btc.last > 39000. {
                    format!("<span color='#B85335'>\u{f15a} {}</span>", btc.last)
                } else if btc.last < 36500. {
                    format!("<span color='3bb835'>\u{f15a} {}</span>", btc.last)
                } else {
                    format!("\u{f15a} {}", btc.last)
                }
            })
    });
}


fn parse_btcturk_data(data: &str) -> Option<BtcTurkData> {
    let v: Option<Vec<BtcTurkData>> = serde_json::from_str(data).ok();
    v.and_then(|v| v.into_iter().filter(|d| d.pair == "BTCTRY").next())
}


#[test]
fn test_btcturk_parse_btcturk_data() {
    const BTCTURK_EXAMPLE_DATA: &'static str = "[{\"pair\":\"BTCTRY\",\"high\":40490.0,\
        \"last\":40005.0,\"timestamp\":1519741789.0,\"bid\":40001.0,\
        \"volume\":452.92,\"low\":38251.0,\"ask\":40005.0,\"open\":38756.0,\
        \"average\":39432.46,\"daily\":0.0,\"dailyPercent\":0.0},\
        {\"pair\":\"ETHBTC\",\"high\":0.0,\"last\":0.0,\"timestamp\":1519741789.0,\
        \"bid\":0.0,\"volume\":0.0,\"low\":0.0,\"ask\":0.0,\"open\":0.0,\
        \"average\":0.0,\"daily\":0.0,\"dailyPercent\":0.0},\
        {\"pair\":\"ETHTRY\",\"high\":3365.0,\"last\":3320.0,\
        \"timestamp\":1519741789.0,\"bid\":3306.0,\"volume\":1339.64,\
        \"low\":3266.0,\"ask\":3320.0,\"open\":3321.0,\"average\":3312.67,\
        \"daily\":0.0,\"dailyPercent\":0.0},{\"pair\":\"XRPTRY\",\
        \"high\":3.67,\"last\":3.56,\"timestamp\":1519741789.0,\
        \"bid\":3.54,\"volume\":435798.22,\"low\":3.47,\"ask\":3.55,\
        \"open\":3.53,\"average\":3.55,\"daily\":0.0,\
        \"dailyPercent\":0.0}]";
    assert!(parse_btcturk_data(BTCTURK_EXAMPLE_DATA).is_some());
}
