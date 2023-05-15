use crate::*;
use bevy::window::PrimaryWindow;

pub struct BuildingItem {
    pub building_type: BuildingType,
    pub quantity: u8,
    pub name: String,
    pub price: u32,
}

impl BuildingItem {
    fn new(building_type: BuildingType, quantity: u8) -> Self {
        Self {
            building_type,
            quantity,
            name: Self::get_name(building_type),
            price: Self::get_price(building_type),
        }
    }

    fn get_price(building_type: BuildingType) -> u32 {
        use BuildingType::*;
        match building_type {
            CandyShop => 100,
            CoffeeShop => 100,
            Tree => 100,
            Factory => 100,
            _ => 0,
        }
    }

    fn get_name(building_type: BuildingType) -> String {
        use BuildingType::*;
        match building_type {
            CandyShop => "Candy Shop",
            CoffeeShop => "Coffee Shop",
            Tree => "Tree",
            Factory => "Factory",
            _ => "Untitled",
        }
        .to_string()
    }
}

#[derive(Resource)]
pub struct ConstructInventory {
    pub items: [BuildingItem; 4],
}

impl Default for ConstructInventory {
    fn default() -> Self {
        Self {
            items: [
                BuildingItem::new(BuildingType::CandyShop, 4),
                BuildingItem::new(BuildingType::CoffeeShop, 3),
                BuildingItem::new(BuildingType::Tree, 3),
                BuildingItem::new(BuildingType::Factory, 2),
            ],
        }
    }
}

pub struct ConstructPlugin;

impl Plugin for ConstructPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ConstructPhase>()
            .init_resource::<ConstructInventory>()
            .init_resource::<SelectedItemStats>()
            .init_resource::<BuildingStash>()
            .add_event::<ChangeItemStatsEvent>()
            .add_system(draw_construct.in_schedule(OnEnter(UiState::Construct)))
            .add_system(undraw_construct.in_schedule(OnExit(UiState::Construct)))
            .add_system(undraw_construct.in_schedule(OnEnter(ConstructPhase::Preview)))
            .add_systems(
                (
                    exit_uistate,
                    item_button_interaction,
                    change_item_stats,
                    buy_button_interaction,
                )
                    .in_set(OnUpdate(ConstructPhase::Normal)),
            );
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum ConstructPhase {
    #[default]
    Normal,
    Preview,
}

pub fn exit_uistate(
    keybinds: Res<Keybinds>,
    keys: Res<Input<KeyCode>>,
    previous_camera_state: Res<PreviousCameraState>,
    mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
    mut next_ui_state: ResMut<NextState<UiState>>,
) {
    // TODO: Make [ESC] key work here as well
    if !keys.just_pressed(keybinds.exit_uistate) {
        return;
    }

    next_ui_state.set(UiState::None);
    send_change_camera_state_event.send(ChangeCameraStateEvent(previous_camera_state.0.clone().unwrap()));
}

// Marker
#[derive(Component)]
struct ConstructUIRoot;

#[derive(Component)]
struct BalanceText;

#[derive(Component, Reflect)]
struct ConstructItemButton {
    building_type: Option<BuildingType>,
}

#[derive(Component)]
struct MiniQuantityText;

#[derive(Component)]
struct ItemStatsName;

// Marker
#[derive(Component)]
struct ItemStatsImage;

// Marker
#[derive(Component)]
struct ItemStatsQuantity;

// Marker
#[derive(Component)]
struct ItemStatsBuyPrice;

// Marker
#[derive(Component)]
struct BuyButton;

