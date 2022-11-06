
pub static mut CONFIG: Config = Config::default();


#[derive(serde_derive::Deserialize)]
pub struct Config {
    pub color: Color
}

#[derive(serde_derive::Deserialize)]
pub struct Color {
    pub bg: (u8, u8, u8),
    pub p_dynamic: (u8, u8, u8),
    pub p_static: (u8, u8, u8)
}


pub fn load(path: &str) {
    let content = std::fs::read_to_string(path).unwrap();
    unsafe {
        CONFIG = match toml::from_str(&content) {
            Ok(c) => c,
            Err(_) => {
                println!("Unable to load config. Default config loaded.");
                Config::default()
            }
        }
    } 
}

impl Config {
    pub const fn default() -> Self {
        Self {
            color: Color {
                bg: (0, 0, 0),
                p_dynamic: (0, 255, 255),
                p_static: (255, 0, 0)
            }
        }
    }
}
