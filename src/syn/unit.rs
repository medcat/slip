use super::item::Item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    items: Vec<Item>,
}
