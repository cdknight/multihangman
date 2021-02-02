
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IpUsernamePair {
    ip: String,
    username: String
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    recent_ips: Vec<IpUsernamePair>,

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

    pub fn add_configset(&mut self, ip: &str, username: &str, password: &str) {
        self.recent_ips.push( IpUsernamePair { ip: ip.to_string(), username: username.to_string() });
        let keyring = Keyring::new(SERVICE, username);
        keyring.set_password(password);

        let toml = toml::to_string(&self).unwrap();
        fs::write(&self.file_name, &toml);
    }

    pub fn remove_ip(&mut self, i: usize) {
        self.recent_ips.remove(i);

        let toml = toml::to_string(&self).unwrap();
        fs::write(&self.file_name, &toml);
    }
}

static SERVICE: &'static str = "hangmanudpclient";
lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::from_file("ClientConfiguration.toml".to_string()));
}
