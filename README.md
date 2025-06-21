# student-report-generator

![image](https://github.com/user-attachments/assets/b4866e83-afd1-40c5-b2fb-afe2a9200ab5)


### 📄 `README.md` for Your Project

````markdown
# 📝 Student Report Card Generator (Rust)

A Rust-based console application that generates a clean report card as a PDF using user input such as student name, total marks, and number of subjects.

---

## 🎯 Features

- Collects student information from console input
- Calculates average marks using a custom function
- Assigns grades based on average:
  - **A:** 90+
  - **B:** 75–89
  - **C:** 60–74
  - **D:** Below 60
- Displays a summary in the terminal
- Generates a well-formatted PDF report using `genpdf`

---

## 🛠️ Technologies Used

- **Rust** - Core language
- **genpdf** - PDF generation library
- **Liberation Serif** font - For clean typography in the PDF

---

## 🚀 Getting Started

### 📦 Prerequisites

- Rust installed: [Install Rust](https://www.rust-lang.org/tools/install)
- Font file: `LiberationSerif-Regular.ttf` in project root

### 🔧 Setup & Run

1. Clone the repository:
   ```bash
   git clone https://github.com/Harish-SS56/student-report-generator.git
   cd student-report-generator
````

2. Place `LiberationSerif-Regular.ttf` in the root folder (same directory as `Cargo.toml`)

3. Run the app:

   ```bash
   cargo run
   ```

4. After input, a file named `report_card.pdf` will be generated.

---

## 📄 Sample Output

```
Name    : Harish s.s.
Total   : 486
Subjects: 5
Average : 97.20
Grade   : A
```

➡️ PDF: `report_card.pdf`
✅ Clean, printable report for the student

---

## 📁 Project Structure

```
student-report-generator/
├── Cargo.toml
├── LiberationSerif-Regular.ttf
├── report_card.pdf
└── src/
    └── main.rs
```

---

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss.

---

## ✨ Author

**Harish S.S.**
Built as part of Mini Task-3 (Rust-based Report Card App) under a tech internship initiative.

````

---

### ✅ To Add It to Your Project

Save this content in a file named `README.md` at the **root of your project** (same level as `Cargo.toml`), then run:

```bash
git add README.md
git commit -m "Added project README"
git push
````


