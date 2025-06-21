use std::fs;
use std::io;
use genpdf::{elements, Alignment};

fn calculate_average(total_marks: f32, subjects: u32) -> f32 {
    total_marks / subjects as f32
}

fn assign_grade(average: f32) -> char {
    if average >= 90.0 {
        'A'
    } else if average >= 75.0 {
        'B'
    } else if average >= 60.0 {
        'C'
    } else {
        'D'
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks_str = String::new();
    let mut subjects_str = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks_str).expect("Failed to read total marks");

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subjects_str).expect("Failed to read number of subjects");

    let total_marks: f32 = total_marks_str.trim().parse().expect("Enter valid number");
    let subjects: u32 = subjects_str.trim().parse().expect("Enter valid number");

    let average = calculate_average(total_marks, subjects);
    let grade = assign_grade(average);

    println!("\n Report Card");
    println!("Name    : {}", name.trim());
    println!("Total   : {}", total_marks);
    println!("Subjects: {}", subjects);
    println!("Average : {:.2}", average);
    println!("Grade   : {}", grade);

    generate_pdf(&name.trim(), total_marks, subjects, average, grade);
}

fn generate_pdf(name: &str, total: f32, subjects: u32, average: f32, grade: char) {
    // Loads font
    let font_path = "LiberationSerif-Regular.ttf";
    if !fs::metadata(font_path).is_ok() {
        println!(" Font file not found: {}", font_path);
        println!("Please place 'LiberationSerif-Regular.ttf' in project root");
        return;
    }

    let font_family = genpdf::fonts::from_files(".", "LiberationSerif", None)
        .expect("Failed to load font");

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Report Card");
    doc.set_minimal_conformance();

    let mut layout = elements::LinearLayout::vertical();
    layout.push(elements::Paragraph::new("Student Report Card").aligned(Alignment::Center));
    layout.push(elements::Break::new(1));

    layout.push(elements::Paragraph::new(format!("Name          : {}", name)));
    layout.push(elements::Paragraph::new(format!("Total Marks   : {}", total)));
    layout.push(elements::Paragraph::new(format!("Subjects      : {}", subjects)));
    layout.push(elements::Paragraph::new(format!("Average       : {:.2}", average)));
    layout.push(elements::Paragraph::new(format!("Grade         : {}", grade)));

    doc.push(layout);
    doc.render_to_file("report_card.pdf").expect("Failed to generate PDF");

    println!(" PDF generated: report_card.pdf");
}
