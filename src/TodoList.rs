use std::fmt;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

use crate::TodoListItem::TodoListQuest;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    allQuests : Vec<TodoListQuest>,
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for quest in self.allQuests.iter() {
            write!(f, "{}", quest).expect("TODO: panic message");
        }
        Ok(())
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { allQuests: vec![] }
    }

    pub fn add_quest(&mut self, quest: TodoListQuest) {
        self.allQuests.push(quest);
    }

    pub fn remove_quest(&mut self, index : usize) {
        self.allQuests.remove(index);
    }
}