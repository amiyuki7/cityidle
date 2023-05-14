use crate::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub struct MarketInventory {
    pub items: [Item; 6],
    pub balance: u32,
}

impl Default for MarketInventory {
    fn default() -> Self {
        Self {
            items: [
                Item::new(ItemType::Taffy, 26),
                Item::new(ItemType::Nougat, 14),
                Item::new(ItemType::Marshmallow, 3),
                Item::new(ItemType::Coffee, 8),
                Item::new(ItemType::Cocoa, 9),
                Item::new(ItemType::Milkshake, 18),
            ],
            balance: 100,
        }
    }
}

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketInventory>()
            .init_resource::<SelectedItemStats>()
            .add_event::<ChangeItemStatsEvent>()
            .add_event::<IncrementEvent>()
            .add_system(draw_market.in_schedule(OnEnter(UiState::Market)))
            .add_system(undraw_market.in_schedule(OnExit(UiState::Market)))
            // .add_system(exit_uistate.in_set(OnUpdate(UiState::Market)))
            .add_systems(
                (
                    exit_uistate,
                    item_button_interaction,
                    increment_button_interaction,
                    change_item_stats,
                    change_buy_quantity,
                    buy_button_interaction,
                )
                    .in_set(OnUpdate(UiState::Market)),
            );
    }
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
#[derive(Component, Reflect)]
struct MenuUIRoot;

// Marker
#[derive(Component)]
struct BalanceText;

#[derive(Component, Reflect)]
struct MarketItemButton {
    item_type: Option<ItemType>,
}

#[derive(Component)]
struct MiniQuantityText {
    item_type: Option<ItemType>,
}

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
struct ItemStatsBuyQuantity {
    quantity: u32,
    buy_allowed: bool,
}

// Marker
#[derive(Component)]
struct BuyButton;

