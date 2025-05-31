use serde::Deserialize;

const DEFAULT: &str = r#"http_addr = "127.0.0.1:8080"
database_url = "postgres://postgres:postgres@localhost:5432/hoyo_sdk"
"#;

#[derive(Deserialize)]
pub struct SdkConfig {
    pub http_addr: String,
    pub database_url: String,
}

pub fn load_or_create(path: &str) -> SdkConfig {
    std::fs::read_to_string(path).map_or_else(
        |_| {
            std::fs::write(path, DEFAULT).unwrap();
            toml::from_str(DEFAULT).unwrap()
        },
        |data| toml::from_str(&data).unwrap(),
    )
}
