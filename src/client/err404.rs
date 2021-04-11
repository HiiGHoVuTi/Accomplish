
#![feature(proc_macro_hygiene)]
use render::{rsx, Render, component};

use crate::client::_page::Page;

pub fn index(name: String) -> String {
	let page = rsx! {
        <html>
            <Page title = { "404" }> { format!("Sorry, we couldn't find the page named '{}'", name) } </Page>
        </html>
    };
	page.render()
}
