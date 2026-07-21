use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Parameters for listing dynamic search rules.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRulesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DynamicSearchRulesFilter>,
}

/// Filters for listing dynamic search rules.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRulesFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// A page of dynamic search rules.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRulesResults {
    pub results: Vec<DynamicSearchRule>,
    pub offset: u32,
    pub limit: u32,
    pub total: u32,
}

/// A dynamic search rule configured on a Meilisearch instance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRule {
    pub uid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precedence: Option<u64>,
    pub active: bool,
    #[serde(default)]
    pub conditions: DynamicSearchRuleConditions,
    pub actions: Vec<DynamicSearchRuleAction>,
}

/// Conditions that must match before a dynamic search rule applies.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRuleConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<DynamicSearchRuleQueryCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<DynamicSearchRuleTimeCondition>,
}

/// A query condition for a dynamic search rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRuleQueryCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_empty: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<String>,
}

/// A time condition for a dynamic search rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRuleTimeCondition {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub start: Option<OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub end: Option<OffsetDateTime>,
}

/// An action performed when a dynamic search rule matches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRuleAction {
    pub selector: DynamicSearchRuleSelector,
    pub action: DynamicSearchRuleActionType,
}

/// A document selected by a dynamic search rule action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRuleSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_uid: Option<String>,
    pub id: String,
}

/// The action applied to a selected document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DynamicSearchRuleActionType {
    Pin { position: u32 },
}

/// Partial update used to create or update a dynamic search rule.
///
/// Each field distinguishes omission (`None`) from reset (`Some(None)`) and a
/// concrete value (`Some(Some(value))`).
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSearchRuleUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<Option<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Option<DynamicSearchRuleConditions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Option<Vec<DynamicSearchRuleAction>>>,
}

impl DynamicSearchRuleUpdate {
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(Some(description.into()));
        self
    }

    #[must_use]
    pub fn with_precedence(mut self, precedence: u64) -> Self {
        self.precedence = Some(Some(precedence));
        self
    }

    #[must_use]
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(Some(active));
        self
    }

    #[must_use]
    pub fn with_conditions(mut self, conditions: DynamicSearchRuleConditions) -> Self {
        self.conditions = Some(Some(conditions));
        self
    }

    #[must_use]
    pub fn with_actions(
        mut self,
        actions: impl IntoIterator<Item = DynamicSearchRuleAction>,
    ) -> Self {
        self.actions = Some(Some(actions.into_iter().collect()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Client, tasks::TaskType};
    use mockito::Matcher;
    use serde_json::json;

    fn task(uid: u32) -> String {
        json!({
            "taskUid": uid,
            "indexUid": null,
            "status": "enqueued",
            "type": "dsrUpdate",
            "enqueuedAt": "2026-07-21T12:00:00Z"
        })
        .to_string()
    }

    fn rule() -> serde_json::Value {
        json!({
            "uid": "black-friday",
            "description": "Black Friday products",
            "precedence": 10,
            "active": true,
            "conditions": { "query": { "words": "black friday" } },
            "actions": [{
                "selector": { "indexUid": "products", "id": "123" },
                "action": { "type": "pin", "position": 1 }
            }]
        })
    }

    #[tokio::test]
    async fn lists_rules_with_pagination_and_filtering() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/dynamic-search-rules")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(json!({
                "offset": 1,
                "limit": 2,
                "filter": { "query": "Black Friday", "active": true }
            })))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({ "results": [rule()], "offset": 1, "limit": 2, "total": 1 }).to_string(),
            )
            .create_async()
            .await;
        let client = Client::new(server.url(), None::<String>).unwrap();
        let query = DynamicSearchRulesQuery {
            offset: Some(1),
            limit: Some(2),
            filter: Some(DynamicSearchRulesFilter {
                query: Some("Black Friday".into()),
                active: Some(true),
            }),
        };

        let result = client.get_dynamic_search_rules(&query).await.unwrap();

        mock.assert_async().await;
        assert_eq!(result.total, 1);
        assert_eq!(result.results[0].uid, "black-friday");
    }

    #[tokio::test]
    async fn gets_rule_by_uid() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/dynamic-search-rules/black-friday")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(rule().to_string())
            .create_async()
            .await;
        let client = Client::new(server.url(), None::<String>).unwrap();

        let result = client
            .get_dynamic_search_rule("black-friday")
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(result.uid, "black-friday");
        assert_eq!(result.precedence, Some(10));
    }

    #[tokio::test]
    async fn creates_and_updates_rules_as_tasks() {
        let mut server = mockito::Server::new_async().await;
        let create = server
            .mock("PATCH", "/dynamic-search-rules/black-friday")
            .match_body(Matcher::Json(json!({
                "description": "Black Friday products",
                "precedence": 10,
                "active": true,
                "conditions": { "query": { "words": "black friday" } },
                "actions": [{
                    "selector": { "indexUid": "products", "id": "123" },
                    "action": { "type": "pin", "position": 1 }
                }]
            })))
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(task(1))
            .create_async()
            .await;
        let update = server
            .mock("PATCH", "/dynamic-search-rules/black-friday")
            .match_body(Matcher::Json(json!({
                "description": "Black Friday and Cyber Monday",
                "precedence": 5
            })))
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(task(2))
            .create_async()
            .await;
        let client = Client::new(server.url(), None::<String>).unwrap();
        let create_payload = DynamicSearchRuleUpdate::default()
            .with_description("Black Friday products")
            .with_precedence(10)
            .with_active(true)
            .with_conditions(DynamicSearchRuleConditions {
                query: Some(DynamicSearchRuleQueryCondition {
                    words: Some("black friday".into()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .with_actions([DynamicSearchRuleAction {
                selector: DynamicSearchRuleSelector {
                    index_uid: Some("products".into()),
                    id: "123".into(),
                },
                action: DynamicSearchRuleActionType::Pin { position: 1 },
            }]);

        let created = client
            .update_dynamic_search_rule("black-friday", &create_payload)
            .await
            .unwrap();
        let updated = client
            .update_dynamic_search_rule(
                "black-friday",
                &DynamicSearchRuleUpdate::default()
                    .with_description("Black Friday and Cyber Monday")
                    .with_precedence(5),
            )
            .await
            .unwrap();

        create.assert_async().await;
        update.assert_async().await;
        assert_eq!(created.task_uid, 1);
        assert_eq!(updated.task_uid, 2);
        assert!(matches!(created.update_type, TaskType::DsrUpdate { .. }));
        assert!(matches!(updated.update_type, TaskType::DsrUpdate { .. }));
    }

    #[tokio::test]
    async fn deletes_rule_as_task() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/dynamic-search-rules/black-friday")
            .with_status(202)
            .with_header("content-type", "application/json")
            .with_body(task(3))
            .create_async()
            .await;
        let client = Client::new(server.url(), None::<String>).unwrap();

        let result = client
            .delete_dynamic_search_rule("black-friday")
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(result.task_uid, 3);
        assert!(matches!(result.update_type, TaskType::DsrUpdate { .. }));
    }
}
