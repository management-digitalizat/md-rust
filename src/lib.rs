use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen]
    fn get_from_local_storage(key: &str) -> JsValue;
}


#[derive(Debug, Deserialize, Serialize)]
#[wasm_bindgen]
struct Room {
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_nao: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_Qu1: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_V: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_p: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_Cp: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_E: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_i: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_L: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_v: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qi_Qu2: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_Ap: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_Abc: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_Abcj: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_Rp: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_Rbc: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_0i: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_0e: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_0ej: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_0p: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_CM: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qs_ns: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qt_CM: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qt_m: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qt_A: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qt_0i: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qt_0e: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    Qt_Rprim: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    G_Ao: f64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    G_Ac: f64,
}


fn read_all_from_local_storage() -> HashMap<String, Room> where
    Room: serde::de::DeserializeOwned, {
    log("WASM -> Load from local storage");
    let local_storage = window()
        .unwrap()
        .local_storage()
        .unwrap().unwrap();
    let value = local_storage.get_item("md_necesar_caldura");
    log("WASM -> Loaded from local storage");
    let json_value = value.unwrap().unwrap();

    match serde_json::from_str::<HashMap<String, Room>>(json_value.as_str()) {
        Ok(mp) => {
            mp
        }
        Err(err) => {
            // Print the error message
            log(err.to_string().as_str());
            HashMap::new()
        }
    }
}

fn deserialize_string_to_f64<'de    , D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

#[wasm_bindgen]
pub fn calculate_qs(room_data: &Room) -> f64 {
    let Ap = room_data.Qs_Ap.clone();
    let Abc = room_data.Qs_Abc.clone();
    let Abcj = room_data.Qs_Abcj.clone();
    let Rp = room_data.Qs_Rp.clone();
    let Rbc = room_data.Qs_Rbc.clone();
    let _0i = room_data.Qs_0i.clone();
    let _0e = room_data.Qs_0e.clone();
    let _0ej = room_data.Qs_0ej.clone();
    let _0p = room_data.Qs_0p.clone();
    let CM = room_data.Qs_CM.clone();
    let ns = room_data.Qs_ns.clone();


    let qs = ((Ap * (_0i - _0p)) / Rp) + (CM * ((_0i - _0e) / Rbc) * Abc) + ((1.0 / ns) * ((_0i - _0ej) / Rbc) * Abcj);

    round(qs, 4)
}

#[wasm_bindgen]
pub fn calculate_qi(room_data: &Room) -> f64 {
    let ac = room_data.G_Ac.clone();

    // Values from Qs
    let CM = room_data.Qs_CM.clone();
    let _0i = room_data.Qs_0i.clone();
    let _0e = room_data.Qs_0e.clone();

    // Qi specific
    let nao = room_data.Qi_nao.clone();
    let v_upper = room_data.Qi_V.clone();
    let p = room_data.Qi_p.clone();
    let cp = room_data.Qi_Cp.clone();
    let e = room_data.Qi_E.clone();
    let i = room_data.Qi_i.clone();
    let l = room_data.Qi_L.clone();
    let v = room_data.Qi_v.clone();
    let qu1 = room_data.Qi_Qu1.clone();
    let qu2 = room_data.Qi_Qu2.clone();

    let qi1 = ((nao * CM * v_upper * p * cp * (_0i - _0e) + qu1) * (1.0 + ac / 100.0)).round();
    let qi2 = ((CM * e * i * l * v.powf(4.0 / 3.0) * (_0i - _0e) + qu2) * (1.0 + ac / 100.0)).round();

    let qi = qi1.max(qi2);
    qi
}

#[wasm_bindgen]
pub fn calculate_qt(room_data: &Room, calculated_qs: f64) -> f64 {
    let CM = room_data.Qt_CM.clone();
    let m = room_data.Qt_m.clone();
    let A = room_data.Qt_A.clone();
    let _0i = room_data.Qt_0i.clone();
    let _0e = room_data.Qt_0e.clone();
    let Rprim = room_data.Qt_Rprim.clone();

    let Qt = (((CM * m * A * (_0i - _0e)) / Rprim) + calculated_qs).round();
    Qt
}

#[wasm_bindgen]
pub fn calculate_values() -> String {
    let mut results: HashMap<String, serde_json::Value> = HashMap::new();

    let rooms = read_all_from_local_storage();
    log("WASM -> Calculating values for rooms");
    for (room_name, room_data) in rooms.iter() {
        let qs = calculate_qs(room_data);
        let result = json!({
            "Qs": qs,
            "Qi": calculate_qi(room_data),
            "Qt": calculate_qt(room_data, qs)
        });
        results.insert(room_name.clone(), result);
    }
    serde_json::to_string(&results).unwrap()
}
