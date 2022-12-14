use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct PersonalData{
    name: String,
    address: String,
}

type PersonalDataBase = HashMap<String, PersonalData>;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_000(){
    let name = "Tom".to_string();
    let data = PersonalData{name: name.clone(), address: "Tokyo".to_string()};
    let mut map = HashMap::new();
    map.insert(&name, &data);
    assert_eq!(map.get(&"Tom".to_string()).unwrap().address, "Tokyo".to_string());
    assert_eq!(map.get(&"Sam".to_string()), None);
}