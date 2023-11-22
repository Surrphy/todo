use std::str::FromStr;

use anyhow::{Result, anyhow};
use persy::{Persy, PersyId};


/// Adding todo record to the persy database
///
/// Todo stores descriptions as a vector of bytes, last one being "done" byte
///
/// # Errors 
///
/// Returns `PersyError`
pub fn add(persy_db: &Persy, desc: &str, done: bool) -> Result<String> {
    let mut tx = persy_db.begin()?;

    let desc = desc.as_bytes();

    let mut data = vec![0; desc.len()+1];

    let len = data.len();

    for (i, value) in desc.iter().enumerate() {
        data[i] = *value;
    }

    data[len-1] = u8::from(done);

    tx.insert("todos", &data)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    list(persy_db)
}

/// Changing the "done" byte to the specified value
///
/// Todo stores descriptions as a vector of bytes, last one being "done" byte
///
/// # Errors
///
/// Returns `PersyError`
pub fn change_done(persy_db: &Persy, index: &str, done: bool) -> Result<String> {
    let mut data = get_record(persy_db, index)?;
    let len = data.len();

    data[len-1] = u8::from(done);

    let mut tx = persy_db.begin()?;

    tx.update("todos", &PersyId::from_str(index)?, &data)?;

    let prepared = tx.prepare()?;
    prepared.commit()?;

    list(persy_db)
}

/// Deleting todo record with specified Persy index
///
/// Todo stores descriptions as a vector of bytes, last one being "done" byte
///
/// # Errors
///
/// Returns `PersyError`
pub fn delete(persy_db: &Persy, index: &str) -> Result<String> {
    let mut tx = persy_db.begin()?;

    tx.delete("todos", &PersyId::from_str(index)?)?;
    

    let prepared = tx.prepare()?;
    prepared.commit()?;

    list(persy_db)
}


/// List all todos stored in the Persy database
///
/// Todo stores descriptions as a vector of bytes, last one being "done" byte
///
/// # Errors
///
/// Returns `PersyError`
pub fn list(persy_db: &Persy) -> Result<String> {
    let mut result = String::new();

    for (id, mut content) in persy_db.scan("todos")? {
        let Some(done) = content.pop() else {
            return Err(anyhow!("Record was not formated correctly (Record empty)")) 
        };

        let Ok(desc) = std::str::from_utf8(&content) else {
            return Err(anyhow!("Description was not formatted correctly"))
        };

        result += &format!("{} [{}] {}\n", id, if done == 0 {" "} else {"X"}, desc);
    }

    Ok(result)
}

fn get_record(persy_db: &Persy, index: &str) -> Result<Vec<u8>> {
    let mut result = Vec::new();

    for (i, content) in persy_db.scan("todos")? {
        if index == i.to_string() {
            result = content;
        }
    }

    Ok(result)
}
