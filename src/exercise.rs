use std::{fs::{self, File}, io::Write, path::PathBuf, process::Command, thread};

use egui::{Button, RichText, TextEdit, Ui, Vec2};
use mlua::{Function, Lua, ObjectLike};
use serde::{Deserialize, Serialize};
use crate::{custom_gui::GeneratorGUI, exercise_list::text};

pub static mut IS_FS_ALLOWED: bool = false;
pub static mut FS_ACCESS_WARN: Vec<String> = Vec::new();

#[derive(Debug)]
pub enum AnswerState {
    NotSolved,
    Correct,
    Err(String),
    /// Optional correct answer
    Wrong(Option<String>),
}

#[derive(Debug)]
pub struct ExerciseData {
    pub lua_vm: Lua,
    pub right_answer: Option<String>,
    pub exercise_text: Option<String>,
    pub current_input: String,
    pub answer_state: AnswerState,
    pub name: String,
    pub custom_gui: GeneratorGUI,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnloadedExerciseData {
    pub lua_code: String,
    pub name: String,
}

pub fn display_exercise(exercise: &mut ExerciseData, ui: &mut Ui, idx: usize) {
    let exercise_number_text = format!("Задание №{} ({})", idx + 1, exercise.name);
    ui.label(RichText::from(exercise_number_text).size(36.0));
    if !exercise.custom_gui.containers.is_empty() {
        exercise.custom_gui.draw(ui);
    } else {
        let fallback_text = &("Не удалось получить текст задания!".to_string());
        let text = exercise.exercise_text.as_ref().unwrap_or(fallback_text);
        ui.label(RichText::from(text).size(24.0));
        ui.add_space(7.0);
    }

    match &exercise.answer_state {
        AnswerState::NotSolved => {
            ui.horizontal(|ui| {
                ui.add_sized(Vec2::new(300.0, 30.0), TextEdit::singleline(&mut exercise.current_input));
                //ui.add_space(5.0);
                if ui.add_sized(Vec2::new(100.0, 30.0), Button::new(RichText::new("Ответить").size(22.0))).clicked() {
                    check_answer(exercise);
                }
            });
        },
        AnswerState::Correct => {
            ui.label(RichText::new("Задание решено верно!").size(16.0));
        },
        AnswerState::Wrong(right_answer) => {
            match right_answer {
                Some(answer) =>
                    ui.label(RichText::new(
                            &format!("Задание решено неверно! Правильный ответ: {}", answer))
                        .size(16.0)
                    ),
                None => 
                    ui.label(RichText::new("Задание решено неверно!").size(16.0)),
            };
        },
        AnswerState::Err(err) => {
            ui.label(text(format!("Ошибка проверки задания: {}", err), 16.0));
        },
    }
    ui.add_space(15.0);
    ui.separator();
    ui.add_space(10.0);
}

fn check_answer(exercise: &mut ExerciseData) {
    match &exercise.right_answer {
        Some(answer) => {
            if &exercise.current_input == answer {
                exercise.answer_state = AnswerState::Correct
            } else {
                exercise.answer_state = AnswerState::Wrong(Some(answer.clone()))
            }
        },
        None => {
            exercise.answer_state = match exercise.lua_vm.globals().get::<Function>("check_exercise") {
                Ok(function) => {
                    match function.call::<bool>(()) {
                        Ok(is_right) =>
                            match is_right {
                                true => AnswerState::Correct,
                                false => AnswerState::Wrong(None),
                            },
                        Err(err) =>
                            AnswerState::Err(err.to_string()),
                    }
                },
                Err(err) => 
                    AnswerState::Err(err.to_string()),
            }
        },
    }
}

pub fn exercises_count_string(exercises_count: usize) -> String {
    let word = match exercises_count % 10 {
        1 => "задание",
        2 => "задания",
        3 => "задания",
        4 => "задания",
        _ => "заданий",
    };

    return format!(" ({} {})", exercises_count, word)
}

pub fn get_ex_path(lua: &Lua, path: String) -> PathBuf {
    let id: mlua::AppDataRef<'_, usize> = lua.app_data_ref().unwrap();
    let home_dir = dirs::home_dir().expect("Failed to get home dir");

    home_dir.join("Test").join(id.to_string()).join(path)
}

pub fn add_lua_io_functions(lua: &mut Lua) {
    let new_dir = lua.create_function_mut(|lua, (path, open): (String, bool)| {
        let path = get_ex_path(lua, path);
        let _ = fs::create_dir_all(&path);
        if open {
            let _ = thread::spawn(|| open::that(path));
        }
        Ok(())
    }).expect("Failed to create a Lua function (new_dir)!");

    let new_file = lua.create_function_mut(|lua, (path, contents, open): (String, String, bool)| {
        let path = get_ex_path(lua, path);
        match File::create_new(&path) {
            Ok(mut file) => {
                 match file.write_all(contents.as_bytes()) {
                    Ok(_) => {
                        if open {
                            let _ = thread::spawn(move || open::that(path.with_file_name("")));
                        }
                    },
                    Err(err) => 
                        println!("Failed to write to a file (new_file)! Err: {}", err),
                }
            },
            Err(err) => 
                println!("Failed to create a file (new_file)! Err: {}", err),
        };
        Ok(())
    }).expect("Failed to create a Lua function (new_file)!");

    let get_full_path = lua.create_function_mut(|lua, path: String| {
        Ok(get_ex_path(lua, path))
    }).expect("Failed to create a Lua function (new_file)!");

    let read_file = lua.create_function_mut(|lua, path: String| {
        Ok(fs::read_to_string(get_ex_path(lua, path)).ok())
    }).expect("Failed to create a Lua function (read_file)!");

    let run_command = lua.create_function_mut(|_, (command, args): (String, String)| {
        let mut command = Command::new(command);
        let command = command.args(args.split(" "));
        let output = command.output();
        if let Ok(output) = output {
            let output = String::from_utf8_lossy(&output.stdout).to_string();
            return Ok(Some(output))
        }
        Ok(None)
    }).expect("Failed to create a Lua function (new_file)!");

    lua.globals().set("new_dir", new_dir)
        .expect("Failed to set a Lua function global (new_dir)");
    lua.globals().set("new_file", new_file)
        .expect("Failed to set a Lua function global (new_file)");
    lua.globals().set("run_command", run_command)
        .expect("Failed to set a Lua function global (run_command)");
    lua.globals().set("get_full_path", get_full_path)
        .expect("Failed to set a Lua function global (get_full_path)");
    lua.globals().set("read_file", read_file)
        .expect("Failed to set a Lua function global (read_file)");
}
