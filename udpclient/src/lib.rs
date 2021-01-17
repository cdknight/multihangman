
pub mod newgamewizard;
pub mod game;
pub mod hangmanclient;
pub mod opening;
pub mod joingame;
pub mod textbox;
pub mod raylibscene;
pub mod resources;
pub mod connect;

pub use raylibscene::RaylibScene; // Re-import

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Scenes {
    JoinGameScene, OpeningScene, NewGameWizardScene, GameScene, None
}

