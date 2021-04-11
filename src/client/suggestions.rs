#![feature(proc_macro_hygiene)]
use render::{rsx, raw, Render, component};
use std::fs;
use crate::client::_page::{Page};
use crate::srv::activities;
use csv::StringRecord;
pub fn index(user_id: String) -> String {
    let suggestions = get_suggestions(user_id);
        let page = rsx! {
        <Page title = { "Suggestions" }>
            <script> { fs::read_to_string("./public/index.js").unwrap() } </script>
            <h1> { "Personal Suggestions" } </h1>
            <h2> { "Most likely for you:" } </h2>
            <p> { suggestions[0].to_string() } </p>
            <h2> { "You could do at some point:" } </h2>
            <p> { suggestions[1].to_string() } </p>
            <h2> { "To try right now:" } </h2>
            <p> { suggestions[2].to_string() } </p>
        </Page>
    };
        page.render()
}



use std::collections::HashMap;
use std::any::Any;


fn get_suggestions(user_id: String) -> Vec<String> {
	let personal_data = activities::get(user_id.to_string(), &({
	let mut object: HashMap<&str, Box<dyn Any>> = HashMap::new();
	object
}));
	let all_data = activities::get(user_id.to_string(), &({
	let mut object: HashMap<&str, Box<dyn Any>> = HashMap::new();
	object.insert("get_all", Box::new(true));
	object
}));
	let for_you = personal_data.iter().map(|a| (a, a[5].parse::<f32>().unwrap())).max_by(|a, b| a.1.partial_cmp(&(b.1)).expect("a comp")).unwrap_or((&(StringRecord::from(vec! ["", "", "You haven't regietered anything, I can't guess"])), 0.0)).0[2].to_string();
	let eventually = all_data.iter().map(|a| (a, a[5].parse::<f32>().unwrap())).max_by(|a, b| a.1.partial_cmp(&(b.1)).expect("a comp")).unwrap().0[2].to_string();
	let right_now = "Vote for me ? idk this feature's not in :(".to_string();
	return vec! [
        for_you, eventually, right_now
    ];
}

