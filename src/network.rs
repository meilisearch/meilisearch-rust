use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    #[serde(rename = "self")]
    pub self_: Option<String>,
    #[serde(deserialize_with = "network_deserializer")]
    pub remotes: Vec<Remote>,
}

fn network_deserializer<'de, D>(d: D) -> Result<Vec<Remote>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: BTreeMap<String, HashMap<String, String>> = Deserialize::deserialize(d)?;

    Ok(map
        .into_iter()
        .map(|(name, remote)| Remote {
            name,
            url: remote.get("url").cloned().unwrap_or_default(),
            search_api_key: remote.get("searchApiKey").cloned(),
        })
        .collect())
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Remote {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,

    pub url: String,
    pub search_api_key: Option<String>,
}

#[derive(Serialize, Default)]
pub struct NetworkUpdate {
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    self_: Option<Option<String>>,
    remotes: Option<BTreeMap<String, Option<Remote>>>,
}

impl NetworkUpdate {
    #[must_use]
    pub fn new() -> Self {
        NetworkUpdate {
            self_: None,
            remotes: Some(BTreeMap::new()),
        }
    }

    pub fn reset_self(&mut self) -> &mut Self {
        self.self_ = Some(None);
        self
    }

    pub fn reset_remotes(&mut self) -> &mut Self {
        self.remotes = None;
        self
    }

    pub fn with_self(&mut self, new_self: &str) -> &mut Self {
        self.self_ = Some(Some(new_self.to_string()));
        self
    }

    pub fn with_remotes(&mut self, new_remotes: &[Remote]) -> &mut Self {
        if self.remotes.is_none() {
            self.remotes = Some(BTreeMap::new());
        }

        self.remotes.as_mut().unwrap().extend(
            new_remotes
                .iter()
                .map(|new_remote| (new_remote.name.clone(), Some(new_remote.clone()))),
        );

        self
    }

