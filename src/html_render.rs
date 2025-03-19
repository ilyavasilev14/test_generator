use std::{fs::{self, File}, io::Write, thread};

use crate::{custom_gui::{Container, ContainerType, WidgetType}, exercise::ExerciseData};

pub fn render(exercises: &Vec<ExerciseData>) {
    let mut result: String = "<head><meta charset=\"utf-8\"></head><body>"
        .into();
    for (idx, exercise) in exercises.iter().enumerate() {
        result += &render_exercise(idx, exercise);
    }

    let file_path = dirs::home_dir()
        .expect("Failed to get the document (home) directory!");
    let _ = fs::create_dir_all(&file_path); // just in case
    let file_path = file_path.join("Вариант.html");


    let _ = fs::remove_file(&file_path);
    match File::create(&file_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(result.as_bytes()) {
                println!("Failed to write to the html file! Err: {}", err);
                return
            }

            thread::spawn(|| {
                if let Err(err) = open::that(file_path) {
                    println!("Failed to open the html file! Err: {}", err);
                }
            });
        },
        Err(err) =>
            println!("Failed to create an html file! Err: {}", err),
    }
}

fn render_exercise(idx: usize, exercise: &ExerciseData) -> String {
    let mut result = String::new();
    result += &format!("<h3>Задание №{} ({})</h3>", idx + 1, exercise.name);
    if !exercise.custom_gui.containers.is_empty() {
        for container in &exercise.custom_gui.containers {
            result += &container.to_html();
        }
    } else {
        let fallback_text = &("Не удалось получить текст задания!".to_string());
        let text = exercise.exercise_text.as_ref().unwrap_or(fallback_text);
        result += &format!("<p style=\"white-space: pre\">{}</p><br>", text);
    }

    result
}

impl Container {
    fn html_container_widgets(&self, line_break: &str) -> String {
        let mut result = String::new();
        for (widget, _) in &self.widgets {
            match widget {
                WidgetType::Button(text, font_size, size) => {
                    match size {
                        Some(size) => {
                            result += &format!("<button style=\"{}\">{}</button>{}", 
                                format!("font-size: {}px; width: {}px; height: {}px;", 
                                    font_size, size[0], size[1]), text, line_break)
                        }
                        None => {
                            result += &format!("<button style=\"font-size: {}px\">{}</button>{}", 
                                    font_size, text, line_break)
                        },
                    };
                },
                WidgetType::Label(text, font_size) => {
                    result += &format!("<div style=\"font-size: {}px\">{}</div>{}", 
                        font_size, text, line_break)
                },
                WidgetType::Image(path) => {
                    result += &format!("<img src=\"{}\">{}", path, line_break)
                },
                WidgetType::Separator => { result += "<br style=\"height: 0px;\">" },
                WidgetType::Space(space) => {
                    result += &format!("<div style=\"height: {}px;\">", space) 
                },
            }
        }
        result
    }

    fn html_container_children(&self) -> String {
        let mut result = String::new();
        for child in &self.children {
            result += &child.to_html();
        }
        result
    }

    fn to_html(&self) -> String {
        let mut result = String::new();
        match self.container_type {
            ContainerType::Horizontal => {
                result += "<div>";
                result += &self.html_container_widgets("");
                result += &self.html_container_children();
                result += "</div>";
            },
            ContainerType::Vertical => {
                result += "<div>";
                result += &self.html_container_widgets("<br>");
                result += &self.html_container_children();
                result += "</div>";
            },
        };
        return result
    }
}
