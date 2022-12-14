use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct PersonalData{
    name: String,
    address: String,
    work_experience : Vec<String>,
}

type PersonalDataBase = HashMap<String, PersonalData>;

#[test]
fn test_000(){
    let name = "Tom".to_string();
    let data = PersonalData{name: name.clone(), 
                                          address: "Tokyo".to_string(),
                                          work_experience : vec!["Dei INC., Tokyo January 2020 - present".to_string(), "JAPAN OFFICE LTD., Kanagawa April 2018 - December 2019".to_string()]};
    let mut map = HashMap::new();
    map.insert(&name, &data);
    assert_eq!(map.get(&"Tom".to_string()).unwrap().address, "Tokyo".to_string());
    assert_eq!(map.get(&"Sam".to_string()), None);
}

#[test]
fn test_json(){
    let data = PersonalData{name: "Tom".to_string(), 
                                          address: "Tokyo".to_string(),
                                          work_experience : vec!["Dei INC., Tokyo January 2020 - present".to_string(), "JAPAN OFFICE LTD., Kanagawa April 2018 - December 2019".to_string()]};
    let json = serde_json::to_string(&data).unwrap();
    assert_eq!(json, r#"{"name":"Tom","address":"Tokyo","work_experience":["Dei INC., Tokyo January 2020 - present","JAPAN OFFICE LTD., Kanagawa April 2018 - December 2019"]}"#);
    assert_eq!(serde_json::from_str::<PersonalData>(&json).unwrap(), data);
}

#[test]
fn test_xml(){
    let data = PersonalData{name: "Tom".to_string(), 
                                          address: "Tokyo".to_string(),
                                          work_experience : vec!["Dei INC., Tokyo January 2020 - present".to_string(), "JAPAN OFFICE LTD., Kanagawa April 2018 - December 2019".to_string()]};
    let xml = serde_xml_rs::to_string(&data).unwrap();
    assert_eq!(xml, r#"<?xml version="1.0" encoding="UTF-8"?><PersonalData><name>Tom</name><address>Tokyo</address><work_experience>Dei INC., Tokyo January 2020 - present</work_experience><work_experience>JAPAN OFFICE LTD., Kanagawa April 2018 - December 2019</work_experience></PersonalData>"#);
    assert_eq!(serde_xml_rs::from_str::<PersonalData>(&xml).unwrap(), data);
}