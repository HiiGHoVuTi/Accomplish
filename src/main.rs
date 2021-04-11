#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::{
    content::Html,
    Flash, Redirect
};
use rocket::Request;
use rocket::http::{Cookies, Cookie};

extern crate rocket_contrib;
use rocket_contrib::json::Json;

use render::{rsx, Render};

use std::fs;

extern crate rand;

extern crate csv;
#[macro_use]
extern crate serde_derive;

extern crate gbdt;


mod client {
    pub mod index;
    pub mod todo;
    pub mod activities;
    pub mod suggestions;
    pub mod err404;
    pub mod _page;
}
mod srv {
    pub mod todos;
    pub mod activities;

    pub mod ml;
}

use crate::srv::todos::{Todo};
use crate::srv::todos;

use crate::srv::activities::Activity;
use crate::srv::activities;


fn format_and_HTML(txt: String) -> Html<String> {
	return Html(txt.replace("&quot;", "\"").replace("&gt;", ">").replace("&lt;", "<").replace("&amp;", "&").replace("&apos;", "'"));
}

#[catch(404)]

fn not_found(req: &Request) -> Html<String> {
	format_and_HTML(client::err404::index(format! ("{}", req.uri())))
}

#[get("/public/<name>")]

fn files(name: String) -> Html<String> {
	let raw = fs::read_to_string(format! ("/client/public/{}", name));
	if let Ok(value) = raw {
		return format_and_HTML(value);
	}
	else{
	return format_and_HTML(client::err404::index(name));
	}
}

#[get("/")]

fn index() -> Html<String> {
	format_and_HTML(client::index::index())
}

#[get("/login")]

fn login(mut cookies: Cookies) -> Flash<Redirect> {
	if let Some(user_id) = cookies.get_private("user_id") {
		return Flash::success(Redirect::to("/"), "Already logged in");
	}
	else{
	let uid = rand::random::<u128>().to_string();
	cookies.add_private(Cookie::new("user_id", uid.to_string()));
	let mut handle = csv::Writer::from_writer(fs::OpenOptions::new().append(true).open("users.csv").expect("a file"));
	handle.write_record(&([uid])).expect("an entry");
	handle.flush().expect("a write to the db");
	return Flash::success(Redirect::to("/"), "Logged in, created account");
	}
}

#[get("/home")]

fn home() -> Flash<Redirect> {
	Flash::success(Redirect::to("/"), "redirected to home")
}

#[get("/suggestions")]

fn suggestions(mut cookies: Cookies) -> Html<String> {
	if let Some(user_id) = cookies.get_private("user_id") {
		format_and_HTML(client::suggestions::index(user_id.value().to_string()))
	}
	else{
	format_and_HTML("you're not logged in, please <a href = '/login'>login here</a>".to_string())
	}
}

#[get("/todo")]

fn todo(mut cookies: Cookies) -> Html<String> {
	if let Some(user_id) = cookies.get_private("user_id") {
		format_and_HTML(client::todo::index(user_id.value().to_string()))
	}
	else{
	format_and_HTML("you're not logged in, please <a href = '/login'>login here</a>".to_string())
	}
}

#[post("/todoadd", format = "application/json", data = "<todo>")]

fn todo_add(mut cookies: Cookies, todo: Json<Todo>) {
	if let Some(user_id) = cookies.get_private("user_id") {
		todos::add(todo);
	}
}

#[get("/activity")]

fn activity(mut cookies: Cookies) -> Html<String> {
	if let Some(user_id) = cookies.get_private("user_id") {
		format_and_HTML(client::activities::index(user_id.value().to_string()))
	}
	else{
	format_and_HTML("you're not logged in, please <a href = '/login'>login here</a>".to_string())
	}
}

#[post("/activityadd", format = "application/json", data = "<activity>")]

fn activity_add(mut cookies: Cookies, activity: Json<Activity>) {
	if let Some(user_id) = cookies.get_private("user_id") {
		activities::add(activity);
	}
}

fn main() {
	//srv::ml::train();
	rocket::ignite().mount("/", routes! [index, login, home, suggestions, todo, todo_add, activity, activity_add, files]).register(catchers! [not_found]).launch();
}