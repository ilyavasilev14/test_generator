use std::{collections::HashMap, ffi::OsStr, fmt::Display, fs, path::PathBuf, sync::mpsc::{Receiver, Sender}, thread};

use egui::{Button, CentralPanel, Context, Label, Modal, RichText, ScrollArea, Vec2};
use mlua::{Function, Lua, LuaOptions, ObjectLike, StdLib};

use crate::{create_exercise::CreateExerciseData, custom_gui::GeneratorGUI, exercise::{add_lua_io_functions, exercises_count_string, AnswerState, ExerciseData, UnloadedExerciseData}, exercise_download::{show_exercise_download_modal, ExerciseDownloadModal}};

pub fn text (text: impl Into<String>, font_size: f32) -> RichText {
    return RichText::new(text).size(font_size)
}

/// returns true if the config should be updated
pub fn display_list(ctx: &Context, selected_now: &mut HashMap<String, usize>, exercise_files: &mut Vec<UnloadedExerciseData>,
    exercises: &mut Vec<ExerciseData>, new_exercise_modal_open: &mut bool, exercise_download_modal: &mut ExerciseDownloadModal,
    create_exercise_data: &mut Option<CreateExerciseData>) -> DisplayListResponse {
    let mut update_config = false;
    CentralPanel::default().show(ctx, |ui| {
        let mut exercises_count = 0;
        let width = ui.available_width();
        let height = ui.available_height();
        let offset = ui.spacing().item_spacing.x;
        ui.vertical(|ui| {
            ui.vertical(|ui| {
                ui.set_min_height(height - 150.0);
                ScrollArea::vertical().max_height(height - 280.0).show(ui, |ui| {
                    for exercise_file in exercise_files.chunks(3) {
                        let offset = ui.spacing().item_spacing.x;
                        let exercise_button_size = Vec2::new(width / 3.0 - offset / 1.5, 50.0);
                        let plus_minus_buttons_size = Vec2::new(30.0, 30.0);
                        let counter_size = Vec2::new(exercise_button_size.x - 60.0, 30.0);
                        ui.horizontal(|ui| {
                            for exercise_file in exercise_file.iter() {
                                ui.vertical(|ui| {
                                    let count = 
                                        match selected_now.get(&exercise_file.name) {
                                            Some(count) => count.clone(),
                                            None => 0,
                                        };
                                    exercises_count += count;
                                    ui.add_sized(exercise_button_size, Button::new(&exercise_file.name));
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 0.0;

                                        if ui.add_sized(plus_minus_buttons_size, Button::new("-")).clicked() {
                                            if count != 0 {
                                                selected_now.insert(exercise_file.name.clone(), count - 1);
                                            }
                                        };

                                        ui.add_sized(counter_size, Label::new(count.to_string()));

                                        if ui.add_sized(plus_minus_buttons_size, Button::new("+")).clicked() {
                                            selected_now.insert(exercise_file.name.clone(), count + 1);
                                        }
                                    });
                                });
                            }
                        });
                        ui.add_space(10.0);
                    }
                })
            });

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            let exercise_text = format!("Составить вариант{}", exercises_count_string(exercises_count));
            if ui.add_sized(Vec2::new(width - offset / 2.0, 50.0), Button::new(text(exercise_text, 22.0))).clicked() {
                let _ = fs::remove_dir(get_test_path());
                for exercise_file in &mut *exercise_files {
                    let count = 
                        match selected_now.get(&exercise_file.name) {
                            Some(count) => count.clone(),
                            None => 0,
                        };

                    for idx in 0..count {
                        let mut lua_vm = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).expect("Failed to create a Lua VM!");
                        let chunk = lua_vm.load(&exercise_file.lua_code);
                        if let Err(err) = chunk.exec() {
                            println!("Lua code execution failed: {}", err);
                            return 
                        }
                        let right_answer: Result<Option<String>, mlua::Error> = lua_vm.globals().call_function("get_exercise_right_answer", ());
                        let exercise_text: Result<Option<String>, mlua::Error> = lua_vm.globals().call_function("get_exercise_text", ());
                        let right_answer = match right_answer {
                            Ok(answer) => answer,
                            Err(err) => {
                                println!("Failed to run the function get_exercise_right_answer!: {}", err);
                                return
                            },
                        };
                        let exercise_text = match exercise_text {
                            Ok(text) => text,
                            Err(err) => {
                                println!("Failed to run the function get_exercise_text!: {}", err);
                                return
                            },
                        };

                        let mut custom_gui = GeneratorGUI::default();
                        let _ = lua_vm.scope(|scope| {
                            if let Ok(gui_function) = lua_vm.globals().get::<Function>("get_custom_gui") {
                                let gui_userdata = scope.create_userdata_ref_mut(&mut custom_gui)
                                    .expect("Failed to create a GeneratorGUI userdata! - exercise_list");
                                let _ = gui_function.call::<()>(gui_userdata);
                            }
                            Ok(())
                        });
                        let _ = lua_vm.set_app_data(idx);
                        add_lua_io_functions(&mut lua_vm);

                        exercises.push(ExerciseData {
                            lua_vm,
                            right_answer,
                            exercise_text,
                            current_input: String::new(),
                            answer_state: AnswerState::NotSolved,
                            name: exercise_file.name.clone(),
                            custom_gui,
                        });
                    }
                }
            }
            if ui.add_sized(Vec2::new(width - offset / 2.0, 50.0), Button::new(text("Добавить генератор", 22.0))).clicked() {
                *new_exercise_modal_open = true;
                return
            }
        });
    });
    if exercise_download_modal.is_open {
        let response = Modal::new("Exercise download modal".into()).show(ctx, |ui| {
            if show_exercise_download_modal(ui, exercise_download_modal, exercise_files) == true {
                *new_exercise_modal_open = false;
                update_config = true;
            }
        }).backdrop_response;
        if response.clicked() {
            exercise_download_modal.is_open = false;
        }
        if update_config {
            return DisplayListResponse::UpdateConfig
        }
    }
    if *new_exercise_modal_open {
        let mut current_response: Option<DisplayListResponse> = None;
        let response = Modal::new("Open file modal".into()).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_width(400.0);
                ui.label(text("Добавить задание", 34.0));
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(10.0);
                let size = Vec2::new(ui.available_width(), 35.0);
                let from_file_button = Button::new(text("Загрузить из файла", 24.0));
                let download_button = Button::new(text("Загрузить из репозитория", 24.0));
                let new_button = Button::new(text("Создать генератор", 24.0));
                let close_button = Button::new(text("Отмена", 24.0));
                if ui.add_sized(size, from_file_button).clicked() {
                    //pick_and_load_exercise(exercise_files);
                    *new_exercise_modal_open = false;
                    update_config = true;
                    current_response = Some(DisplayListResponse::LoadExercise);
                }
                if ui.add_sized(size, download_button).clicked() {
                    exercise_download_modal.is_open = true;
                }
                if ui.add_sized(size, new_button).clicked() {
                    current_response = Some(DisplayListResponse::CreateExercise)
                }
                if ui.add_sized(size, close_button).clicked() {
                    *new_exercise_modal_open = false;
                }
            })
        }).backdrop_response;
        if let Some(response) = current_response {
            return response
        }
        if response.clicked() {
            *new_exercise_modal_open = false;
        }
    }
    if update_config {
        return DisplayListResponse::UpdateConfig
    } else {
        return DisplayListResponse::None
    }
}

