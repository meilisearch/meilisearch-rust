#![recursion_limit = "512"]
use meilisearch_sdk::{
    client::Client,
    indexes::Index,
    search::{SearchResults, Selectors::All},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod document;
use crate::document::Crate;

// We need a static client because yew's component trait does not allow lifetimes shorter than static
pub static CLIENT: Client = Client::new(
    "https://finding-demos.meilisearch.com",
    "2b902cce4f868214987a9f3c7af51a69fa660d74a785bed258178b96e3480bb3",
);

struct Model {
    link: Rc<ComponentLink<Self>>,
    index: Rc<Index<'static>>, // The lifetime of Index is the lifetime of the client
    results: Rc<RefCell<Vec<Crate>>>,
}

enum Msg {
    Input(String),
    Update,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Model {
        Self {
            link: Rc::new(link),
            index: Rc::new(CLIENT.assume_index("crates")),
            results: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value) => {
                let index = Rc::clone(&self.index);
                let link = Rc::clone(&self.link);
                let results = Rc::clone(&self.results);

                // Spawn a task loading results
                spawn_local(async move {
                    let fresh_results: SearchResults<Crate> = index
                        .search()
                        .with_query(&value)
                        .with_attributes_to_highlight(All)
                        .execute()
                        .await
                        .expect("Failed to execute query");

                    let mut fresh_formatted_results = Vec::new();
                    for result in fresh_results.hits {
                        fresh_formatted_results.push(result.formatted_result.unwrap());
                    }

                    *results.borrow_mut() = fresh_formatted_results;
                    link.send_message(Msg::Update);
                });
                false
            },
            Msg::Update => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <header id="serp">
                <div class="inner-col">
                    <h3>{"Meili crates browser 2000"}</h3>
                    <p>
                        {"This search bar is provided by "}<a href="https://meilisearch.com">{"Meili"}</a>{", it is a demonstration of our instant search engine."}<br/>
                        {"If you want to take a look at the project source code, it's your lucky day as it is "}<a href="https://github.com/meilisearch/MeiliDB">{"available on github"}</a>{"."}<br/>
                        {"We wrote a blog post about "}<a href="https://blog.meilisearch.com/meili-finds-rust-crates/">{"how we made this search engine available for you"}</a>{"."}<br/>
                        {"The whole design was taken from "}<a href="https://lib.rs">{"lib.rs"}</a>{" because we love it."}<br/>
                        <br/>{"We pull new crates and crates updates every "}<em>{"10 minutes"}</em>{" from "}<a href="https://docs.rs/releases">{"docs.rs"}</a>{" and all the downloads counts "}<em>{"every day at 3:30 PM UTC"}</em>{" from "}<a href="https://crates.io/data-access">{"crates.io"}</a>{". Currently we have something like "}<em>{" 31 729 crates"}</em>{"."}<br/>
                        <br/>{"Have fun using it "}<img draggable="false" class="emoji" alt="⌨️" src="moz-extension://57a82bfe-3134-4c34-bdb1-bc4ada430e6c/data/components/twemoji/svg/2328.svg"/>{" "}<img draggable="false" class="emoji" alt="💨" src="moz-extension://57a82bfe-3134-4c34-bdb1-bc4ada430e6c/data/components/twemoji/svg/1f4a8.svg"/><br/>
                    </p>
                    <form role="search" id="search">
                        <input placeholder="name, keywords, description" autofocus=true autocapitalize="none" autocorrect=false autocomplete=false tabindex="1" type="search" id="textSearch" oninput=self.link.callback(|e: yew::html::InputData| Msg::Input(e.value))/>
                        <span id="request-time">{"0 ms"}</span>
                    </form>
                    <nav>
                        <ul>
                            <li class="active">{"Sorted by relevance"}</li>
                        </ul>
                    </nav>
                </div>
            </header>
            <main id="results">
                <div class="inner-col">
                    <ol id="handlebars-list">
                        {for self.results.borrow().iter().map(|r| r.to_html())}
                    </ol>
                </div>
            </main>
            <footer>
                <div class="inner-col">
                <p>{"Search powered by "}<a href="https://github.com/meilisearch/MeiliDB">{"MeiliDB"}</a>{"."}</p>
                </div>
            </footer>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
