use meilisearch_sdk::document::Document;
use serde::{Deserialize, Serialize};
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

impl Crate {
    pub fn get_readable_download_count(&self) -> String {
        if let Some(downloads) = self.downloads {
            if downloads < 1000 {
                downloads.to_string()
            } else if downloads < 1000000 {
                format!("{:.1}k", downloads as f64 / 1000.0)
            } else {
                format!("{:.1}M", downloads as f64 / 1000000.0)
            }
        } else {
            String::from("?")
        }
    }

    pub fn display(&self) -> Html {
        let mut url = format!("https://lib.rs/crates/{}", self.name);
        url = url.replace("<em>", "");
        url = url.replace("</em>", "");

        html! {
            <li><a href=url>
                <div class="h">
                    <h4>
                        {
                            // This field is formatted so we don't want Yew to escape the HTML tags
                            unescaped_html(&self.name)
                        }
                    </h4>
                    <p class="desc">{unescaped_html(&self.description)}</p>
                </div>
                <div class="meta">
                    <span class="version stable">
                        <span>{"v"}</span>
                        {&self.version}
                    </span>
                    <span class="downloads" title=format!("{} recent downloads", self.downloads.unwrap_or(0))>
                        {self.get_readable_download_count()}
                    </span>
                    {for self.keywords.iter().map(|keyword|
                        html! {
                            <span class="k">
                                <span>{"#"}</span>
                                {keyword}
                            </span>
                        }
                    )}
                </div>

            </a></li>
        }
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
