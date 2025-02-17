use egui::{Button, ScrollArea, TextEdit, Ui, Vec2};
use serde::Deserialize;
use crate::{exercise::UnloadedExerciseData, exercise_list::text};

pub struct ExerciseDownloadModal {
    pub is_open: bool,
    pub current_repo: String,
    pub current_end_idx: usize,
    pub trusted_only: bool,
    pub exercise_list: Vec<(usize, String)>,
    pub search_query: String,
    pub error: String,
    pub current_exercise: Option<CurrentExerciseResponse>
}

impl Default for ExerciseDownloadModal {
    fn default() -> Self {
        Self {
            is_open: false,
            current_repo: String::from("skillissuedev.net"),
            current_end_idx: 0,
            exercise_list: Vec::new(),
            trusted_only: false,
            search_query: String::new(),
            error: String::new(),
            current_exercise: None,
        }
    }
}

fn get_current_exercise(modal_data: &mut ExerciseDownloadModal, idx: usize) {
    let repo = match modal_data.current_repo.strip_suffix('/') {
        Some(repo) => repo,
        None => &modal_data.current_repo,
    }.to_string();
    let url = "http://".to_string() + &repo + ":4001/getExercise?idx=" + &idx.to_string();

    let response = reqwest::blocking::get(&url);
    match response {
        Ok(response) => {
            match response.text() {
                Ok(response) => {
                    match serde_json::from_str::<CurrentExerciseResponse>(&response) {
                        Ok(response) => {
                            modal_data.current_exercise = Some(CurrentExerciseResponse {
                                name: response.name,
                                trusted: response.trusted,
                                lua_code: response.lua_code,
                            });
                        },
                        Err(err) =>
                            modal_data.error = format!("{}", err),
                    }
                },
                Err(err) =>
                    modal_data.error = format!("{}", err),
            }
        },
        Err(err) => {
            modal_data.error = format!("{}", err);
        }
    }
}
fn update_modal_data(modal_data: &mut ExerciseDownloadModal) {
    if modal_data.search_query.is_empty() {
        let repo = match modal_data.current_repo.strip_suffix('/') {
            Some(repo) => repo,
            None => &modal_data.current_repo,
        }.to_string();
        let url = match modal_data.trusted_only {
            true =>
                "http://".to_string() + &repo + ":4001/trustedList?currentIdx=0",
            false =>
                "http://".to_string() + &repo + ":4001/list?currentIdx=0",
        };

        modal_data.exercise_list.clear();
        let response = reqwest::blocking::get(&url);
        match response {
            Ok(response) => {
                match response.text() {
                    Ok(response) => {
                        add_exercises_to_list(modal_data, response);
                    },
                    Err(err) =>
                        modal_data.error = format!("{}", err),
                }
            },
            Err(err) =>
                modal_data.error = format!("{}", err),
        }
    } else {
        let repo = match modal_data.current_repo.strip_suffix('/') {
            Some(repo) => repo,
            None => &modal_data.current_repo,
        }.to_string();
        let url = match modal_data.trusted_only {
            true =>
                ("http://".to_string() + &repo + ":4001" + "/trustedSearch?query=") + &modal_data.search_query.replace(" ", "+"),
            false =>
                ("http://".to_string() + &repo + ":4001" + "/search?query=") + &modal_data.search_query.replace(" ", "+"),
        };

        modal_data.exercise_list.clear();
        let response = reqwest::blocking::get(&url);
        match response {
            Ok(response) => {
                match response.text() {
                    Ok(response) => {
                        add_exercises_to_list(modal_data, response);
                    },
                    Err(err) =>
                        modal_data.error = format!("{}", err),
                }
            },
            Err(err) =>
                modal_data.error = format!("{}", err),
        }
    }
}
fn add_exercises_to_list(modal_data: &mut ExerciseDownloadModal, response_string: String) {
    match serde_json::from_str::<ExerciseListResponse>(&response_string) {
        Ok(response) => {
            match response.end_idx {
                Some(idx) => 
                    modal_data.current_end_idx = idx,
                None => 
                    modal_data.current_end_idx = 0,
            }
            if let Some(ids) = &response.ids {
                if let Some(names) = &response.names {
                    for (id, ex_idx) in ids.iter().enumerate() {
                        modal_data.exercise_list
                            .push((*ex_idx, names[id].clone()));
                    }
                }
            } 
        },
        Err(err) => modal_data.error = format!("{}", err),
    };
}


