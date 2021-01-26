use serde::{Serialize, Deserialize};
use hangmanstructs::Configurable;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServerConfig {
    pub db_url: String,
    pub secret_key: String,

    #[serde(skip)]
    pub file_name: String
}

impl Configurable<ServerConfig> for ServerConfig {
    fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }
    fn file_name(&self) -> String {
        self.file_name.clone()
    }
}
