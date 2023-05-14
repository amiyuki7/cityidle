use crate::*;

#[derive(Reflect, FromReflect, Copy, Clone, Debug, PartialEq)]
pub enum ItemType {
    Taffy,
    Nougat,
    Marshmallow,
    Coffee,
    Cocoa,
    Milkshake,
}

#[derive(Resource)]
pub struct ItemIcons {
    pub empty: Handle<Image>,
    pub taffy: Handle<Image>,
    pub nougat: Handle<Image>,
    pub marshmallow: Handle<Image>,
    pub coffee: Handle<Image>,
    pub cocoa: Handle<Image>,
    pub milkshake: Handle<Image>,

    pub candy_shop: Handle<Image>,
    pub coffee_shop: Handle<Image>,
}

pub fn load_item_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ItemIcons {
        empty: asset_server.load("items/empty256.png"),
        taffy: asset_server.load("items/taffy256.png"),
        nougat: asset_server.load("items/nougat256.png"),
        marshmallow: asset_server.load("items/marshmallow256.png"),
        coffee: asset_server.load("items/coffee256.png"),
        cocoa: asset_server.load("items/cocoa256.png"),
        milkshake: asset_server.load("items/milkshake256.png"),

        candy_shop: asset_server.load("items/candy_shop256.png"),
        coffee_shop: asset_server.load("items/coffee_shop256.png"),
    })
}

pub struct Item {
    pub item_type: ItemType,
    pub quantity: u32,
    pub name: String,
    pub sell_price: u32,
    pub base_buy_price: u32,
}

impl Item {
    pub fn new(item_type: ItemType, quantity: u32) -> Self {
        Self {
            item_type,
            quantity,
            name: Self::get_name(item_type),
            sell_price: Self::get_sell_price(item_type),
            base_buy_price: Self::get_base_buy_price(item_type),
        }
    }

    fn get_name(item_type: ItemType) -> String {
        use ItemType::*;
        match item_type {
            Taffy => "Taffy",
            Nougat => "Nougat",
            Marshmallow => "Marshmallow",
            Coffee => "Coffee",
            Cocoa => "Cocoa",
            Milkshake => "Milkshake",
        }
        .to_string()
    }

    fn get_sell_price(item_type: ItemType) -> u32 {
        use ItemType::*;
        match item_type {
            Taffy => 5,
            Nougat => 12,
            Marshmallow => 26,
            Coffee => 8,
            Cocoa => 15,
            Milkshake => 35,
        }
    }

    fn get_base_buy_price(item_type: ItemType) -> u32 {
        use ItemType::*;
        match item_type {
            Taffy => 6,
            Nougat => 14,
            Marshmallow => 30,
            Coffee => 9,
            Cocoa => 16,
            Milkshake => 40,
        }
    }
}
