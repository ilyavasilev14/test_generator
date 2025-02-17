use egui::{Button, RichText, TextEdit, Ui, Vec2};
use mlua::Lua;
use serde::{Deserialize, Serialize};

use crate::custom_gui::GeneratorGUI;

#[derive(Debug)]
pub enum AnswerState {
    NotSolved,
    Correct,
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
            // call a lua function to check the current answer
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
