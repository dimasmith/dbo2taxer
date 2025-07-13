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
        let config = config::Config::builder()
            .set_default("operation", default_config.operation)?
            .set_default("income_type", default_config.income_type)?
            .add_source(config::Environment::with_prefix("DBO2TAXER"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
