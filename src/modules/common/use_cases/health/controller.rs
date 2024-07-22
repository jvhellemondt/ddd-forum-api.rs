use crate::modules::common::use_cases::health::model;
use crate::modules::common::use_cases::health::model::Health;

pub fn handle() -> Health {
    model::run()
}
