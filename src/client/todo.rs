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
        <Page title = { "TODO" }>
            <table>
                <thead>
                    <tr>
                        <th colspan={"3"}> { "Here are your current todos" } </th>
                    </tr>
                    <tr>
                        <th> { "Name" } </th>
                        <th> { "Expected Duration" } </th>
                        <th> { "Priority" } </th>
                    </tr>
                </thead>
                <tbody>
                    <tr> <td> { todo_strings.join("\n") } </td> </tr>
                </tbody>
            </table>
            <h2> { "Add a new one" } </h2>
            <p id = { "user_id" }> { "User: " } { user_id } </p>
            <p id = { "id" }> { "Todo: " } { rand::random::<u128>().to_string() } </p>
            <form>
                <p> { "Name:" } </p>
                <input id = { "name" }></input>
                <p> { "Expected duration (hours):" } </p>
                <input id = { "expected_duration" } > </input>
                <p> { "Priority:" } </p>
                <input id = { "priority" } > </input> 
            </form>
            <button id = { "submit_todo" }> { "Submit Todo" } </button>
            <script> { fs::read_to_string("./public/index.js").unwrap() } </script> 
        </Page>
    };
	page.render() //.replace("<p>", "").replace("</p>", "").replace("\"", "")
}