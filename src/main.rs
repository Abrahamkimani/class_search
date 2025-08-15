use std::env;
use std::process;

use search::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No command provided. Use: register-class, register-student, mark-attendance, view-attendance, analytics");
        process::exit(1);
    }

    match args[1].as_str() {
        "register-class" => {
            if args.len() < 5 {
                eprintln!("Usage: register-class <uuid> <name> <date>");
                process::exit(1);
            }
            search::register_class(&args[2], &args[3], &args[4]);
        }
        "register-student" => {
            if args.len() < 4 {
                eprintln!("Usage: register-student <name> <email>");
                process::exit(1);
            }
            search::register_student(&args[2], &args[3]);
        }
        "mark-attendance" => {
            if args.len() < 3 {
                eprintln!("Usage: mark-attendance <class_uuid>");
                process::exit(1);
            }
            search::mark_attendance(&args[2]);
        }
        "view-attendance" => {
            if args.len() < 3 {
                eprintln!("Usage: view-attendance <class_uuid>");
                process::exit(1);
            }
            search::view_attendance(&args[2]);
        }
        "analytics" => {
            search::attendance_analytics();
        }
        _ => {
            eprintln!("Unknown command.");
            process::exit(1);
        }
    }
}
