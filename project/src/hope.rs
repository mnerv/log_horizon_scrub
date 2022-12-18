use postgres::Client;
use std::error::Error;

pub enum StoreMode {
    Command,
    StateMachine,
}

#[derive(Copy, Clone, PartialEq)]
pub enum HopeMode {
    Admin,
    Customer,
}

#[derive(Copy, Clone, PartialEq)]
pub enum LockStatus {
    LogIn,
    LogOut,
}

pub struct Login {
    id: i32,
    email: String,
    mode: HopeMode,
}
impl Login {
    pub fn new(id: i32, email: String, mode: HopeMode) -> Login {
        Login { id, email, mode }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn to_string(&self) -> String {
        format!("id: {}, email: {}", self.id, self.email).to_string()
    }
}

pub struct Hope {
    pub user: Login,
    pub status: LockStatus,
}

impl Hope {
    pub fn new() -> Hope {
        Hope {
            user: Login {
                id: 0,
                email: String::new(),
                mode: HopeMode::Customer,
            },
            status: LockStatus::LogOut,
        }
    }

    pub fn login(&mut self, user: Login) {
        self.user = user;
        self.status = LockStatus::LogIn;
    }

    pub fn logout(&mut self) {
        self.user.id = 0;
        self.user.email = String::new();
        self.user.mode = HopeMode::Customer;
        self.status = LockStatus::LogOut;
    }

    pub fn is_login(&mut self) -> bool {
        self.user.id != 0 && !self.user.email.is_empty()
    }
}
