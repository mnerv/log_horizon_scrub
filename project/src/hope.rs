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

#[derive(Clone)]
pub struct Address {
    id: i32,
    street: String,
    city: String,
    country: String,
    telephone: String
}
impl Address {
    pub fn new(id: i32, street: String, city: String, country: String, telephone: String) -> Address {
        Address { id, street, city, country, telephone }
    }

    pub fn default() -> Address {
        Address {
            id: 0,
            street: String::new(),
            city: String::new(),
            country: String::new(),
            telephone: String::new(),
        }
    }
}

pub trait LoginTrait {
    fn id(&self) -> i32;
    fn logout(&mut self);
    fn is_login(&mut self) -> bool;
    fn to_string(&self) -> String;
}

#[derive(Clone)]
pub struct Admin {
    id: i32,
    email: String,
}
impl Admin {
    pub fn new(id: i32, email: String) -> Admin {
        Admin { id, email }
    }

    pub fn default() -> Admin {
        Admin { id: 0, email: "".to_string() }
    }

    pub fn login(&mut self, admin: &mut Admin) {
        self.id = admin.id;
        self.email = admin.email.to_owned();
    }
}
impl LoginTrait for Admin {
    fn id(&self) -> i32 {
        self.id
    }

    fn logout(&mut self) {
        self.id = 0;
        self.email = String::new();
    }

    fn is_login(&mut self) -> bool {
        self.id != 0 || !self.email.is_empty()
    }

    fn to_string(&self) -> String {
        format!("id: {}, email: {}", self.id, self.email)
    }
}

#[derive(Clone)]
pub struct Customer {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    address: Address
}
impl Customer {
    pub fn new(id: i32, first_name: String, last_name: String, email: String, address: Address) -> Customer {
        Customer { id, first_name, last_name, email, address }
    }

    pub fn default() -> Customer {
        Customer {
            id: 0,
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            address: Address::default(),
        }
    }

    pub fn login(&mut self, customer: &Customer) {
        self.id = customer.id;
        self.first_name = customer.first_name.to_owned();
        self.last_name = customer.last_name.to_owned();
        self.email = customer.email.to_owned();
        self.address = customer.address.to_owned();
    }
}
impl LoginTrait for Customer {
    fn id(&self) -> i32 {
        self.id
    }

    fn logout(&mut self) {
        self.id = 0;
        self.first_name = String::new();
        self.last_name  = String::new();
        self.email   = String::new();
        self.address = Address::default();
    }

    fn is_login(&mut self) -> bool {
        self.id != 0 && !self.email.is_empty()
    }

    fn to_string(&self) -> String {
        format!("id: {}, email: {}", self.id, self.email)
    }
}