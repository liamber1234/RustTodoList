use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoListQuest {
    description: String,
    finished: bool,
}

impl fmt::Display for TodoListQuest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl TodoListQuest {
    pub fn new(description: String) -> Self {
        TodoListQuest {
            description,
            finished: false,
        }
    }

    pub fn finish_quest(&mut self) {
        self.finished = true;
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}