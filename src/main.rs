use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { 
        document::Stylesheet{ href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let save = move |evt| {};
    let skip = move |evt| {};

    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    rsx! {
        div {
            id: "dogview",
            img { src: "{img_src}" }
        }
        div {
            id: "buttons",
            button {
                onclick: skip,
                id: "skip",
                "skip"
            }
            button {
                onclick: save,
                id: "save",
                "save!"
            }
        }
    }
}
