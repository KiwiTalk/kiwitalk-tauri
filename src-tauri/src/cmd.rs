use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  Login {
    email: String,
    password: String,
    #[serde(default)]
    forced: bool,
    callback: String,
    error: String,
  },
}
