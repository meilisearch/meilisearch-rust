use meilisearch_sdk::document::Document;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Crate {
    name: String,
    downloads: usize,
    description: String,
    keywords: Vec<String>,
    categories: Vec<String>,
    readme: String,
    version: String,
}

impl Document for Crate {
    type UIDType = String;

    fn get_uid(&self) -> &Self::UIDType {
        &self.name
    }
}

impl Crate {
    pub fn readable_downloads(&self) -> String {
        if self.downloads < 1000 {
            self.downloads.to_string()
        } else if self.downloads < 1000000 {
            format!("{:.1}k", self.downloads as f64 / 1000.0)
        } else {
            format!("{:.1}M", self.downloads as f64 / 1000000.0)
        }
    }

    pub fn to_html(&self) -> Html {
        use web_sys::Node;
        use yew::virtual_dom::VNode;

        let name = {
            let name = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("span")
                .unwrap();
            name.set_inner_html(&self.name);
            name
        };

        let desc = {
            let desc = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("span")
                .unwrap();
            desc.set_inner_html(&self.description);
            desc
        };

        let name = VNode::VRef(Node::from(name));
        let desc = VNode::VRef(Node::from(desc));
        let mut url = format!("https://lib.rs/crates/{}", self.name);
        url = url.replace("<em>", "");
        url = url.replace("</em>", "");

        html! {
            <li><a href=url>
                <div class="h">
                    <h4>{name}</h4>
                    <p class="desc">{desc}</p>
                </div>
                <div class="meta">
                    <span class="version stable">
                        <span>{"v"}</span>
                        {&self.version}
                    </span>
                    <span class="downloads" title=format!("{} recent downloads", self.downloads)>
                        {self.readable_downloads()}
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
