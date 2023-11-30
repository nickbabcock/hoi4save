use crate::{de::deserialize_vec_pair, CountryTag, Hoi4Date};
use jomini::JominiDeserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(JominiDeserialize, Debug, Clone, Serialize)]
pub struct Hoi4Save {
    pub player: String,
    pub date: Hoi4Date,
    #[jomini(default, deserialize_with = "deserialize_vec_pair")]
    pub countries: Vec<(CountryTag, Country)>,
}

#[derive(JominiDeserialize, Debug, Clone, Serialize)]
pub struct Country {
    #[jomini(default)]
    pub stability: f64,
    #[jomini(default)]
    pub war_support: f64,
    #[jomini(default)]
    pub variables: HashMap<String, f64>,
}
