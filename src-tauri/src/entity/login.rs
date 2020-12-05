use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginAccessData {
  pub message: String,
  #[serde(rename = "userId")]
  pub user_id: i64,
  #[serde(rename = "countryIso")]
  pub country_iso: String,
  #[serde(rename = "countryCode")]
  pub country_code: String,
  #[serde(rename = "accountId")]
  pub account_id: i32,
  #[serde(rename = "server_time")]
  pub logon_server_time: i32,
  #[serde(rename = "resetUserData")]
  pub reset_user_data: bool,
  pub access_token: String,
  pub refresh_token: String,
  pub token_type: String,
  #[serde(rename = "autoLoginAccountId")]
  pub auto_login_email: String,
  #[serde(rename = "displayAccountId")]
  pub display_account_id: String,
  #[serde(rename = "mainDeviceAgentName")]
  pub main_device: String,
  #[serde(rename = "mainDeviceAppVersion")]
  pub main_device_app_version: String,
}
