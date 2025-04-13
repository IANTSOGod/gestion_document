use dioxus::prelude::*;

#[component]
pub fn Home()-> Element{
    rsx!{
        div { 
            style: "text-align: center",
            h1 {
                style: "color: blue",
                "Hello, world!"
            }
        }
    }
}