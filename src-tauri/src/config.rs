use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
  pub agent: String,
  pub version: String,
  pub os_version: String,
  pub language: String,
  pub xvc_seeds: [String; 2],
}

impl Default for Config {
  fn default() -> Self {
    Self {
      agent: "win32".into(),
      version: "3.1.4".into(),
      os_version: "10.0".into(),
      language: "ko".into(),
      xvc_seeds: ["HEATH".into(), "DEMIAN".into()],
    }
  }
}

impl Config {
  pub fn user_agent(&self) -> String {
    format!(
      "KT/{} Wd/{} {}",
      self.version, self.os_version, self.language
    )
  }

  pub fn account_api_path(&self, api: &str) -> String {
    format!("{}/account/{}", self.agent, api)
  }
}
