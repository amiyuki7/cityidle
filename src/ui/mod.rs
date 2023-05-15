mod construct;
mod inventory;
mod items;
mod market;
mod upgrade;
pub use crate::*;
pub use construct::*;
pub use inventory::*;
pub use items::*;
pub use market::*;
pub use upgrade::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Reflect)]
pub enum UiState {
    Inventory,
    // CityCentreInfo,
    Market,
    Construct,
    Upgrade,
    #[default]
    None,
}
