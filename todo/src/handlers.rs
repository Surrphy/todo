use std::str::FromStr;

use persy::{Persy, PersyId};

pub fn add(persy_db: Persy, desc: String, done: bool) -> String {
    let mut tx = persy_db.begin().unwrap();

    //tx.create_segment("todos").unwrap();

    let desc = desc.as_bytes();

    let mut data = vec![0; desc.len()+1];

    let len = data.len();

    for (i, value) in desc.iter().enumerate() {
        data[i] = *value;
    }

    data[len-1] = done as u8;

    tx.insert("todos", &data).unwrap();

    let prepared = tx.prepare().unwrap();
    prepared.commit().unwrap();

    String::from("Ok")
}

pub fn change_done(persy_db: Persy, index: String, done: bool) -> String {
    let mut data = get_record(&persy_db, index.clone());
    let len = data.len();

    data[len-1] = done as u8;

    let mut tx = persy_db.begin().unwrap();

    tx.update("todos", &PersyId::from_str(&index).unwrap(), &data).unwrap();

    let prepared = tx.prepare().unwrap();
    prepared.commit().unwrap();

    String::from("Ok")
}

pub fn list(persy_db: Persy) -> String {
    let mut result = String::new();

    for (id, mut content) in persy_db.scan("todos").unwrap() {
        let done = content.pop().unwrap() != 0;
        result += &format!("{} [{}] {}\n", id, if done {"X"} else {" "}, std::str::from_utf8(&content).unwrap());
    }

    result
}

pub fn delete(persy_db: Persy, index: String) -> String {
    let mut tx = persy_db.begin().unwrap();

    tx.delete("todos", &PersyId::from_str(&index).unwrap()).unwrap();
    

    let prepared = tx.prepare().unwrap();
    prepared.commit().unwrap();

    String::from("Ok")
}

fn get_record(persy_db: &Persy, index: String) -> Vec<u8> {
    let mut result = Vec::new();

    for (i, content) in persy_db.scan("todos").unwrap() {
        if index == i.to_string() {
            result = content;
        }
    }

    result
}
