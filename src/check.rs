use crate::fortnite::{SKINS, SkinTag};

// TODO: Multi-threading
pub async fn check(keys: Vec<String>) -> Option<SkinTag> {
    for key in keys {
        let key = key.trim().to_lowercase();

        // Skin checks
        for skin in SKINS {
            for id in skin.id {
                if key.contains(id.to_lowercase().trim()) {
                    return Some(skin.tag);
                }
            }
            for name in skin.name {
                if key.contains(name.to_lowercase().trim()) {
                    return Some(skin.tag);
                }
            }
            if let Some(set) = skin.set {
                if key.contains(set.id) || key.contains(set.name) {
                    return Some(skin.tag);
                }
            }
        }
    }
    return None;
}
