
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

#[macro_use]
extern crate lazy_static;
extern crate keyring;

use serde::{Serialize, Deserialize};
use hangmanstructs::Configurable;
use std::fs;
use std::sync::RwLock;
use keyring::Keyring;



#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    recent_ips: Vec<String>,
    username: String,
    password: String,

    #[serde(skip)]
    pub file_name: String
}

impl Configurable<Config> for Config {
    fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }
    fn file_name(&self) -> String {
        self.file_name.clone()
    }
}

impl Config {

    pub fn add_ip(&mut self, ip: &str) {
        self.recent_ips.push(ip.to_string());

        let toml = toml::to_string(&self).unwrap();
        fs::write(&self.file_name, &toml);
    }

    pub fn remove_ip(&mut self, i: usize) {
        self.recent_ips.remove(i);

        let toml = toml::to_string(&self).unwrap();
        fs::write(&self.file_name, &toml);
    }
}

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::from_file("ClientConfiguration.toml".to_string()));
}
