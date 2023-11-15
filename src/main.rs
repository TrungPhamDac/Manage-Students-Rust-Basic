mod student;

use std::io;

fn main() {
    let mut student_list = student::StudentList::new();

    loop {
        println!("1. Thêm học sinh");
        println!("2. Xem danh sách học sinh");
        println!("3. Sửa thông tin học sinh");
        println!("4. Xóa học sinh");
        println!("5. Thoát");

        let choice: u32 = get_input("Nhập lựa chọn: ");

        match choice {
            1 => {
                let student = student::create_student();
                student_list.add_student(student);
            }
            2 => student_list.display_students(),
            3 => {
                let id: usize = get_input("Nhập ID học sinh cần sửa: ");
                let updated_student = student::update_student(id);
                student_list.update_student(updated_student);
            }
            4 => {
                let id: usize = get_input("Nhập ID học sinh cần xóa: ");
                student_list.remove_student(id);
            }
            5 => break,
            _ => println!("Lựa chọn không hợp lệ!"),
        }
        println!("=======================");
    }
}

fn get_input<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Nhập không hợp lệ. Vui lòng thử lại."),
        }
    }
}
