use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct LatestSales {
    pub data: HashMap<i32, LatestSale>
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LatestSale {
    pub high: Option<i32>,
    pub high_time: Option<i32>,
    pub low: Option<i32>,
    pub low_time: Option<i32>
}
