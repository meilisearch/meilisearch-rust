#![recursion_limit = "512"]
use lazy_static::lazy_static;
use meilisearch_sdk::{Client, Index, SearchResults, Selectors::All};
use serde_json::{Map, Value};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html::Scope, Html};

mod document;
use crate::document::{display, Crate};

lazy_static! {
    static ref CLIENT: Client = Client::new("http://localhost:7700", Some("masterKey"));
}

struct Model {
    index: Rc<Index>,
    results: Vec<Map<String, Value>>,
    processing_time_ms: usize,

    // These two fields are used to avoid rollbacks by giving an ID to each request
    latest_sent_request_id: usize,
    displayed_request_id: usize,
}

enum Msg {
    /// An event sent to update the results with a query
    Input(String),
    /// The event sent to display new results once they are received
    Update {
        results: Vec<Map<String, Value>>,
        processing_time_ms: usize,
        request_id: usize,
    },
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Model {
        Model {
            // The index method avoids checking the existence of the index.
            // It won't make any HTTP request so the function is not async so it's easier to use.
            // Use only if you are sure that the index exists.
            index: Rc::new(CLIENT.index("crates")),
            results: Vec::new(),
            processing_time_ms: 0,

            latest_sent_request_id: 0,
            displayed_request_id: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Sent when the value of the text input changed (so we have to make a new request)
            Msg::Input(value) => {
                let index = Rc::clone(&self.index);
                let link = ctx.link().clone();
                self.latest_sent_request_id += 1;
                let request_id = self.latest_sent_request_id;
                // Spawn a task loading results
                spawn_local(async move {
                    // Load the results
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

                    // We send a new event with the up-to-date data so that we can update the results and display them.
                    link.send_message(Msg::Update {
                        results: fresh_formatted_results,
                        processing_time_ms: fresh_results.processing_time_ms,
                        request_id,
                    });
                });
                false
            }

            // Sent when new results are received
            Msg::Update {
                results,
                processing_time_ms,
                request_id,
            } => {
                if request_id >= self.latest_sent_request_id {
                    self.results = results;
                    self.processing_time_ms = processing_time_ms;
                    self.displayed_request_id = request_id;
                    true
                } else {
                    // We are already displaying more up-to-date results.
                    // This request is too late so we cannot display these results to avoid rollbacks.
                    false
                }
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <header id="serp">
                {header_content(self.processing_time_ms, ctx.link())}
            </header>
            <main id="results">
                <div class="inner-col">
                    <ol id="handlebars-list">
                        {
                            // Display the results
                            for self.results.iter().map(display)
                        }
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

fn header_content(processing_time_ms: usize, link: &Scope<Model>) -> Html {
    html! {
        <div class="inner-col">
            <h3>{"Meili crates browser 2000"}</h3>
            <p>
                {"This search bar is provided by "}<a href="https://meilisearch.com">{"Meili"}</a>{", it is a demonstration of our instant search engine."}<br/>
                {"If you want to take a look at the project source code, it's your lucky day as it is "}<a href="https://github.com/meilisearch/MeiliDB">{"available on github"}</a>{"."}<br/>
                {"We wrote a blog post about "}<a href="https://blog.meilisearch.com/meili-finds-rust-crates/">{"how we made this search engine available for you"}</a>{"."}<br/>
                {"What you are currently using is not the original front end, but a clone using "}<a href="https://github.com/meilisearch/meilisearch-rust">{"the Meilisearch Rust SDK"}</a>{" and "}<a href="https://yew.rs">{"Yew"}</a>{". The code is available "}<a href="https://github.com/meilisearch/meilisearch-rust/tree/main/examples/web_app">{"here"}</a>{"."}<br/>
                {"The whole design was taken from "}<a href="https://lib.rs">{"lib.rs"}</a>{" because we love it."}<br/>
                <br/>{"We pull new crates and crates updates every "}<em>{"10 minutes"}</em>{" from "}<a href="https://docs.rs/releases">{"docs.rs"}</a>{" and all the downloads counts "}<em>{"every day at 3:30 PM UTC"}</em>{" from "}<a href="https://crates.io/data-access">{"crates.io"}</a>{". Currently we have something like "}<em>{" 31 729 crates"}</em>{"."}<br/>
                <br/>{"Have fun using it "}<img draggable="false" class="emoji" alt="⌨️" src="moz-extension://57a82bfe-3134-4c34-bdb1-bc4ada430e6c/data/components/twemoji/svg/2328.svg"/>{" "}<img draggable="false" class="emoji" alt="💨" src="moz-extension://57a82bfe-3134-4c34-bdb1-bc4ada430e6c/data/components/twemoji/svg/1f4a8.svg"/><br/>
            </p>
            <form role="search" id="search">
                // We fire an event each time the value changes so that we can update the results
                <input placeholder="name, keywords, description" autofocus=true autocapitalize="none" autocorrect="off" autocomplete="off" tabindex="1" type="search" id="textSearch" oninput={link.callback(|e: InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    Msg::Input(input.value())
                })}/>
                // We display the processing time here
                <span id="request-time">{processing_time_ms}{" ms"}</span>
            </form>
            <nav>
                <ul>
                    <li class="active">{"Sorted by relevance"}</li>
                </ul>
            </nav>
        </div>
    }
}

// The main() function of wasm
#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    yew::Renderer::<Model>::new().render();
}
