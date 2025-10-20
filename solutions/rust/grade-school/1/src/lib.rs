use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct School {
    student_grades: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            student_grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.student_grades.entry(student.to_string()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.student_grades.values().unique().sorted().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = self
            .student_grades
            .iter()
            .filter_map(|(student, &g)| if g == grade { Some(student.clone()) } else { None })
            .collect();
        students.sort();
        students
    }
}
