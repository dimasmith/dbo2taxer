use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct TaxerConfig {
    #[serde(default)]
    pub operation: String,
    #[serde(default)]
    pub income_type: String,
    #[serde(default)]
    pub account_name: String,
}

impl Default for TaxerConfig {
    fn default() -> Self {
        Self {
            operation: "Дохід".to_string(),
            income_type: "Основний дохід".to_string(),
            account_name: "".to_string(),
        }
    }
}

impl TaxerConfig {
    pub fn from_configuration() -> anyhow::Result<Self> {
        let default_config = TaxerConfig::default();
        let mut config = config::Config::builder()
            .set_default("operation", default_config.operation)?
            .set_default("income_type", default_config.income_type)?;

        if let Some(project_dirs) = ProjectDirs::from("", "", "dbo2taxer") {
            let config_dir = project_dirs.config_dir();
            if let Some(dir_name) = config_dir.to_str() {
                config = config.add_source(config::File::with_name(dir_name).required(false));
            }
        }

        config = config.add_source(config::Environment::with_prefix("DBO2TAXER"));

        let config = config.build()?;
        Ok(config.try_deserialize()?)
    }
}
