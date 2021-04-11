
#![feature(proc_macro_hygiene)]
use render::{rsx, raw, Render, component};
use std::fs;

use crate::client::_page::{Page};

pub fn index() -> String {
	let page = rsx! {
        <Page title = { "Welcome" }>
            <script> { fs::read_to_string("./public/index.js").unwrap() } </script> 
            <h1> { "Hello World !" } </h1>
        </Page>
    };
	page.render()
}
