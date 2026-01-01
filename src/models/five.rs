use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct FiveMinute {
    avg_high_price: Option<i32>,
    high_price_volume: Option<i32>,
    avg_low_price: Option<i32>,
    low_price_volume: Option<i32>
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
struct FiveMinuteData {
    data: HashMap<i32, FiveMinute>
}
