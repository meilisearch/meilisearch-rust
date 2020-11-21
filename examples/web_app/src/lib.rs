#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
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
                        <input placeholder="name, keywords, description" autofocus=true autocapitalize="none" autocorrect=false autocomplete=false tabindex="1" type="search" id="textSearch"/>
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
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
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
