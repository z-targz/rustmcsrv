use std::path::Path;

pub enum PluginEnableError {
    ApiVersionError,
    Panic,
}

pub trait TraitPlugin {
    fn get_data_folder(&self) -> Path;
    fn get_version(&self) -> String;
    fn get_description(&self) -> String;
    fn get_api_version(&self) -> u16;
    fn on_enable(&mut self) -> Result<(), PluginEnableError>;
    fn on_disable(&mut self);
}

pub struct PluginYML {
    plugin_name: String,
    plugin_version: String,
    plugin_description: String,
    api_version: u16,
    depends: Vec<(String, (u16, u16, u16))>,
}