fn draw_construct(
    mut commands: Commands,
    inventory: Res<Inventory>,
    construct_inventory: Res<ConstructInventory>,
    asset_server: Res<AssetServer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    item_icons: Res<ItemIcons>,
) {
    let physical_screen_height = primary_window.single().resolution.physical_height() as f32;

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
            ..default()
        })
        .insert(Name::new("Construct UI Root"))
        .insert(ConstructUIRoot)
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::SpaceAround,
                        align_self: AlignSelf::Center,
                        margin: UiRect::left(Val::Percent(25.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.13, 0.14, 0.26).into(),
                    ..default()
                })
                .insert(Name::new("Layout"))
                .with_children(|commands| {
                    // The left half
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Name::new("Left side container"))
                        .with_children(|commands| {
                            // Stats container
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.17, 0.19, 0.36).into(),
                                    ..default()
                                })
                                .insert(Name::new("Stats container"))
                                .with_children(|commands| {
                                    // Balance text
                                    commands
                                        .spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::all(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                format!("Balance: ${}", inventory.balance),
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    // Font size 40 looked nice on my own screen height of 2880, which is a ratio of 1:72
                                                    font_size: physical_screen_height / 72.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(BalanceText)
                                        .insert(Name::new("Balance text"));
                                    // Market Text
                                    // TODO: Swap out this "Market Text" with a daily "discounted item"
                                    commands
                                        .spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::all(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                "[[ The Construct Shop ]]".to_string(),
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: physical_screen_height / 72.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(Name::new("Market Indicator text"));
                                });

                            // Market grid container
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceAround,
                                        flex_wrap: FlexWrap::Wrap,
                                        ..default()
                                    },
                                    focus_policy: bevy::ui::FocusPolicy::Block,
                                    background_color: Color::rgb(0.17, 0.19, 0.36).into(),
                                    ..default()
                                })
                                .insert(Name::new("Construct selection grid container"))
                                .with_children(|commands| {
                                    for i in 0..=29 {
                                        commands
                                            .spawn(ButtonBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0 / (5.0 / 0.9)),
                                                        Val::Percent(100.0 / (6.0 / 0.9)),
                                                    ),
                                                    justify_content: JustifyContent::Center,
                                                    ..default()
                                                },
                                                background_color: Color::rgb(0.22, 0.25, 0.48).into(),
                                                ..default()
                                            })
                                            .insert(Name::new(format!("Button {i}")))
                                            .insert(ConstructItemButton {
                                                building_type: {
                                                    if i < construct_inventory.items.len() {
                                                        Some(construct_inventory.items[i].building_type)
                                                    } else {
                                                        None
                                                    }
                                                },
                                            })
                                            .with_children(|commands| {
                                                // Item icon
                                                commands.spawn(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Percent(5.0 * (100.0 / 6.0)),
                                                            Val::Percent(5.0 * (100.0 / 5.0)),
                                                        ),
                                                        ..default()
                                                    },
                                                    image: UiImage {
                                                        texture: {
                                                            if i < construct_inventory.items.len() {
                                                                match construct_inventory.items[i].building_type {
                                                                    BuildingType::CandyShop => {
                                                                        item_icons.candy_shop.clone()
                                                                    }
                                                                    BuildingType::CoffeeShop => {
                                                                        item_icons.coffee_shop.clone()
                                                                    }
                                                                    BuildingType::Tree => item_icons.tree.clone(),
                                                                    BuildingType::Factory => item_icons.factory.clone(),
                                                                    _ => item_icons.empty.clone(),
                                                                }
                                                            } else {
                                                                item_icons.empty.clone()
                                                            }
                                                        },
                                                        flip_x: false,
                                                        flip_y: false,
                                                    },
                                                    transform: Transform::from_scale(Vec3::splat(0.7)),
                                                    ..default()
                                                });
                                                // Quantity text
                                                commands
                                                    .spawn(TextBundle {
                                                        style: Style {
                                                            size: Size::new(Val::Percent(25.0), Val::Percent(30.0)),
                                                            position_type: PositionType::Absolute,
                                                            position: UiRect::new(
                                                                Val::Percent(5.0),
                                                                Val::Percent(0.0),
                                                                Val::Percent(65.0),
                                                                Val::Percent(0.0),
                                                            ),
                                                            ..default()
                                                        },
                                                        text: Text::from_section(
                                                            {
                                                                if i < construct_inventory.items.len() {
                                                                    construct_inventory.items[i].quantity.to_string()
                                                                } else {
                                                                    "-".to_string()
                                                                }
                                                            },
                                                            TextStyle {
                                                                font: asset_server.load("font.otf"),
                                                                font_size: physical_screen_height / 108.0,
                                                                color: Color::WHITE,
                                                            },
                                                        ),
                                                        ..default()
                                                    })
                                                    .insert(MiniQuantityText)
                                                    .insert(Name::new("Quantity text"));
                                            });
                                    }
                                });
                        });

                    // Item stats container
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(45.0), Val::Percent(90.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::rgb(0.17, 0.19, 0.36).into(),
                            ..default()
                        })
                        .insert(Name::new("Item stats container"))
                        .with_children(|commands| {
                            // Item name
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::top(Val::Percent(5.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(Name::new("Item name text wrapper"))
                                .with_children(|commands| {
                                    commands
                                        .spawn(TextBundle::from_section(
                                            "Item name",
                                            TextStyle {
                                                font: asset_server.load("font.otf"),
                                                font_size: physical_screen_height / 60.0,
                                                color: Color::WHITE,
                                            },
                                        ))
                                        .insert(ItemStatsName)
                                        .insert(Name::new("Item name text"));
                                });

                            // Item image
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(Name::new("Item image wrapper"))
                                .with_children(|commands| {
                                    commands
                                        .spawn(ImageBundle {
                                            image: item_icons.empty.clone().into(),
                                            style: Style {
                                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                                ..default()
                                            },
                                            transform: Transform::from_scale(Vec3::splat(0.8)),
                                            ..default()
                                        })
                                        .insert(ItemStatsImage)
                                        .insert(Name::new("Item image"));
                                });

                            // Quantity text
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::FlexStart,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(Name::new("Item quantity text wrapper"))
                                .with_children(|commands| {
                                    commands
                                        .spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::left(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                "Remaining: #",
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: physical_screen_height / 90.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(ItemStatsQuantity)
                                        .insert(Name::new("Item quantity text"));
                                });

                            // Buy price text
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::FlexStart,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(Name::new("Item sell price text wrapper"))
                                .with_children(|commands| {
                                    commands
                                        .spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::left(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                "Price: $",
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: physical_screen_height / 90.0,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(ItemStatsBuyPrice)
                                        .insert(Name::new("Item sell price text"));
                                });

                            // Buy button
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    commands
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::rgb(0.22, 0.25, 0.48).into(),
                                            ..default()
                                        })
                                        .insert(BuyButton)
                                        .insert(Name::new("Buy button"))
                                        .with_children(|commands| {
                                            commands.spawn(TextBundle::from_section(
                                                "BUY",
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: physical_screen_height / 60.0,
                                                    color: Color::WHITE,
                                                },
                                            ));
                                        });
                                });
                        });
                });
        });
}

