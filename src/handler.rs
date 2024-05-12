use std::{
    sync::atomic::AtomicU16,
    sync::atomic::Ordering::Relaxed,
};

use crate::model::Greeting;

pub struct WebHandler {
    number_of_visits: AtomicU16,
}

impl WebHandler  {
   pub fn greet(&self, visitor: String) -> Greeting {
    let visits = self.number_of_visits.fetch_add(1, Relaxed);
    Greeting::new("Hello", visitor, visits)
   }

   pub fn say_goodbye(&self) -> String {
    "Goodbye".to_string()
   }
}

impl Default for WebHandler {
    fn default() -> Self {
        WebHandler {
            number_of_visits: AtomicU16::new(0),
        }
    }
}