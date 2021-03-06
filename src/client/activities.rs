#![feature(proc_macro_hygiene)]
use render::{rsx, html, raw, Render, component, SimpleElement};
use std::fs;

use rand;

use crate::client::_page::{Page};
use crate::srv::todos;

extern crate serde_derive;


pub fn index(user_id: String) -> String {
    let todo_raw = todos::get(user_id.to_string());
    let todo_data = todo_raw.iter().map(|r| r.deserialize(None).unwrap()).collect::<Vec<todos::Todo>>();
    let todo_strings = todo_data.iter().map(|d| format!(
        "<> {} § {} § {} </>"
    , d.name, d.expected_duration, d.priority)).collect::<Vec<String>>();
    let page = rsx! {
        <Page title = { "Done !" }>
            <h2> { "You're done with:" } </h2>
            <p id = { "user_id" }> { "User:" } { user_id } </p>
            <p id = { "id" }> { "Accomplishment: " } { rand::random::<u128>().to_string() } </p>
            <form>
                <p> { "Name:" } </p>
                <input id = { "type" }></input>
                <p> { "Start time (24h format):" } </p>
                <input id = { "start_time" } > </input>
                <p> { "Actual duration (hours):" } </p>
                <input id = { "duration" } > </input>
                <p> { "Satisfaction, how you feel now (1 to 10):" } </p>
                <input id = { "satisfaction" } > </input> 
            </form>
            <button id = { "submit_activity" }> { "Submit Accomplishment" } </button>
            <script> { fs::read_to_string("./public/index.js").unwrap() } </script> 
        </Page>
    };
	page.render() //.replace("<p>", "").replace("</p>", "").replace("\"", "")
}