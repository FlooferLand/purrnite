use serde::{Deserialize, Serialize};

// Shop JSON

#[derive(Serialize, Deserialize)]
pub struct ShopJson {
    #[serde(alias="shop", alias="entries")]
    pub items: Vec<ShopEntry>
}
impl ShopJson {
    pub fn empty() -> ShopJson {
        Self { items: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShopEntry {
    #[serde(alias="name", alias="displayName")]
    name: Option<String>,

    #[serde(alias="layout")]
    layout: Option<ShopEntryLayout>,

    #[serde(alias="description", alias="displayDescription", alias="devName")]
    description: String,

    #[serde(alias="items", alias="granted")]
    sub_items: Option<Vec<ShopItem>>,

    #[serde(alias="mainId", alias="sectionId")]
    id: String,

    #[serde()]
    path: Option<String>
}

#[derive(Serialize, Deserialize)]
struct ShopEntryLayout {
    id: String,
    name: String,
    category: String
}

// Shop item

#[derive(Serialize, Deserialize)]
pub struct ShopItem {
    id: String,
    name: String,
    description: String,
    path: Option<String>,
    upcoming: Option<bool>,
    set: Option<ShopItemSet>
}
#[derive(Serialize, Deserialize)]
pub struct ShopItemSet {
    #[serde(alias = "backendValue")]
    id: String,

    #[serde(alias = "value")]
    name: String,

    #[serde(alias = "partOf")]
    text: String
}

/// Combines all data together for easy string searches
pub fn aggregate(input: ShopJson) -> Vec<String> {
    let mut output = Vec::new();
    let input = input.items;

    // Cobweb of slow safety checks
    for entry in input {
        output.push(entry.id);
        output.push(entry.description);

        // Sub-items
        if let Some(items) = entry.sub_items {
            for item in items {
                output.push(item.id);
                output.push(item.description);
                output.push(item.name);

                if let Some(set) = item.set {
                    output.push(set.id);
                    output.push(set.text);
                    output.push(set.name);
                }
                if let Some(path) = item.path {
                    output.push(path);
                }
            }
        }

        // Other
        if let Some(layout) = entry.layout {
            output.push(layout.id);
            output.push(layout.category);
            output.push(layout.name);
        }
        if let Some(path) = entry.path {
            output.push(path);
        }
        if let Some(name) = entry.name {
            output.push(name);
        }
    }

    // Returning
    //output.iter().map(|string| string.to_lowercase().trim().to_string()).collect()
    output
}
