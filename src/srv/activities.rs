use std::collections::HashMap;
use std::any::Any;

extern crate csv;
use csv::StringRecord;
use std::fs;
use rocket::request::Form;
use rocket_contrib::json::Json;
#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: u128, pub user_id: u128, pub r#type: String, pub start_time: String, pub duration: String, pub satisfaction: String
}


pub fn get(user_id: String, obj1: &HashMap<&str, Box<dyn Any + 'static>>) -> Vec<StringRecord> {
	let get_all = {
		let mut out: Option<&bool> = None;
		let entry = (obj1).get("get_all");
		if let Some(pointer) = entry {
		    out = pointer.downcast_ref::<bool>();
		}
		out
	};
	let mut out = vec! [];
	let mut handle = csv::Reader::from_path("./activity.csv").unwrap();
	for result in handle.records() {
		let record = result.expect("a csv record");
		if (record[1] == user_id) || !get_all.is_none() {
			out.push(record);
		}
	}
	return out;
}

pub fn add(entry: Json<Activity>) {
	let mut handle = csv::Writer::from_writer(fs::OpenOptions::new().append(true).open("activity.csv").expect("a file"));
	handle.write_record(&(vec! [
			entry.id.to_string(),
			entry.user_id.to_string(),
			entry.r#type.to_string(), 
            entry.start_time.to_string(),
            entry.duration.to_string(), 
            entry.satisfaction.to_string()
        ])).expect("an entry");
	handle.flush().expect("a write to the db");
}