pub fn load_picked_exercise(exercise_files: &mut Vec<UnloadedExerciseData>, exercise_path: Option<PathBuf>) {
    if let Some(exercise_path) = exercise_path {
        let file_extension = exercise_path.extension();
        // continue only if the extension of the file is .lua
        match file_extension {
            Some(extension) => {
                if extension != "lua" {
                    println!("selected file isn't a lua file!");
                    return
                }
            },
            None => {
                println!("selected file isn't a lua file!");
                return
            },
        }
        let lua_code = fs::read_to_string(&exercise_path);
        let lua_vm = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).expect("Failed to create a Lua VM!");
        match lua_code {
            Ok(lua_code) => {
                let chunk = lua_vm.load(&lua_code);
                if let Err(err) = chunk.exec() {
                    println!("Lua code execution failed: {}", err);
                    return 
                }
                let name: Result<String, mlua::Error> = lua_vm.globals().call_function("get_exercise_title", ());
                let name = match name {
                    Ok(name) => name,
                    Err(err) => {
                        println!("Can't get the name of the exercise! Err: {}", err);
                        let name = exercise_path.file_name().unwrap_or_else(|| return OsStr::new("Не удаётся получить название упражнения!"));
                        name.to_str().unwrap_or_else(|| "Не удаётся получить название упражнения!").to_string()
                    },
                };

                for data in &mut *exercise_files {
                    if data.name == name {
                        data.lua_code = lua_code;
                        return
                    }
                }
                exercise_files.push(UnloadedExerciseData { lua_code, name });
            },
            Err(err) => {
                println!("failed to read the lua file to a string: {}", err);
                return
            },
        }
    }
}

fn get_test_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to get home dir");
    home_dir.join("Test")
}

pub enum DisplayListResponse {
    CreateExercise,
    LoadExercise,
    UpdateConfig,
    None
}

pub enum RfdDataType {
    CreateExercise,
    LoadExercise,
}
