// Libraries
use csv::{Position, ReaderBuilder};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{self, Write};

// Create student struct
#[derive(Deserialize, Debug)]
struct Student {
    course_code: String,
    student_number: String,
    name: String,
    program: String,
    plan: String,
    wam: f64,
    session: String,
    birthdate: String,
    sex: String,
}

fn main() {
    // open the file enrollments.csv
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path("enrolments.psv")
        .expect("Couldn't open file");

    // initialise hashmap
    let mut students: HashMap<String, Student> = HashMap::new();

    // add student to hashmap
    reader.deserialize().for_each(|result| {
        let student: Student = result.unwrap();
        students.insert(student.student_number.clone(), student);
    });

    // print number of students
    println!("Number of students: {}", students.len());

    // find most common course:
    let mut course_counts: HashMap<String, u32> = HashMap::new();
    reader.seek(Position::new()).unwrap();
    reader.deserialize().for_each(|student| {
        let student: Student = student.unwrap();
        course_counts
            .entry(student.course_code)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    // get course with highest course count
    let (course, count) = course_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();

    // print most common course
    println!("Most common course: {} with {} students", course, count);

    // find least common course:
    let (course, count) = course_counts
        .iter()
        .min_by_key(|(_, count)| *count)
        .unwrap();

    // print least common course
    println!("Least common course: {} with {} students", course, count);

    // find average wam
    // let total_wam = students.values().map(|student| student.wam).sum::<f64>();
    let average_wam = students.values().map(|student| student.wam).sum::<f64>() / students.len() as f64;

    // print average wam
    println!("Average WAM: {:.02}", average_wam);
}
