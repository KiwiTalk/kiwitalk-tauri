use openssl::sha::sha512;
use thiserror::Error;

use crate::{
  config::Config,
  entity::{LoginAccessData, WebApiResponse, WebApiStatus},
};

pub struct Client {
  login_access_data: Option<LoginAccessData>,
  device_uuid: String,
  device_name: String,
  config: Config,
  web: ureq::Agent,
}

#[derive(Error, Debug)]
pub enum LoginError {
  #[error("The client has already logged in")]
  AlreadyLoggedIn,
  #[error("Could not send request: {0}")]
  SystemFail(#[from] std::io::Error),
}

impl Client {
  pub fn new(config: Config) -> Self {
    let web = ureq::agent()
      .set("User-Agent", &config.user_agent())
      .build();

    Self {
      login_access_data: Default::default(),
      device_uuid: Default::default(),
      device_name: Default::default(),
      config,
      web,
    }
  }

  pub fn is_logged_in(&self) -> bool {
    self.login_access_data.is_some()
  }

  pub fn login(
    &mut self,
    email: &str,
    password: &str,
    forced: bool,
  ) -> Result<WebApiStatus, LoginError> {
    if self.is_logged_in() {
      return Err(LoginError::AlreadyLoggedIn);
    }

    let form = self.create_login_form(email, password, false, forced);
    let xvc = self.xvc_key(&self.config.user_agent(), email);

    let url = format!(
      "https://katalk.kakao.com/{}",
      self.config.account_api_path("login.json")
    );

    let response: WebApiResponse<LoginAccessData> = self
      .web
      .post(&url)
      .set("X-VC", &xvc)
      .send_form(&form)
      .into_json_deserialize()?;

    self.login_access_data = response.data;

    Ok(response.status)
  }

  fn xvc_key(&self, user_agent: &str, email: &str) -> String {
    let mut full_key = self.full_xvc_key(user_agent, email);
    full_key.truncate(16);
    full_key
  }

  fn full_xvc_key(&self, user_agent: &str, email: &str) -> String {
    let source = format!(
      "{seed0}|{user_agent}|{seed1}|{email}|{device_uuid}",
      seed0 = self.config.xvc_seeds[0],
      user_agent = user_agent,
      seed1 = self.config.xvc_seeds[1],
      email = email,
      device_uuid = self.device_uuid,
    );

    let hash = sha512(source.as_ref());
    hex::encode(hash)
  }

  fn create_login_form<'a>(
    &'a self,
    email: &'a str,
    password: &'a str,
    permanent: bool,
    forced: bool,
  ) -> Vec<(&str, &'a str)> {
    let mut form = vec![
      ("email", email),
      ("password", password),
      ("device_uuid", &self.device_uuid),
      ("os_version", &self.config.os_version),
      ("device_name", &self.device_name),
    ];

    if permanent {
      form.push(("permanent", "true"));
    }

    if forced {
      form.push(("forced", "true"));
    }

    form
  }
}
