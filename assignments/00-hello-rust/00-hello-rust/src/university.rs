#[derive(PartialEq, Clone, Copy, Debug)]
enum ClassYear {
    Senior,
    Junior,
    Sophomore,
    FirstYear,
}

struct Student {
    name: &'static str,
    class_year: ClassYear,
    gpa: f32,
}

const OLIN_STUDENTS: [Student; 8] = [
    Student {
        name: "Alice",
        class_year: ClassYear::Senior,
        gpa: 3.9,
    },
    Student {
        name: "Foo",
        class_year: ClassYear::Sophomore,
        gpa: 2.3,
    },
    Student {
        name: "Bar",
        class_year: ClassYear::Junior,
        gpa: 3.9,
    },
    Student {
        name: "Ralph",
        class_year: ClassYear::Senior,
        gpa: 3.1,
    },
    Student {
        name: "Ayush",
        class_year: ClassYear::Senior,
        gpa: 0.0,
    },
    Student {
        name: "Anna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Hannah",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lorin",
        class_year: ClassYear::Junior,
        gpa: 3.6,
    },
];

/// Returns the average GPA of the students in the university
///
/// # Returns
///
/// An `f32` representing the average GPA
fn get_average_gpa() -> f32 {
    let mut sum: f32 = 0.0;
    let mut counter: u32 = 0;

    // Calculate the sum of all students' GPAs except for the first year
    for student in OLIN_STUDENTS {
        if student.class_year != ClassYear::FirstYear {
            sum += student.gpa;
            counter += 1;
        }
    }

    return sum / counter as f32;
}

/// Returns the number of students in a class year who have a gpa greater
/// than or equal to the average gpa of all students
///
/// # Arguments
///
/// * `class_year`: A `ClassYear` enum representing the class year
///
/// # Returns
///
/// An `u32` representing the number of students in the class year
fn get_num_excel_students_for_class(class_year: ClassYear) -> u32 {
    let mut counter: u32 = 0;

    // Count the number of students who have a gpa greater than or equal to the average gpa
    for student in OLIN_STUDENTS {
        if student.class_year == class_year && student.gpa >= get_average_gpa() {
            counter += 1;
        }
    }

    return counter;
}

/// Returns the class year with the most excel students
///
/// # Returns
///
/// A `ClassYear` enum representing the class year with the most excel students
fn get_best_class() -> ClassYear {
    let mut most_excel_students: u32 = 0;
    let mut best_class: ClassYear = ClassYear::FirstYear;

    // Find the class year with the most excel students
    for class_year in [
        ClassYear::Senior,
        ClassYear::Junior,
        ClassYear::Sophomore,
        ClassYear::FirstYear,
    ]
    .into_iter()
    {
        let current_excel_students = get_num_excel_students_for_class(class_year);
        if current_excel_students > most_excel_students {
            most_excel_students = current_excel_students;
            best_class = class_year;
        }
    }

    return best_class;
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::university::{
        get_average_gpa, get_best_class, get_num_excel_students_for_class, ClassYear,
    };

    #[test]
    fn test_get_average_gpa() {
        assert!(approx_eq!(f32, get_average_gpa(), 2.8))
    }

    #[test]
    fn test_get_num_excel_students_for_class() {
        assert_eq!(get_num_excel_students_for_class(ClassYear::Sophomore), 0);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Junior), 2);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Senior), 2);
    }

    #[test]
    fn test_get_best_class() {
        assert_eq!(get_best_class(), ClassYear::Senior);
    }
}
