use crate::util::app_env::Environment;

pub fn load_configuration<'a, T: serde::Deserialize<'a>>() -> Result<T, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    // Read the "default" configuration file
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Detect the running environment.
    // Default to `development` if unspecified.
    let environment = Environment::get_current_env();

    // Layer on the environment-specific values.
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    // Add in settings from environment variables (with a prefix of SAASFACTORY and
    // '_' as separator) E.g. `SAASFACTORY_APPLICATION_PORT=5001 would set
    // `Settings.application.port`
    settings.merge(config::Environment::with_prefix("saasfactory").separator("_"))?;

    settings.try_into()
}
