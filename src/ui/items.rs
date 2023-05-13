use crate::*;

#[derive(Reflect, FromReflect, Copy, Clone, Debug, PartialEq)]
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
    pub quantity: u32,
    pub name: String,
    pub sell_price: u32,
}

impl Item {
    pub fn new(item_type: ItemType, quantity: u32) -> Self {
        Self {
            item_type,
            quantity,
            name: Self::get_name(item_type),
            sell_price: Self::get_sell_price(item_type),
        }
    }

    fn get_name(item_type: ItemType) -> String {
        use ItemType::*;
        match item_type {
            Taffy => "Taffy",
            Nougat => "Nougat",
            Marshmallow => "Marshmallow",
        }
        .to_string()
    }

    fn get_sell_price(item_type: ItemType) -> u32 {
        use ItemType::*;
        match item_type {
            Taffy => 5,
            Nougat => 12,
            Marshmallow => 26,
        }
    }
}
