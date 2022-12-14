use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct PersonalData{
    name: String,
    address: String,
}

type PersonalDataBase = HashMap<String, PersonalData>;

#[test]
fn test_000(){
    let name = "Tom".to_string();
    let data = PersonalData{name: name.clone(), address: "Tokyo".to_string()};
    let mut map = HashMap::new();
    map.insert(&name, &data);
    assert_eq!(map.get(&"Tom".to_string()).unwrap().address, "Tokyo".to_string());
    assert_eq!(map.get(&"Sam".to_string()), None);
}

#[test]
fn test_json(){
    let data = PersonalData{name: "Tom".to_string(), address: "Tokyo".to_string()};
    let json = serde_json::to_string(&data).unwrap();
    assert_eq!(json, r#"{"name":"Tom","address":"Tokyo"}"#);
    assert_eq!(serde_json::from_str::<PersonalData>(&json).unwrap(), data);
}

#[test]
fn test_xml(){
    let data = PersonalData{name: "Tom".to_string(), address: "Tokyo".to_string()};
    let xml = serde_xml_rs::to_string(&data).unwrap();
    assert_eq!(xml, r#"<?xml version="1.0" encoding="UTF-8"?><PersonalData><name>Tom</name><address>Tokyo</address></PersonalData>"#);
    assert_eq!(serde_xml_rs::from_str::<PersonalData>(&xml).unwrap(), data);
}