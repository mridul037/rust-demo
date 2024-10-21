pub fn login(cred: crate::database::Credentials) {
    crate::database::get_user();
  }
  
  pub fn connect_to_db() -> crate::database::Status {
    crate::database::Status::Connected
  }
  