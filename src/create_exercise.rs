const DEFAULT_CODE: &str =
"-- логика создания заданий пишется на языке программирования Lua

-- генерация задания
answer = math.random(0, 100)
text = \"Ответ на это задание = \" .. answer

-- слеующая функция возвращает текст задания
function get_exercise_text()
    return text
end

-- слеующая функция возвращает правильный ответ на задание
function get_exercise_right_answer()
    return answer
end

-- слеующая функция возвращает название генератора заданий
function get_exercise_title()
    return \"Название генератора!\"
end

--[[ свой интерфейс для задания, опционально
function get_custom_gui(gui)
    gui:vertical(function (vertical_gui)
        vertical_gui:label(\"текст!\", 72)
        vertical_gui:horizontal(function (horizontal_gui)
            horizontal_gui:button(\"кнопка 1\", 20, {500.0, 100})
            horizontal_gui:button(\"кнопка 2\", 23, {200.0, 100})
        end)
    end)
end]]--

--[[ своя логика проверки задания, опционально
function check_exercise(answer)
end
]]--
";

use std::{fs::{self, File}, io::Write, path::PathBuf, thread};

use egui::{Button, CentralPanel, Context, SidePanel, Vec2};
use mlua::{Function, Lua, LuaOptions, ObjectLike, StdLib};

use crate::{custom_gui::GeneratorGUI, exercise::{display_exercise, AnswerState, ExerciseData}, exercise_list::text};

pub struct CreateExerciseData {
    current_exercise: Result<ExerciseData, String>,
    path: String,
}

impl CreateExerciseData {
    pub fn draw(&mut self, ctx: &Context) -> bool {
        let mut close = false;

        SidePanel::right("RightSidePanel").exact_width(300.0).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let size = Vec2::new(ui.available_width(), 50.0);
                let update = Button::new(text("Обновить", 24.0));
                let directory = Button::new(text("Открыть директорию", 24.0));
                let exit = Button::new(text("Выход", 24.0));

                if ui.add_sized(size, update).clicked() {
                    match fs::read_to_string(&self.path) {
                        Ok(file) => {
                            self.current_exercise = update_exercise(file);
                        },
                        Err(err) => {
                            self.current_exercise = Err(err.to_string());
                        },
                    }
                }
                if ui.add_sized(size, directory).clicked() {
                    let dir_path = PathBuf::from(self.path.clone());
                    let dir_path = dir_path.with_file_name("");
                    thread::spawn(|| {
                        if let Err(err) = open::that(dir_path) {
                            println!("Can't even open a file {} in a file manager!", err)
                        }
                    });
                }
                if ui.add_sized(size, exit).clicked() {
                    close = true;
                }
            })
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                match &mut self.current_exercise {
                    Ok(current_exercise) => 
                        display_exercise(current_exercise, ui, 0),
                    Err(display_err) => {
                        let display_err =
                            format!("Ошибка: {}", display_err);
                        ui.label(text(display_err, 24.0));
                    },
                }
            });
        });

        close
    }
}

pub fn create_file(path: &str) -> Option<CreateExerciseData> {
    match File::open(path) {
        Ok(_) => {
            match fs::read_to_string(path) {
                Ok(file) => {
                    let current_exercise = update_exercise(file);
                    Some(CreateExerciseData {
                        current_exercise,
                        path: path.to_string()
                    })
                },
                Err(err) => {
                    Some(CreateExerciseData {
                        current_exercise: Err(err.to_string()),
                        path: path.to_string()
                    })
                },
            }
        },
        Err(_) => {
            let new_file = File::create_new(&path);
            match new_file {
                Ok(mut file) => {
                    let _ = file.write_all(DEFAULT_CODE.as_bytes());
                    match fs::read_to_string(path) {
                        Ok(file) => {
                            let current_exercise = update_exercise(file);
                            Some(CreateExerciseData {
                                current_exercise,
                                path: path.to_string()
                            })
                        },
                        Err(err) => {
                            Some(CreateExerciseData {
                                current_exercise: Err(err.to_string()),
                                path: path.to_string()
                            })
                        }
                    }
                },
                Err(err) => {
                    println!("Failed to create a file @ {}, err: {}", path, err);
                    None
                },
            }
        },
    }
}

fn update_exercise(code: String) -> Result<ExerciseData, String> {
    let lua_vm = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::default()).expect("Failed to create a Lua VM!");
    let chunk = lua_vm.load(&code);
    if let Err(err) = chunk.exec() {
        println!("Lua code execution failed: {}", err);
        return Err(format!("{:?}", err));
    }
    let right_answer: Result<Option<String>, mlua::Error> = lua_vm.globals().call_function("get_exercise_right_answer", ());
    let exercise_text: Result<Option<String>, mlua::Error> = lua_vm.globals().call_function("get_exercise_text", ());
    let name: Result<Option<String>, mlua::Error> = lua_vm.globals().call_function("get_exercise_title", ());
    let right_answer = match right_answer {
        Ok(answer) => answer,
        Err(err) => {
            println!("Failed to run the function get_exercise_right_answer!: {}", err);
            return Err(format!("{:?}", err));
        },
    };
    let exercise_text = match exercise_text {
        Ok(text) => text,
        Err(err) => {
            println!("Failed to run the function get_exercise_text!: {}", err);
            return Err(format!("{:?}", err));
        },
    };
    let name = match name {
        Ok(text) => text.unwrap_or(String::from("Задание без названия")),
        Err(err) => {
            println!("Failed to run the function get_exercise_text!: {}", err);
            String::from("Задание без названия")
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

    Ok(ExerciseData {
        lua_vm,
        right_answer,
        exercise_text,
        current_input: String::new(),
        answer_state: AnswerState::NotSolved,
        name,
        custom_gui,
    })
}