    pub fn delete_remotes(&mut self, remotes_to_delete: &[&str]) -> &mut Self {
        if self.remotes.is_none() {
            self.remotes = Some(BTreeMap::new());
        }

        self.remotes.as_mut().unwrap().extend(
            remotes_to_delete
                .iter()
                .map(|remote_name| (remote_name.to_string(), None)),
        );

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_network() {
        let example_json = r###"
            {
              "self": "ms-00",
              "remotes": {
                "ms-00": {
                  "url": "http://ms-1235.example.meilisearch.io",
                  "searchApiKey": "Ecd1SDDi4pqdJD6qYLxD3y7VZAEb4d9j6LJgt4d6xas"
                },
                "ms-01": {
                  "url": "http://ms-4242.example.meilisearch.io",
                  "searchApiKey": "hrVu-OMcjPGElK7692K7bwriBoGyHXTMvB5NmZkMKqQ"
                }
              }
            }
        "###;

        let actual_remotes: Network = serde_json::from_str(example_json).unwrap();

        let expected_remotes = Network {
            self_: Some("ms-00".to_string()),
            remotes: vec![
                Remote {
                    name: "ms-00".to_string(),
                    url: "http://ms-1235.example.meilisearch.io".to_string(),
                    search_api_key: Some("Ecd1SDDi4pqdJD6qYLxD3y7VZAEb4d9j6LJgt4d6xas".to_string()),
                },
                Remote {
                    name: "ms-01".to_string(),
                    url: "http://ms-4242.example.meilisearch.io".to_string(),
                    search_api_key: Some("hrVu-OMcjPGElK7692K7bwriBoGyHXTMvB5NmZkMKqQ".to_string()),
                },
            ],
        };

        assert_eq!(actual_remotes, expected_remotes);
    }

    #[test]
    fn test_serialize_network_update_reset_all() {
        let mut expected_reset_self_json = r###"
            {
              "self": null,
              "remotes": null
            }
        "###
        .to_string();
        expected_reset_self_json.retain(|c| !c.is_whitespace());

        let mut reset_self_json =
            serde_json::to_string(NetworkUpdate::new().reset_self().reset_remotes()).unwrap();
        reset_self_json.retain(|c| !c.is_whitespace());

        assert_eq!(expected_reset_self_json, reset_self_json);
    }

    #[test]
    fn test_serialize_network_update_reset_self() {
        let mut expected_reset_self_json = r###"
            {
              "self": null,
              "remotes": {}
            }
        "###
        .to_string();
        expected_reset_self_json.retain(|c| !c.is_whitespace());

        let mut reset_self_json = serde_json::to_string(NetworkUpdate::new().reset_self()).unwrap();
        reset_self_json.retain(|c| !c.is_whitespace());

        assert_eq!(expected_reset_self_json, reset_self_json);
    }

    #[test]
    fn test_serialize_network_update_with_self() {
        let mut expected_with_self_json = r###"
            {
              "self": "ms-00",
              "remotes": {}
            }
        "###
        .to_string();
        expected_with_self_json.retain(|c| !c.is_whitespace());

        let mut with_self_json =
            serde_json::to_string(NetworkUpdate::new().with_self("ms-00")).unwrap();
        with_self_json.retain(|c| !c.is_whitespace());

        assert_eq!(expected_with_self_json, with_self_json);
    }

    #[test]
    fn test_serialize_network_update_reset_remotes() {
        let mut expected_reset_remotes_json = r###"
            {
              "remotes": null
            }
        "###
        .to_string();
        expected_reset_remotes_json.retain(|c| !c.is_whitespace());

        let mut reset_remotes_json =
            serde_json::to_string(NetworkUpdate::new().reset_remotes()).unwrap();
        reset_remotes_json.retain(|c| !c.is_whitespace());

        assert_eq!(expected_reset_remotes_json, reset_remotes_json);
    }

    #[test]
    fn test_serialize_network_update_add_remotes() {
        let mut expected_with_remotes_json = r###"
            {
              "remotes": {
                "ms-00": {
                  "url": "http://localhost:7700",
                  "searchApiKey": "hello_world"
                },
                "ms-01": {
                  "url": "http://localhost:7701",
                  "searchApiKey": "another_key"
                }
              }
            }
        "###
        .to_string();
        expected_with_remotes_json.retain(|c| !c.is_whitespace());

        let mut with_remotes_json = serde_json::to_string(NetworkUpdate::new().with_remotes(&[
            Remote {
                name: "ms-00".to_string(),
                url: "http://localhost:7700".to_string(),
                search_api_key: Some("hello_world".to_string()),
            },
            Remote {
                name: "ms-01".to_string(),
                url: "http://localhost:7701".to_string(),
                search_api_key: Some("another_key".to_string()),
            },
        ]))
        .unwrap();
        with_remotes_json.retain(|c| !c.is_whitespace());

        assert_eq!(expected_with_remotes_json, with_remotes_json);
    }

    #[test]
    fn test_serialize_network_update_delete_remotes() {
        let mut expected_with_remotes_json = r###"
            {
              "remotes": {
                "ms-00": null,
                "ms-01": null
              }
            }
        "###
        .to_string();
        expected_with_remotes_json.retain(|c| !c.is_whitespace());

        let mut with_remotes_json =
            serde_json::to_string(NetworkUpdate::new().delete_remotes(&["ms-00", "ms-01"]))
                .unwrap();
        with_remotes_json.retain(|c| !c.is_whitespace());

        assert_eq!(expected_with_remotes_json, with_remotes_json);
    }

    #[test]
    fn test_serialize_network_update_operation_override() {
        let mut expected_overridden_json = r###"
            {
              "remotes": {
                "ms-00": null,
                "ms-01": null
              }
            }
        "###
        .to_string();
        expected_overridden_json.retain(|c| !c.is_whitespace());

        let mut overriden_json = serde_json::to_string(
            NetworkUpdate::new()
                .with_remotes(&[
                    Remote {
                        name: "ms-00".to_string(),
                        url: "http://localhost:7700".to_string(),
                        search_api_key: Some("hello_world".to_string()),
                    },
                    Remote {
                        name: "ms-01".to_string(),
                        url: "http://localhost:7701".to_string(),
                        search_api_key: Some("another_key".to_string()),
                    },
                ])
                .delete_remotes(&["ms-00", "ms-01"]),
        )
        .unwrap();
        overriden_json.retain(|c| !c.is_whitespace());

        assert_eq!(expected_overridden_json, overriden_json);
    }
}
