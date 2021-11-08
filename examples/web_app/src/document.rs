use meilisearch_sdk::document::Document;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Crate {
    name: String,
    downloads: Option<usize>,
    description: String,
    keywords: Vec<String>,
    categories: Vec<String>,
    readme: String,
    version: String,
}

// Implement the Document trait so that we can use our struct with MeiliSearch
impl Document for Crate {
    type UIDType = String;

    fn get_uid(&self) -> &Self::UIDType {
        &self.name
    }
}


fn get_readable_download_count(this: &Map<String, Value>) -> String {
    if let Some(downloads) = this["downloads"].as_f64() {
        if downloads < 1000.0 {
            downloads.to_string()
        } else if downloads < 1000000.0 {
            format!("{:.1}k", downloads / 1000.0)
        } else {
            format!("{:.1}M", downloads / 1000000.0)
        }
    } else {
        String::from("?")
    }
}

pub fn display(this: &Map<String, Value>) -> Html {
    let mut url = format!("https://lib.rs/crates/{}", this["name"].as_str().unwrap_or_default());
    url = url.replace("<em>", "");
    url = url.replace("</em>", "");

    html! {
        <li><a href=url>
            <div class="h">
                <h4>
                    {
                        // This field is formatted so we don't want Yew to escape the HTML tags
                        unescaped_html(&this["name"].as_str().unwrap_or_default())
                    }
                </h4>
                <p class="desc">{unescaped_html(&this["description"].as_str().unwrap_or_default())}</p>
            </div>
            <div class="meta">
                <span class="version stable">
                    <span>{"v"}</span>
                    {&this["version"].as_str().unwrap_or_default()}
                </span>
                <span class="downloads" title=format!("{} recent downloads", this["downloads"].as_f64().unwrap_or(0.0))>
                    {get_readable_download_count(this)}
                </span>
                {for this["keywords"].as_array().unwrap().iter().map(|keyword|
                    html! {
                        <span class="k">
                            <span>{"#"}</span>
                            {keyword.as_str().unwrap_or_default()}
                        </span>
                    }
                )}
            </div>

        </a></li>
    }
}

use web_sys::Node;
use yew::virtual_dom::VNode;

/// Creates an element from raw HTML
fn unescaped_html(html: &str) -> VNode {
    let element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    element.set_inner_html(html);

    VNode::VRef(Node::from(element))
}
