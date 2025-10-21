use std::sync::{LazyLock, Mutex};
use rand::seq::IteratorRandom;

pub struct RobotNamingAPI {
    available_names: Vec<String>,
    used: Vec<String>,
}

impl RobotNamingAPI {
    pub fn new() -> Self {
        let mut names = Vec::new();
        for c1 in 'A'..='Z' {
            for c2 in 'A'..='Z' {
                for n in 0..=999 {
                    names.push(format!("{c1}{c2}{n:03}"));
                }
            }
        }
        RobotNamingAPI {
            available_names: names,
            used: Vec::new(),
        }
    }

    pub fn get_unique_name(&mut self) -> String {
        let available_count = self.available_names.len();
        if available_count > 0 {
            let random_index = rand::random_range(0..available_count);
            let name = self.available_names.remove(random_index);
            self.used.push(name.to_string());
            return name.clone();
        }
        panic!("No unique names available");
    }

    pub fn release_name(&mut self, name: &str) {
        if let Some(pos) = self.used.iter().position(|x| x == name) {
            self.used.remove(pos);
            self.available_names.push(name.to_string());
        }
    }
}

static ROBOT_NAMING_API: LazyLock<Mutex<RobotNamingAPI>> = LazyLock::new(|| Mutex::new(RobotNamingAPI::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: ROBOT_NAMING_API.lock().unwrap().get_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut api = ROBOT_NAMING_API.lock().unwrap();
        api.release_name(&self.name);
        self.name = api.get_unique_name();
    }
}