fn undraw_construct(
    mut commands: Commands,
    ui_root: Query<Entity, With<ConstructUIRoot>>,
    mut selected_item_stats: ResMut<SelectedItemStats>,
) {
    *selected_item_stats = SelectedItemStats::default();
    for entity in ui_root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Resource, Default)]
pub struct BuildingStash(pub Option<BuildingType>);

#[allow(clippy::complexity)]
fn buy_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BuyButton>)>,
    selected_item_stats: ResMut<SelectedItemStats>,
    mut inventory: ResMut<Inventory>,
    mut construct_inventory: ResMut<ConstructInventory>,
    mut building_stash: ResMut<BuildingStash>,
    mut next_construct_state: ResMut<NextState<ConstructPhase>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
    mut send_change_camera_state_event: EventWriter<ChangeCameraStateEvent>,
) {
    for (interaction, mut background_colour) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                if inventory.balance >= selected_item_stats.buy_price {
                    // Buy it
                    let item = construct_inventory
                        .items
                        .iter_mut()
                        .find(|item| item.building_type == selected_item_stats.building_type.unwrap())
                        .unwrap();

                    if item.quantity > 0 {
                        item.quantity -= 1;
                        inventory.balance -= item.price;

                        building_stash.0 = Some(item.building_type);
                        next_construct_state.set(ConstructPhase::Preview);
                        next_ui_state.set(UiState::None);
                        send_change_camera_state_event.send(ChangeCameraStateEvent(CameraState::ConstructPreview));
                    }
                }
            }
            Interaction::Hovered => *background_colour = Color::rgb(0.34, 0.37, 0.60).into(),
            _ => *background_colour = Color::rgb(0.22, 0.25, 0.48).into(),
        }
    }
}

struct ChangeItemStatsEvent {
    name: String,
    image: Handle<Image>,
    buy_price: u32,
}

#[derive(Resource, Default)]
struct SelectedItemStats {
    building_type: Option<BuildingType>,
    buy_price: u32,
}

#[allow(clippy::complexity)]
fn item_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ConstructItemButton),
        (Changed<Interaction>, With<ConstructItemButton>),
    >,
    construct_inventory: Res<ConstructInventory>,
    item_icons: Res<ItemIcons>,
    mut selected_item_stats: ResMut<SelectedItemStats>,
    mut send_change_item_stats_event: EventWriter<ChangeItemStatsEvent>,
) {
    for (interaction, mut background_colour, item_button_cmp) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                if let Some(building_type) = item_button_cmp.building_type {
                    let target_item = construct_inventory
                        .items
                        .iter()
                        .find(|item| item.building_type == building_type)
                        .unwrap();

                    *selected_item_stats = SelectedItemStats {
                        building_type: Some(building_type),
                        buy_price: target_item.price,
                    };

                    send_change_item_stats_event.send(ChangeItemStatsEvent {
                        name: target_item.name.clone(),
                        image: match building_type {
                            BuildingType::CandyShop => item_icons.candy_shop.clone(),
                            BuildingType::CoffeeShop => item_icons.coffee_shop.clone(),
                            BuildingType::Tree => item_icons.tree.clone(),
                            BuildingType::Factory => item_icons.factory.clone(),
                            _ => item_icons.empty.clone(),
                        },
                        buy_price: target_item.price,
                    });
                }
            }
            Interaction::Hovered => *background_colour = Color::rgb(0.34, 0.37, 0.60).into(),
            _ => *background_colour = Color::rgb(0.22, 0.25, 0.48).into(),
        }
    }
}

#[allow(clippy::complexity)]
fn change_item_stats(
    mut image: Query<&mut UiImage, With<ItemStatsImage>>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<ItemStatsName>>,
        Query<&mut Text, With<ItemStatsQuantity>>,
        Query<&mut Text, With<ItemStatsBuyPrice>>,
    )>,
    mut change_item_stats_events: EventReader<ChangeItemStatsEvent>,
    construct_inventory: Res<ConstructInventory>,
) {
    for event in change_item_stats_events.iter() {
        let mut image = image.single_mut();
        image.texture = event.image.clone();

        for mut name in param_set.p0().iter_mut() {
            name.sections[0].value = event.name.clone();
        }
        for mut quantity in param_set.p1().iter_mut() {
            let target_item = construct_inventory.items.iter().find(|item| item.name == event.name);

            if let Some(item) = target_item {
                quantity.sections[0].value = format!("Remaining: {}", item.quantity);
            }
        }
        for mut sell_price in param_set.p2().iter_mut() {
            sell_price.sections[0].value = format!("Price: ${}", event.buy_price);
        }
    }
}