/// Lots of code duplication from ./inventory.rs - refactor common parts after making it work
fn draw_market(
    mut commands: Commands,
    inventory: Res<Inventory>,
    market_inventory: Res<MarketInventory>,
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
        .insert(Name::new("MarketUIRoot"))
        .insert(MenuUIRoot)
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
                    background_color: Color::rgb(0.96, 0.83, 0.39).into(),
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
                                                "[[ The Market ]]".to_string(),
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
                                .insert(Name::new("Market grid container"))
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
                                            .insert(MarketItemButton {
                                                item_type: {
                                                    if i < market_inventory.items.len() {
                                                        Some(market_inventory.items[i].item_type)
                                                    } else {
                                                        None
                                                    }
                                                },
                                            })
                                            .with_children(|commands| {
                                                // Item icon
                                                commands.spawn(ImageBundle {
                                                    style: Style {
                                                        // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                                        size: Size::new(
                                                            Val::Percent(5.0 * (100.0 / 6.0)),
                                                            Val::Percent(5.0 * (100.0 / 5.0)),
                                                        ),
                                                        ..default()
                                                    },
                                                    image: UiImage {
                                                        texture: {
                                                            if i < market_inventory.items.len() {
                                                                match market_inventory.items[i].item_type {
                                                                    ItemType::Taffy => item_icons.taffy.clone(),
                                                                    ItemType::Nougat => item_icons.nougat.clone(),
                                                                    ItemType::Marshmallow => {
                                                                        item_icons.marshmallow.clone()
                                                                    }
                                                                    ItemType::Coffee => item_icons.coffee.clone(),
                                                                    ItemType::Cocoa => item_icons.cocoa.clone(),
                                                                    ItemType::Milkshake => item_icons.milkshake.clone(),
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
                                                                if i < market_inventory.items.len() {
                                                                    market_inventory.items[i].quantity.to_string()
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
                                                    .insert(MiniQuantityText {
                                                        item_type: {
                                                            if i < market_inventory.items.len() {
                                                                Some(market_inventory.items[i].item_type)
                                                            } else {
                                                                None
                                                            }
                                                        },
                                                    })
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
                                        size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
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
                                                "Quantity: #",
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
                                        size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
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

                            // Quantity selector
                            commands
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|commands| {
                                    spawn_quantity_increment_button(
                                        commands,
                                        &asset_server,
                                        -10,
                                        physical_screen_height,
                                    );

                                    spawn_quantity_increment_button(
                                        commands,
                                        &asset_server,
                                        -1,
                                        physical_screen_height,
                                    );

                                    // Quantity selected
                                    commands
                                        .spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::new(
                                                    Val::Percent(5.0),
                                                    Val::Percent(5.0),
                                                    Val::Percent(0.0),
                                                    Val::Percent(0.0),
                                                ),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                "0",
                                                TextStyle {
                                                    font: asset_server.load("font.otf"),
                                                    font_size: physical_screen_height / 60.0,
                                                    color: Color::GREEN,
                                                },
                                            ),
                                            ..default()
                                        })
                                        .insert(ItemStatsBuyQuantity {
                                            quantity: 0,
                                            buy_allowed: false,
                                        });

                                    spawn_quantity_increment_button(commands, &asset_server, 1, physical_screen_height);

                                    spawn_quantity_increment_button(
                                        commands,
                                        &asset_server,
                                        10,
                                        physical_screen_height,
                                    );
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
                                        .insert(Name::new("Sell button"))
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

fn undraw_market(
    mut commands: Commands,
    ui_root: Query<Entity, With<MenuUIRoot>>,
    mut selected_item_stats: ResMut<SelectedItemStats>,
) {
    *selected_item_stats = SelectedItemStats::default();
    for entity in ui_root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct IncrementButton(i8);

struct IncrementEvent(i8);

#[allow(clippy::complexity)]
fn increment_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &IncrementButton),
        (Changed<Interaction>, With<IncrementButton>),
    >,
    mut send_increment_event: EventWriter<IncrementEvent>,
) {
    for (interaction, mut background_colour, IncrementButton(amount)) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                send_increment_event.send(IncrementEvent(*amount));
            }
            Interaction::Hovered => *background_colour = Color::rgb(0.25, 0.26, 0.38).into(),
            _ => *background_colour = Color::rgb(0.13, 0.14, 0.26).into(),
        }
    }
}

fn change_buy_quantity(
    mut increment_events: EventReader<IncrementEvent>,
    selected_item_stats: Res<SelectedItemStats>,
    mut buy_quantity_query: Query<(&mut Text, &mut ItemStatsBuyQuantity)>,
    inventory: Res<Inventory>,
    market_inventory: Res<MarketInventory>,
) {
    for event in increment_events.iter() {
        if let Ok((mut text, mut buy_quantity)) = buy_quantity_query.get_single_mut() {
            let IncrementEvent(amount) = *event;

            if amount.is_positive() {
                buy_quantity.quantity += amount as u32;
            } else {
                // Negative value - decrement
                let abs_decrement = amount.unsigned_abs() as u32;
                if abs_decrement <= buy_quantity.quantity {
                    buy_quantity.quantity -= abs_decrement;
                } else {
                    // Subtracting too much sets it to 0, e.g. subtracting 10 from 7
                    buy_quantity.quantity = 0;
                }
            }

            text.sections[0].value = buy_quantity.quantity.to_string();

            let market_item = market_inventory
                .items
                .iter()
                .find(|item| item.item_type == selected_item_stats.item_type.unwrap())
                .unwrap();

            if inventory.balance >= buy_quantity.quantity * selected_item_stats.buy_price
                && selected_item_stats.item_type.is_some()
                && market_item.quantity >= buy_quantity.quantity
            {
                text.sections[0].style.color = Color::GREEN;
                buy_quantity.buy_allowed = true;
            } else {
                text.sections[0].style.color = Color::RED;
                buy_quantity.buy_allowed = false;
            }
        }
    }
}

#[allow(clippy::complexity)]
fn buy_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BuyButton>)>,
    mut selected_item_stats: ResMut<SelectedItemStats>,
    mut inventory: ResMut<Inventory>,
    mut market_inventory: ResMut<MarketInventory>,
    mut param_set: ParamSet<(
        Query<(&mut Text, &mut ItemStatsBuyQuantity)>,
        Query<&mut Text, With<ItemStatsQuantity>>,
        Query<&mut Text, With<BalanceText>>,
        Query<(&mut Text, &MiniQuantityText)>,
    )>,
) {
    for (interaction, mut background_colour) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => 'onclick: {
                if let Ok((mut text, mut buy_quantity)) = param_set.p0().get_single_mut() {
                    if !buy_quantity.buy_allowed || buy_quantity.quantity == 0 {
                        break 'onclick;
                    }
                    // Inventory: add to balance, subtract from items
                    inventory.balance -= selected_item_stats.buy_price * buy_quantity.quantity;
                    let balance = inventory.balance;

                    let item = inventory
                        .items
                        .iter_mut()
                        .find(|item| item.item_type == selected_item_stats.item_type.unwrap())
                        .unwrap();

                    let market_item = market_inventory
                        .items
                        .iter_mut()
                        .find(|item| item.item_type == selected_item_stats.item_type.unwrap())
                        .unwrap();

                    item.quantity += buy_quantity.quantity;
                    market_item.quantity -= buy_quantity.quantity;

                    selected_item_stats.quantity = item.quantity;

                    // Reset quantiy select text & state
                    buy_quantity.quantity = 0;
                    buy_quantity.buy_allowed = true;
                    text.sections[0].value = "0".to_string();
                    text.sections[0].style.color = Color::GREEN;

                    // Reset quantity text
                    if let Ok(mut quantity_text) = param_set.p1().get_single_mut() {
                        quantity_text.sections[0].value = format!("Quantity: {}", item.quantity);
                    }

                    // Update balance text
                    if let Ok(mut balance_text) = param_set.p2().get_single_mut() {
                        balance_text.sections[0].value = format!("Balance: ${}", balance);
                    }

                    // Update mini quantity text
                    for (mut text, MiniQuantityText { item_type }) in param_set.p3().iter_mut() {
                        if *item_type == selected_item_stats.item_type {
                            text.sections[0].value = market_item.quantity.to_string();
                            break;
                        }
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
    quantity: u32,
    buy_price: u32,
}

#[derive(Resource, Default)]
struct SelectedItemStats {
    item_type: Option<ItemType>,
    quantity: u32,
    buy_price: u32,
}

#[allow(clippy::complexity)]
fn item_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MarketItemButton),
        (Changed<Interaction>, With<MarketItemButton>),
    >,
    market_inventory: Res<MarketInventory>,
    item_icons: Res<ItemIcons>,
    mut selected_item_stats: ResMut<SelectedItemStats>,
    mut send_change_item_stats_event: EventWriter<ChangeItemStatsEvent>,
) {
    for (interaction, mut background_colour, item_button_cmp) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                if let Some(item_type) = item_button_cmp.item_type {
                    let target_item = market_inventory
                        .items
                        .iter()
                        .find(|item| item.item_type == item_type)
                        .unwrap();

                    *selected_item_stats = SelectedItemStats {
                        item_type: Some(item_type),
                        quantity: target_item.quantity,
                        buy_price: target_item.base_buy_price,
                    };

                    send_change_item_stats_event.send(ChangeItemStatsEvent {
                        name: target_item.name.clone(),
                        image: match item_type {
                            ItemType::Taffy => item_icons.taffy.clone(),
                            ItemType::Nougat => item_icons.nougat.clone(),
                            ItemType::Marshmallow => item_icons.marshmallow.clone(),
                            ItemType::Coffee => item_icons.coffee.clone(),
                            ItemType::Cocoa => item_icons.cocoa.clone(),
                            ItemType::Milkshake => item_icons.milkshake.clone(),
                        },
                        quantity: target_item.quantity,
                        buy_price: target_item.base_buy_price,
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
        Query<(&mut Text, &mut ItemStatsBuyQuantity)>,
    )>,
    mut change_item_stats_events: EventReader<ChangeItemStatsEvent>,
) {
    for event in change_item_stats_events.iter() {
        let mut image = image.single_mut();
        image.texture = event.image.clone();

        for mut name in param_set.p0().iter_mut() {
            name.sections[0].value = event.name.clone();
        }
        for mut quantity in param_set.p1().iter_mut() {
            quantity.sections[0].value = format!("Quantity: {}", event.quantity);
        }
        for mut sell_price in param_set.p2().iter_mut() {
            sell_price.sections[0].value = format!("Price: ${}", event.buy_price);
        }

        // Reset sell quantity text
        if let Ok((mut text, mut sell_quantity)) = param_set.p3().get_single_mut() {
            sell_quantity.quantity = 0;
            sell_quantity.buy_allowed = true;
            text.sections[0].value = "0".to_string();
            text.sections[0].style.color = Color::GREEN;
        }
    }
}

fn spawn_quantity_increment_button(
    commands: &mut ChildBuilder,
    asset_server: &AssetServer,
    amount: i8,
    physical_screen_height: f32,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(10.0), Val::Percent(50.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::new(
                    Val::Percent(2.0),
                    Val::Percent(2.0),
                    Val::Percent(0.0),
                    Val::Percent(0.0),
                ),
                ..default()
            },
            background_color: Color::rgb(0.13, 0.14, 0.26).into(),
            ..default()
        })
        .insert(IncrementButton(amount))
        .with_children(|commands| {
            commands.spawn(TextBundle::from_section(
                format!("{}{}", if amount.is_positive() { "+" } else { "-" }, amount.abs()),
                TextStyle {
                    font: asset_server.load("font.otf"),
                    font_size: physical_screen_height / 90.0,
                    color: Color::WHITE,
                },
            ));
        });
}