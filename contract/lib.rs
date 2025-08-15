use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Student {
    pub name: String,
    pub email: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Class {
    pub uuid: String,
    pub name: String,
    pub date: String,
    pub attendees: UnorderedSet<AccountId>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AttendanceContract {
    pub classes: UnorderedMap<String, Class>,
    pub students: UnorderedMap<AccountId, Student>,
}

impl Default for AttendanceContract {
    fn default() -> Self {
        Self {
            classes: UnorderedMap::new(b"c"),
            students: UnorderedMap::new(b"s"),
        }
    }
}

#[near_bindgen]
impl AttendanceContract {
    pub fn register_class(&mut self, uuid: String, name: String, date: String) {
        let class = Class {
            uuid: uuid.clone(),
            name,
            date,
            attendees: UnorderedSet::new(uuid.as_bytes()),
        };
        self.classes.insert(&uuid, &class);
    }

    pub fn register_student(&mut self, name: String, email: String) {
        let account_id = env::predecessor_account_id();
        let student = Student { name, email };
        self.students.insert(&account_id, &student);
    }

    pub fn mark_attendance(&mut self, class_uuid: String) {
        let account_id = env::predecessor_account_id();
        if let Some(mut class) = self.classes.get(&class_uuid) {
            class.attendees.insert(&account_id);
            self.classes.insert(&class_uuid, &class);
        }
    }

    pub fn get_attendance(&self, class_uuid: String) -> Vec<AccountId> {
        if let Some(class) = self.classes.get(&class_uuid) {
            class.attendees.to_vec()
        } else {
            vec![]
        }
    }

    pub fn attendance_stats(&self) -> usize {
        self.classes.values().map(|c| c.attendees.len()).sum()
    }
}
