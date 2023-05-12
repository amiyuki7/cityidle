pub enum ItemType {
    Taffy,
    Jellybeans,
    Swirlmallow,
}

pub struct Item {
    item_type: ItemType,
    qty: u32,
}

impl Item {
    pub fn new_empty(item_type: ItemType) -> Self {
        Self { item_type, qty: 0 }
    }
}
