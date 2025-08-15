use std::env;
// --snip--

use std::error::Error;
use std::fs;

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref CLASSES: Mutex<HashMap<String, (String, String, HashSet<String>)>> = Mutex::new(HashMap::new());
    static ref STUDENTS: Mutex<HashMap<String, (String, String)>> = Mutex::new(HashMap::new());
}

pub fn register_class(uuid: &str, name: &str, date: &str) {
    let mut classes = CLASSES.lock().unwrap();
    classes.insert(uuid.to_string(), (name.to_string(), date.to_string(), HashSet::new()));
    println!("Class registered: {} - {} on {}", uuid, name, date);
}

pub fn register_student(name: &str, email: &str) {
    let mut students = STUDENTS.lock().unwrap();
    students.insert(email.to_string(), (name.to_string(), email.to_string()));
    println!("Student registered: {} <{}>", name, email);
}

pub fn mark_attendance(class_uuid: &str) {
    let students = STUDENTS.lock().unwrap();
    let mut classes = CLASSES.lock().unwrap();
    // For demo, use email as identifier
    let email = "demo@student.com"; // Replace with actual auth logic
    if let Some(class) = classes.get_mut(class_uuid) {
        class.2.insert(email.to_string());
        println!("Attendance marked for {} in class {}", email, class_uuid);
    } else {
        println!("Class not found");
    }
}

pub fn view_attendance(class_uuid: &str) {
    let classes = CLASSES.lock().unwrap();
    if let Some(class) = classes.get(class_uuid) {
        println!("Attendance for class {}:", class_uuid);
        for attendee in &class.2 {
            println!("- {}", attendee);
        }
    } else {
        println!("Class not found");
    }
}

pub fn attendance_analytics() {
    let classes = CLASSES.lock().unwrap();
    let total: usize = classes.values().map(|c| c.2.len()).sum();
    println!("Total attendance records: {}", total);
    for (uuid, class) in classes.iter() {
        println!("Class {}: {} attendees", uuid, class.2.len());
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

safe, fast, productive.
Pick three.
Duct tape.";
safe, fast, productive.
Pick three.
Trust me.";
// ...existing code...