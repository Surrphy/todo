use std::str::FromStr;

use anyhow::Result;
use persy::{Persy, PersyId};

pub fn add(persy_db: Persy, desc: String, done: bool) -> Result<String> {
    let mut tx = persy_db.begin()?;

    let desc = desc.as_bytes();

    let mut data = vec![0; desc.len()+1];

    let len = data.len();

    for (i, value) in desc.iter().enumerate() {
        data[i] = *value;
    }

    data[len-1] = done as u8;

    tx.insert("todos", &data)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    list(persy_db)
}

pub fn change_done(persy_db: Persy, index: String, done: bool) -> Result<String> {
    let mut data = get_record(&persy_db, index.clone())?;
    let len = data.len();

    data[len-1] = done as u8;

    let mut tx = persy_db.begin()?;

    tx.update("todos", &PersyId::from_str(&index).unwrap(), &data)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    list(persy_db)
}

pub fn delete(persy_db: Persy, index: String) -> Result<String> {
    let mut tx = persy_db.begin().unwrap();

    tx.delete("todos", &PersyId::from_str(&index)?)?;
    

    let prepared = tx.prepare()?;
    prepared.commit()?;

    list(persy_db)
}

pub fn list(persy_db: Persy) -> Result<String> {
    let mut result = String::new();

    for (id, mut content) in persy_db.scan("todos")? {
        let done = content.pop().unwrap() != 0;
        result += &format!("{} [{}] {}\n", id, if done {"X"} else {" "}, std::str::from_utf8(&content).unwrap());
    }

    Ok(result)
}

fn get_record(persy_db: &Persy, index: String) -> Result<Vec<u8>> {
    let mut result = Vec::new();

    for (i, content) in persy_db.scan("todos")? {
        if index == i.to_string() {
            result = content;
        }
    }

    Ok(result)
}
