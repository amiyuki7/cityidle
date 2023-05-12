mod inventory;
mod items;
pub use crate::*;
pub use inventory::*;
pub use items::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum UiState {
    Inventory,
    BuildingInfo,
    ItemShop,
    BuildingShop,
    #[default]
    None,
}
