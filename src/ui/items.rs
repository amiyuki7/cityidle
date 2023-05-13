use crate::*;

#[derive(Reflect, FromReflect, Copy, Clone, Debug)]
pub enum ItemType {
    Taffy,
    Nougat,
    Marshmallow,
}

#[derive(Resource)]
pub struct ItemIcons {
    pub empty: Handle<Image>,
    pub taffy: Handle<Image>,
    pub nougat: Handle<Image>,
    pub marshmallow: Handle<Image>,
}

pub fn load_item_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ItemIcons {
        empty: asset_server.load("items/empty256.png"),
        taffy: asset_server.load("items/taffy256.png"),
        nougat: asset_server.load("items/nougat256.png"),
        marshmallow: asset_server.load("items/marshmallow256.png"),
    })
}

pub struct Item {
    pub item_type: ItemType,
    pub qty: u32,
}

impl Item {
    pub fn new(item_type: ItemType, qty: u32) -> Self {
        Self { item_type, qty }
    }
}
