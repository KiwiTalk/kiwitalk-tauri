use serde::Deserialize;

mod login;
pub use login::*;

#[derive(Deserialize)]
pub struct WebApiResponse<T> {
  pub status: WebApiStatus,
  #[serde(flatten)]
  pub data: Option<T>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WebApiStatus {
  Success,
  InvalidSession,
  OperationDenied,
  LoginFailedReason,
  LoginFailed,
  MobileUnregistered,
  DeviceNotRegistered,
  AnotherLogon,
  DeviceRegisterFailed,
  InvalidDeviceRegister,
  IncorrectPasscode,
  PasscodeRequestFailed,
  AccountRestricted,
  Other(i64),
}

impl WebApiStatus {
  pub fn code(&self) -> i64 {
    match self {
      WebApiStatus::Success => 0,
      WebApiStatus::InvalidSession => -950,
      WebApiStatus::OperationDenied => -500,
      WebApiStatus::LoginFailedReason => 12,
      WebApiStatus::LoginFailed => 30,
      WebApiStatus::MobileUnregistered => 32,
      WebApiStatus::DeviceNotRegistered => -100,
      WebApiStatus::AnotherLogon => -101,
      WebApiStatus::DeviceRegisterFailed => -102,
      WebApiStatus::InvalidDeviceRegister => -110,
      WebApiStatus::IncorrectPasscode => -111,
      WebApiStatus::PasscodeRequestFailed => -112,
      WebApiStatus::AccountRestricted => -997,
      WebApiStatus::Other(v) => *v,
    }
  }
}

impl<'de> Deserialize<'de> for WebApiStatus {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct IntVisitor;
    impl serde::de::Visitor<'_> for IntVisitor {
      type Value = WebApiStatus;

      fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "a number representing authentication status")
      }

      fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        use WebApiStatus::*;

        let status = match v {
          0 => Success,
          -950 => InvalidSession,
          -500 => OperationDenied,
          12 => LoginFailedReason,
          30 => LoginFailed,
          32 => MobileUnregistered,
          -100 => DeviceNotRegistered,
          -101 => AnotherLogon,
          -102 => DeviceRegisterFailed,
          -110 => InvalidDeviceRegister,
          -111 => IncorrectPasscode,
          -112 => PasscodeRequestFailed,
          -997 => AccountRestricted,
          _ => Other(v),
        };
        Ok(status)
      }
    }

    deserializer.deserialize_i32(IntVisitor)
  }
}
