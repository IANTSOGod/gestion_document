use dioxus::prelude::*;

#[component]
pub fn Home()-> Element{
    rsx!{
        h1 {
            style: "color: blue",
            "Hello, world!"
        }
    }
}