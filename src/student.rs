use std::fmt;

#[derive(Debug)]
pub struct Student {
    pub id: usize,
    pub name: String,
    pub age: u32,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}, Tên: {}, Tuổi: {}", self.id, self.name, self.age)
    }
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct StudentList {
    students: Vec<Student>,
}

impl StudentList {
    pub fn new() -> Self {
        StudentList { students: vec![] }
    }

    pub fn add_student(&mut self, student: Student) {
        if self.students.iter().any(|s| s.id == student.id) {
            println!("Học sinh có ID {} đã tồn tại. Không thể thêm.", student.id);
        } else {
            self.students.push(student);
            println!("Học sinh đã được thêm vào danh sách.");
        }
    }

    pub fn display_students(&self) {
        if self.students.is_empty() {
            println!("Danh sách học sinh trống.");
        } else {
            for student in &self.students {
                println!("{}", student);
            }
        }
    }

    pub fn update_student(&mut self, updated_student: Student) {
        if let Some(index) = self.students.iter().position(|s| *s == updated_student) {
            self.students[index] = updated_student;
            println!("Thông tin học sinh đã được cập nhật.");
        } else {
            println!("Không tìm thấy học sinh cần cập nhật.");
        }
    }

    pub fn remove_student(&mut self, id: usize) {
        if let Some(index) = self.students.iter().position(|s| s.id == id) {
            self.students.remove(index);
            println!("Học sinh đã được xóa khỏi danh sách.");
        } else {
            println!("Không tìm thấy học sinh cần xóa.");
        }
    }
}

pub fn create_student() -> Student {
    let id: usize = get_input("Nhập ID học sinh: ");
    let name: String = get_input("Nhập tên học sinh: ");
    let age: u32 = get_input("Nhập tuổi học sinh: ");

    Student { id, name, age }
}

pub fn update_student(id: usize) -> Student {
    let name: String = get_input("Nhập tên học sinh mới: ");
    let age: u32 = get_input("Nhập tuổi học sinh mới: ");

    Student { id, name, age }
}

fn get_input<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        use std::io;
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Nhập không hợp lệ. Vui lòng thử lại."),
        }
    }
}
