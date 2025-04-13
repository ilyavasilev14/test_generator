use std::{collections::HashMap, path::PathBuf, sync::mpsc::{self, Receiver, Sender}, thread};

use config::{load_config, save_config, AppConfig};
use create_exercise::CreateExerciseData;
use eframe::App;
use egui::{Button, CentralPanel, RichText, ScrollArea, Style, Vec2, Visuals};
use exercise::{display_exercise, exercises_count_string, ExerciseData};
use exercise_download::ExerciseDownloadModal;
use exercise_list::RfdDataType;

mod exercise;
mod exercise_list;
mod exercise_download;
mod config;
mod custom_gui;
mod html_render;
mod create_exercise;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Генератор учебных заданий",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::light(),
                ..Default::default()
            });
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            return Ok(Box::new(TestBuilderApp::new()))
        }),
    )
}

struct TestBuilderApp {
    exercises_choice: HashMap<String, usize>,
    exercises: Vec<ExerciseData>,
    config: AppConfig,
    exercise_download_modal: ExerciseDownloadModal,
    create_exercise_data: Option<CreateExerciseData>,
    new_exercise_modal_open: bool,
    rfd_sender: Sender<(RfdDataType, Option<PathBuf>)>,
    rfd_receiver: Receiver<(RfdDataType, Option<PathBuf>)>,
}

impl TestBuilderApp {
    fn new() -> TestBuilderApp {
        let config = load_config();
        let (rfd_sender, rfd_receiver) = mpsc::channel();
        TestBuilderApp {
            exercises: Vec::new(),
            exercises_choice: HashMap::new(),
            config,
            exercise_download_modal: ExerciseDownloadModal::default(),
            create_exercise_data: None,
            new_exercise_modal_open: false,
            rfd_sender,
            rfd_receiver,
        }
    }
}

impl App for TestBuilderApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if self.exercises.is_empty() {
            if let Some(exercise_data) = &mut self.create_exercise_data {
                if exercise_data.draw(ctx) {
                    self.create_exercise_data = None;
                }
                return
            }
            match exercise_list::display_list(
                ctx,
                &mut self.exercises_choice,
                &mut self.config.unloaded_exercises,
                &mut self.exercises,
                &mut self.new_exercise_modal_open,
                &mut self.exercise_download_modal,
                &mut self.create_exercise_data,
            ) {
                exercise_list::DisplayListResponse::CreateExercise => {
                    let sender = self.rfd_sender.clone();
                    thread::spawn(move || {
                        let path = rfd::FileDialog::new()
                            .set_file_name("exercise.lua").save_file();
                        sender.send((RfdDataType::CreateExercise, path)).unwrap();
                    });
                },
                exercise_list::DisplayListResponse::LoadExercise => {
                    let sender = self.rfd_sender.clone();
                    thread::spawn(move || {
                        let path = rfd::FileDialog::new()
                            .pick_file();
                        sender.send((RfdDataType::LoadExercise, path)).unwrap();
                    });
                },
                exercise_list::DisplayListResponse::UpdateConfig => save_config(&self.config),
                exercise_list::DisplayListResponse::None => (),
            }

            if let Ok((data_type, path)) = self.rfd_receiver.try_recv() {
                match data_type {
                    RfdDataType::CreateExercise => {
                        if let Some(path) = path {
                            let path = path.with_extension("lua");
                            self.create_exercise_data =
                                crate::create_exercise::create_file(path.to_str().unwrap());
                        }
                    },
                    RfdDataType::LoadExercise => {
                        exercise_list::load_picked_exercise(&mut self.config.unloaded_exercises, path);
                    },
                }
            }
            return
        }

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.add_sized(Vec2::new(60.0, 60.0), Button::new(RichText::new("◀").size(44.0))).clicked() {
                            self.exercises_choice.clear();
                            self.exercises.clear();
                        }
                        if ui.add_sized(Vec2::new(60.0, 60.0), Button::new(RichText::new("🗋").size(44.0))).clicked() {
                            html_render::render(&self.exercises);
                        }
                        let text = format!("Вариант{}", exercises_count_string(self.exercises.len()));
                        ui.heading(RichText::new(text).size(44.0))
                    });
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    for (exercise_idx, exercise) in self.exercises.iter_mut().enumerate() {
                        display_exercise(exercise, ui, exercise_idx);
                    }
                });
            });
        });
    }
}
