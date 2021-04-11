extern crate csv;
use csv::StringRecord;
use std::fs;

use rocket::request::Form;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, FromForm)]
pub struct Todo {
    pub id: u128, pub user_id: u128, pub name: String, pub expected_duration: String, pub priority: String
}


pub fn get(user_id: String) -> Vec<StringRecord> {
	let mut out = vec! [];
	let mut handle = csv::Reader::from_path("./todos.csv").unwrap();
	for result in handle.records() {
        let record = result.expect("a csv record");
		if record[1].to_string() == user_id {
			out.push(record);
		}
    }
	return out;
}

pub fn add(entry: Json<Todo>) {
	let mut handle = csv::Writer::from_writer(fs::OpenOptions::new().append(true).open("todos.csv").expect("a file"));
	handle.write_record(&vec![entry.id.to_string(), entry.user_id.to_string(), entry.name.to_string(), entry.expected_duration.to_string(), entry.priority.to_string()]).expect("an entry");
	handle.flush().expect("a write to the db");
}

