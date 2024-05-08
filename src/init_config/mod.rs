pub struct Config {
    pub file_path: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        Ok(Config {
            file_path: args.next(),
        })
    }
}
