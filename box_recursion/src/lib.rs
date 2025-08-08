#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        let s_lower = s.to_lowercase();
        if s_lower == "ceo" {
            Role::CEO
        } else if s_lower == "manager" {
            Role::Manager
        } else {
            Role::Worker
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment {
            grade: None,
        }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let rr = Worker {
            role: role.to_string(),
            name: name.to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(rr));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(rr) = self.grade.take() {
            self.grade = rr.next;
            Some(rr.name)
        } else {
            None
        }
    }
    pub fn last_worker(&self) -> Option<(String, Role)> {
        if let Some(w) = &self.grade {
            Some((w.name.clone(), Role::from(w.role.as_str())))
        } else {
            None
        }
    }
}
