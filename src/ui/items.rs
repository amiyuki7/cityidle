use crate::*;

#[derive(Reflect, FromReflect, Copy, Clone, Debug, PartialEq)]
pub enum ItemType {
    BronzeCoin,
    SilverCoin,
    GoldCoin,
    Taffy,
    Nougat,
    Marshmallow,
    Coffee,
    Cocoa,
    Milkshake,
    Apple,
    Branch,
    Honey,
    Steel,
    Chip,
    Phone,
    Log,
    Lantern,
    Axe,
}

#[derive(Resource)]
pub struct ItemIcons {
    pub bronze_coin: Handle<Image>,
    pub silver_coin: Handle<Image>,
    pub gold_coin: Handle<Image>,

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
    pub steel: Handle<Image>,
    pub chip: Handle<Image>,
    pub phone: Handle<Image>,
    pub log: Handle<Image>,
    pub lantern: Handle<Image>,
    pub axe: Handle<Image>,

    pub candy_shop: Handle<Image>,
    pub coffee_shop: Handle<Image>,
    pub tree: Handle<Image>,
    pub factory: Handle<Image>,
    pub cabin: Handle<Image>,
}

pub fn load_item_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ItemIcons {
        bronze_coin: asset_server.load("icons/items/bronze_coin256.png"),
        silver_coin: asset_server.load("icons/items/silver_coin256.png"),
        gold_coin: asset_server.load("icons/items/gold_coin256.png"),

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
        steel: asset_server.load("icons/items/steel256.png"),
        chip: asset_server.load("icons/items/chip256.png"),
        phone: asset_server.load("icons/items/phone256.png"),
        log: asset_server.load("icons/items/log256.png"),
        lantern: asset_server.load("icons/items/lantern256.png"),
        axe: asset_server.load("icons/items/axe256.png"),

        candy_shop: asset_server.load("icons/buildings/candy_shop256.png"),
        coffee_shop: asset_server.load("icons/buildings/coffee_shop256.png"),
        tree: asset_server.load("icons/buildings/tree256.png"),
        factory: asset_server.load("icons/buildings/factory256.png"),
        cabin: asset_server.load("icons/buildings/cabin256.png"),
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
            BronzeCoin => "Brozen Coin",
            SilverCoin => "Silver Coin",
            GoldCoin => "Gold Coin",
            Taffy => "Taffy",
            Nougat => "Nougat",
            Marshmallow => "Marshmallow",
            Coffee => "Coffee",
            Cocoa => "Cocoa",
            Milkshake => "Milkshake",
            Apple => "Apple",
            Branch => "Branch",
            Honey => "Honey",
            Steel => "Steel",
            Chip => "Chip",
            Phone => "Phone",
            Log => "Log",
            Lantern => "Lantern",
            Axe => "Axe",
        }
        .to_string()
    }

    fn get_sell_price(item_type: ItemType) -> u32 {
        use ItemType::*;
        match item_type {
            BronzeCoin => 7,
            SilverCoin => 40,
            GoldCoin => 160,

            Taffy => 5,
            Nougat => 12,
            Marshmallow => 26,

            Coffee => 8,
            Cocoa => 15,
            Milkshake => 35,

            Apple => 18,
            Branch => 30,
            Honey => 56,

            Steel => 60,
            Chip => 82,
            Phone => 350,

            Log => 42,
            Lantern => 70,
            Axe => 210,
        }
    }

    fn get_base_buy_price(item_type: ItemType) -> u32 {
        use ItemType::*;
        match item_type {
            // Can't buy these
            BronzeCoin => 0,
            SilverCoin => 0,
            GoldCoin => 0,

            Taffy => 6,
            Nougat => 14,
            Marshmallow => 30,

            Coffee => 9,
            Cocoa => 16,
            Milkshake => 40,

            Apple => 21,
            Branch => 35,
            Honey => 63,

            Steel => 92,
            Chip => 145,
            Phone => 620,

            Log => 55,
            Lantern => 104,
            Axe => 410,
        }
    }
}
