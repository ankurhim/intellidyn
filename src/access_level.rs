pub enum AccessLevel {
    Administrator,
    Manager,
    Supervisor
}

pub struct Engineering(AccessLevel);