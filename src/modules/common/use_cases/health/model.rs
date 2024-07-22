use serde::{Serialize};

#[derive(Serialize)]
pub struct Health {
    status: &'static str,
}

pub fn run() -> Health {
    Health { status: "healthy" }
}
