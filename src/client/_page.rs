
#![feature(pub_macro_rules)]

use render::html::HTML5Doctype;
use render::{
    component,
    rsx,
    html,
    Render,
};
use std::fs;

#[component]
fn Link<'a>(href: &'a str, name: &'a str){
    rsx! {
        <a href = {href}> {name} </a>
    }
}

#[component]
pub fn Page<'a, Children: Render>(title: &'a str, children: Children) {
   rsx! {
     <>
       <HTML5Doctype />
       <html>
         <head>
            <title>{title}</title>
          </head>
          <style> { fs::read_to_string("./public/global.css").unwrap() } </style>
          <link rel={"stylesheet"} href={"https://www.w3schools.com/w3css/4/w3.css"}></link>
          <link rel={"stylesheet"} href={"https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"}> </link>
         <body>
            <header>
                <nav>
                    <ul>
                        <Link name = { "Home" } href = { "home" }/>
                    </ul>
                    <ul>
                        <Link name = { "Suggestions" } href = { "suggestions" }/>
                    </ul>
                    <ul>
                        <Link name = { "Add a TODO" } href = { "todo" } />
                    </ul>
                    <ul>
                        <Link name = { "Register Activity" } href = { "activity" } />
                    </ul>
                </nav>
            </header>
            {children}
         </body>
       </html>
     </>
   }
}