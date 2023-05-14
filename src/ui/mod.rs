mod inventory;
mod items;
mod market;
pub use crate::*;
pub use inventory::*;
pub use items::*;
pub use market::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum UiState {
    Inventory,
    BuildingInfo,
    CityCentreInfo,
    Market,
    BuildingShop,
    #[default]
    None,
}
