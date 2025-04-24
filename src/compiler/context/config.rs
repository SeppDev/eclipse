pub const CONFIG_NAME: &str = "eclipse";

pub struct Config {
    pub package: Package,
    pub editor: Editor,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            package: Package {
                name: "Test".into(),
                version: "0.0.0-test".into(),
            },
            editor: Editor { tab_size: 4 },
        }
    }
}

#[derive(Default)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Default)]
pub struct Editor {
    pub tab_size: usize,
}
