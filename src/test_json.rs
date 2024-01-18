use std::collections::HashMap;
use serde_json::{Map, Value};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct MyStruct {
    data: HashMap<String, String>,
    status: i32
}
fn map_to_json(map: HashMap<String, String>) -> String {
    let my_struct = MyStruct { data: map, status: 200 };
    let json_string = serde_json::to_string(&my_struct).unwrap();
    json_string
}
pub fn get_response() -> String
{
    let mut my_map = HashMap::new();
    my_map.insert("volume".to_string(), "50".to_string());
    my_map.insert("is_muted".to_string(), "true".to_string());
    my_map.insert("os".to_string(), "darwin".to_string());
    let json_result = map_to_json(my_map);
    return json_result


}
