pub struct LoginState {
    pub username: String,
    pub master_password: String,
}

impl LoginState {
    pub fn username_append(&mut self, c: char) {
        self.username.push(c);
    }

    pub fn master_password_append(&mut self, c: char) {
        self.master_password.push(c);
    }

    pub fn username_pop(&mut self) {
        self.username.pop();
    }

    pub fn master_password_pop(&mut self) {
        self.master_password.pop();
    }
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
