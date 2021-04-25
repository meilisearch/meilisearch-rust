#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) host: String,
    pub(crate) api_key: String,
}

impl Config {

    pub fn new<S: AsRef<str>>(host: S, api_key: S) -> Self {
        Self { host: host.as_ref().into(), api_key: api_key.as_ref().into() }
    }

}