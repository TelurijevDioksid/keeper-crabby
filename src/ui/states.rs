pub struct LoginState {
    pub username: String,
    pub master_password: String,
}

impl LoginState {
    pub fn new() -> Self {
        LoginState {
            username: String::new(),
            master_password: String::new(),
        }
    }
}

pub enum ScreenState {
    Login(LoginState),
}
