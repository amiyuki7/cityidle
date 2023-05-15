use crate::*;

#[derive(Reflect, FromReflect, Copy, Clone, Debug, PartialEq)]
pub enum ItemType {
    Taffy,
    Nougat,
    Marshmallow,
    Coffee,
    Cocoa,
    Milkshake,
    Apple,
    Branch,
    Honey,
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
    pub apple: Handle<Image>,
    pub branch: Handle<Image>,
    pub honey: Handle<Image>,

    pub candy_shop: Handle<Image>,
    pub coffee_shop: Handle<Image>,
    pub tree: Handle<Image>,
    pub factory: Handle<Image>,
}

pub fn load_item_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ItemIcons {
        empty: asset_server.load("icons/items/empty256.png"),
        taffy: asset_server.load("icons/items/taffy256.png"),
        nougat: asset_server.load("icons/items/nougat256.png"),
        marshmallow: asset_server.load("icons/items/marshmallow256.png"),
        coffee: asset_server.load("icons/items/coffee256.png"),
        cocoa: asset_server.load("icons/items/cocoa256.png"),
        milkshake: asset_server.load("icons/items/milkshake256.png"),
        apple: asset_server.load("icons/items/apple256.png"),
        branch: asset_server.load("icons/items/branch256.png"),
        honey: asset_server.load("icons/items/honey256.png"),

        candy_shop: asset_server.load("icons/buildings/candy_shop256.png"),
        coffee_shop: asset_server.load("icons/buildings/coffee_shop256.png"),
        tree: asset_server.load("icons/buildings/tree256.png"),
        factory: asset_server.load("icons/buildings/factory256.png"),
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
            Apple => "Apple",
            Branch => "Branch",
            Honey => "Honey",
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

            Apple => 18,
            Branch => 30,
            Honey => 56,
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

            Apple => 21,
            Branch => 35,
            Honey => 63,
        }
    }
}
