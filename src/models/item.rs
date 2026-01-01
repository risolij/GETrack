use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub examine: String,
    pub id: i32,
    pub members: bool,
    pub lowalch: Option<i32>,
    pub limit: Option<i32>,
    pub value: i32,
    pub highalch: Option<i32>,
    pub icon: String,
    pub name: String 
}

