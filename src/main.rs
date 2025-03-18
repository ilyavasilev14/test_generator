use std::collections::HashMap;

use config::{load_config, save_config, AppConfig};
use create_exercise::CreateExerciseData;
use eframe::App;
use egui::{Button, CentralPanel, RichText, ScrollArea, Style, Vec2, Visuals};
use exercise::{display_exercise, exercises_count_string, AnswerState, ExerciseData};
use exercise_download::ExerciseDownloadModal;

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
        "My egui App",
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
    session_statistics: Vec<AnswerState>,
    exercises_choice: HashMap<String, usize>,
    exercises: Vec<ExerciseData>,
    config: AppConfig,
    exercise_download_modal: ExerciseDownloadModal,
    create_exercise_data: Option<CreateExerciseData>,
    new_exercise_modal_open: bool,
}

impl TestBuilderApp {
    fn new() -> TestBuilderApp {
        let config = load_config();
        TestBuilderApp {
            exercises: Vec::new(),
            session_statistics: Vec::new(),
            exercises_choice: HashMap::new(),
            config,
            exercise_download_modal: ExerciseDownloadModal::default(),
            create_exercise_data: None,
            new_exercise_modal_open: false,
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
            if exercise_list::display_list(
                ctx,
                &mut self.exercises_choice,
                &mut self.config.unloaded_exercises,
                &mut self.exercises,
                &mut self.new_exercise_modal_open,
                &mut self.exercise_download_modal,
                &mut self.create_exercise_data,
            ) {
                save_config(&self.config);
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