pub fn show_exercise_download_modal(ui: &mut Ui, modal_data: &mut ExerciseDownloadModal, exercise_files: &mut Vec<UnloadedExerciseData>) -> bool {
    if modal_data.exercise_list.is_empty() {
        update_modal_data(modal_data);
    }

    let mut close_current_exercise = false;
    let mut return_after_current_exercise = false;
    if let Some(exercise) = &modal_data.current_exercise {
        return_after_current_exercise = true;
        ui.vertical(|ui| {
            ui.set_width(700.0);
            let width = ui.available_width();
            let title = format!("Задание \"{}\"", exercise.name);
            ui.label(text(title, 34.0));
            ui.separator();
            ui.collapsing(text("Код задания", 20.0), |ui| {
                ui.set_height(400.0);
                ScrollArea::vertical().show(ui, |ui| {
                    ui.code(text(&exercise.lua_code, 12.0));
                });
            });

            let trusted_text = match exercise.trusted {
                true => text("Проверенное задание", 20.0),
                false => text(format!("{}{}", "Это задание не было проверено. ", 
                        "Загружайте его, если вы доверяете автору."), 20.0),
            };
            ui.label(trusted_text);

            ui.separator();
            if ui.add_sized(Vec2::new(width, 50.0), Button::new(text("Добавить задание", 24.0))).clicked() {
                let mut is_replaced = false;
                for data in &mut *exercise_files {
                    if data.name == exercise.name {
                        data.lua_code = exercise.lua_code.clone();
                        is_replaced = true;
                    }
                }

                if !is_replaced {
                    exercise_files.push(UnloadedExerciseData {
                        lua_code: exercise.lua_code.clone(),
                        name: exercise.name.clone(),
                    });
                }
                modal_data.is_open = false;
                close_current_exercise = true;
            }
            if ui.add_sized(Vec2::new(width, 50.0), Button::new(text("Отмена", 24.0))).clicked() {
                close_current_exercise = true;
            }
        });
    }

    if close_current_exercise {
        modal_data.current_exercise = None;
        modal_data.is_open = false;
        return true
    }
    if return_after_current_exercise { return false }

    ui.vertical_centered(|ui| {
        ui.set_width(700.0);
        let width = ui.available_width();
        ui.label(text("Скачивание заданий", 34.0));
        ui.separator();
        ui.horizontal(|ui| {
            let searchbar = TextEdit::singleline(&mut modal_data.search_query)
                .hint_text(text("Введите запрос", 24.0));
            ui.add_sized(Vec2::new(width - 100.0, 30.0), searchbar);
            if ui.add_sized(Vec2::new(92.0, 30.0), Button::new(text("Найти", 24.0))).clicked() {
                update_modal_data(modal_data);
            }
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Репозиторий: ");
            ui.text_edit_singleline(&mut modal_data.current_repo);
        });
        ui.horizontal(|ui| {
            if ui.checkbox(&mut modal_data.trusted_only, "Показывать только проверенные задания").clicked() {
                update_modal_data(modal_data);
            }
        });
        ui.separator();
        if !modal_data.error.is_empty() {
            let error_text =
                format!("Ошибка: {}", modal_data.error);
            ui.label(text(error_text, 32.0));
            return
        }
        ui.vertical(|ui| {
            ui.set_height(350.0);
            ScrollArea::vertical().show(ui, |ui| {
                let size = Vec2::new(ui.available_width(), 40.0);
                let mut clicked_idx = None;
                for exercise in &modal_data.exercise_list {
                    let button_text = text(format!("{} (ID {})", exercise.1, exercise.0), 24.0);
                    if ui.add_sized(size, Button::new(button_text)).clicked() {
                        clicked_idx = Some(exercise.0);
                    }
                }
                if let Some(clicked_idx) = clicked_idx {
                    get_current_exercise(modal_data, clicked_idx);
                }
            });
        });
        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);
        if ui.add_sized(Vec2::new(width, 50.0), Button::new(text("Отмена", 24.0))).clicked() {
            modal_data.is_open = false;
        }
    });

    false
}

#[derive(Deserialize, Debug)]
pub struct ExerciseListResponse {
    #[serde(rename(deserialize = "Names"))]
    names: Option<Vec<String>>,
    #[serde(rename(deserialize = "Ids"))]
    ids: Option<Vec<usize>>,
    #[serde(rename(deserialize = "TrustedIds"))]
    trusted_ids: Option<Vec<usize>>,
    #[serde(rename(deserialize = "EndIdx"))]
    end_idx: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct CurrentExerciseResponse {
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Trusted"))]
    trusted: bool,
    #[serde(rename(deserialize = "LuaCode"))]
    lua_code: String,
}
