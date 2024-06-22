use std::collections::HashMap;

use anyhow::Result;
use colored::Colorize;

use super::{Action, Namespace, StorageDescriptor};
use crate::agent::state::State;

#[derive(Debug, Default)]
struct SaveMemory {}

impl Action for SaveMemory {
    fn name(&self) -> &str {
        "save-memory"
    }

    fn description(&self) -> &str {
        include_str!("save.prompt")
    }

    fn attributes(&self) -> Option<HashMap<String, String>> {
        let mut attributes = HashMap::new();

        attributes.insert("key".to_string(), "my-note".to_string());

        Some(attributes)
    }

    fn example_payload(&self) -> Option<&str> {
        Some("put here the custom data you want to keep for later")
    }

    fn run(
        &self,
        state: &State,
        attributes: Option<HashMap<String, String>>,
        payload: Option<String>,
    ) -> Result<Option<String>> {
        if let Some(attrs) = attributes {
            if let Some(key) = attrs.get("key") {
                if let Some(data) = payload {
                    state.get_storage("memories")?.add_tagged(key, &data);
                    return Ok(Some("memory saved".to_string()));
                }

                return Err(anyhow!("no content specified for save-memory"));
            }

            return Err(anyhow!("no key attribute specified for save-memory"));
        }

        Err(anyhow!("no attributes specified for save-memory"))
    }
}

#[derive(Debug, Default)]
struct DeleteMemory {}

impl Action for DeleteMemory {
    fn name(&self) -> &str {
        "delete-memory"
    }

    fn description(&self) -> &str {
        include_str!("delete.prompt")
    }

    fn attributes(&self) -> Option<HashMap<String, String>> {
        let mut attributes = HashMap::new();

        attributes.insert("key".to_string(), "my-note".to_string());

        Some(attributes)
    }

    fn run(
        &self,
        state: &State,
        attributes: Option<HashMap<String, String>>,
        _: Option<String>,
    ) -> Result<Option<String>> {
        if let Some(attrs) = attributes {
            if let Some(key) = attrs.get("key") {
                return if state.get_storage("memories")?.del_tagged(key).is_some() {
                    return Ok(Some("memory deleted".to_string()));
                } else {
                    Err(anyhow!("memory '{}' not found", key))
                };
            }

            return Err(anyhow!("no key attribute specified for delete-memory"));
        }

        Err(anyhow!("no attributes specified for delete-memory"))
    }
}

#[derive(Debug, Default)]
struct RecallMemory {}

impl Action for RecallMemory {
    fn name(&self) -> &str {
        "get-memory"
    }

    fn description(&self) -> &str {
        include_str!("recall.prompt")
    }

    fn attributes(&self) -> Option<HashMap<String, String>> {
        let mut attributes = HashMap::new();

        attributes.insert("key".to_string(), "my-note".to_string());

        Some(attributes)
    }

    fn run(
        &self,
        state: &State,
        attributes: Option<HashMap<String, String>>,
        _: Option<String>,
    ) -> Result<Option<String>> {
        if let Some(attrs) = attributes {
            if let Some(key) = attrs.get("key") {
                return if let Some(memory) = state.get_storage("memories")?.get_tagged(key) {
                    println!("<{}> recalling {}", "memories".bold(), key);
                    return Ok(Some(memory));
                } else {
                    eprintln!("<{}> memory {} does not exist", "memories".bold(), key);
                    Err(anyhow!("memory '{}' not found", key))
                };
            }

            return Err(anyhow!("no key attribute specified for delete-memory"));
        }

        Err(anyhow!("no attributes specified for delete-memory"))
    }
}

pub(crate) fn get_namespace() -> Namespace {
    Namespace::new_default(
        "Memory".to_string(),
        include_str!("ns.prompt").to_string(),
        vec![Box::<SaveMemory>::default(), Box::<DeleteMemory>::default()],
        Some(vec![StorageDescriptor::tagged("memories")]),
    )
}
