use std::collections::HashMap;

struct School {
    students: HashMap<String, u32>,
}

fn main() {
    let mut school = School::new();

    school.add(10, "Alice");
    school.add(4, "Bob");
    school.add(4, "Steve");

    println!("{:?}", school.grade(4));
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = Vec::new();

        for (key, val) in self.students.iter() {
            grades.push(*val);
        }
        grades.sort();
        grades.dedup();

        return grades;
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();

        for (key, val) in self.students.iter() {
            if *val == grade {
                names.push(key.to_string());
            }
        }
        names.sort();

        return names;
    }
}
