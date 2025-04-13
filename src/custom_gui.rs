use egui::{Button, RichText, Ui};
use mlua::{Function, UserData};

#[derive(Debug, Default)]
pub struct GeneratorGUI {
    pub containers: Vec<Container>
}

#[derive(Default, Debug)]
pub enum ContainerType {
    #[default]
    Horizontal,
    Vertical
}

#[derive(Debug)]
pub enum WidgetType {
    Button(String, f32, Option<[f32; 2]>),
    Label(String, f32),
    Image(String),
    Separator,
    Space(f32),
}

#[derive(Default, Debug)]
pub struct Container {
    pub container_type: ContainerType,
    pub widgets: Vec<(WidgetType, Option<Function>)>,
    pub children: Vec<Container>,
}

impl Container {
    fn draw_container_widgets(&mut self, ui: &mut Ui) {
        for (widget, on_click) in &mut self.widgets {
            match widget {
                WidgetType::Button(text, font_size, size) => {
                    let text = RichText::new(text.to_string()).size(*font_size);
                    let widget = match size {
                        Some(size) => 
                            ui.add_sized(*size, Button::new(text)),
                        None => ui.button(text),
                    };

                    if let Some(function) = on_click {
                        if widget.clicked() {
                            let _: Result<(), mlua::Error> = function.call(());
                        }
                    }
                },
                WidgetType::Label(text, font_size) => {
                    let text = RichText::new(text.to_string()).size(*font_size);
                    let widget = ui.label(text);

                    if let Some(function) = on_click {
                        if widget.clicked() {
                            let _: Result<(), mlua::Error> = function.call(());
                        }
                    }
                },
                WidgetType::Image(path) => {
                    ui.image("file://".to_string() + &path);
                },
                WidgetType::Separator => {
                    ui.separator(); 
                },
                WidgetType::Space(space) => ui.add_space(*space),
            }
        }
    }

    fn draw_children_containers(&mut self, ui: &mut Ui) {
        for child in &mut self.children {
            child.draw_container(ui);
        }
    }

    fn draw_container(&mut self, ui: &mut Ui) {
        match self.container_type {
            ContainerType::Horizontal => ui.horizontal(|ui| {
                self.draw_container_widgets(ui);
                self.draw_children_containers(ui);
            }),
            ContainerType::Vertical => ui.vertical(|ui| {
                self.draw_container_widgets(ui);
                self.draw_children_containers(ui);
            }),
        };
    }
}

impl GeneratorGUI {
    pub fn draw(&mut self, ui: &mut Ui) {
        for container in &mut self.containers {
            container.draw_container(ui);
        }
    }
}

impl UserData for GeneratorGUI {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("vertical", |lua, ui, gui_fn: Function| {
            ui.containers.push(Container {
                container_type: ContainerType::Vertical,
                ..Default::default()
            });

            let child = ui.containers.last_mut().unwrap();
            let _ = lua.scope(|scope| {
                let userdata = scope.create_userdata_ref_mut(child);
                let _ = gui_fn.call::<()>(userdata);
                Ok(())
            });

            Ok(())
        });

        methods.add_method_mut("horizontal", |lua, ui, gui_fn: Function| {
            ui.containers.push(Container {
                container_type: ContainerType::Horizontal,
                ..Default::default()
            });

            let child = ui.containers.last_mut().unwrap();
            let _ = lua.scope(|scope| {
                let userdata = scope.create_userdata_ref_mut(child);
                let _ = gui_fn.call::<()>(userdata);
                Ok(())
            });

            Ok(())
        });
    }

    fn register(registry: &mut mlua::UserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}

impl UserData for Container {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("label", |_, ui, (text, font_size, on_click): (String, f32, Option<Function>)| {
            Ok(ui.widgets.push((WidgetType::Label(text, font_size), on_click)))
        });

        methods.add_method_mut("button", |_, ui, (text, font_size, size, on_click): (String, f32, Option<[f32; 2]>, Option<Function>)| {
            Ok(ui.widgets.push((WidgetType::Button(text, font_size, size), on_click)))
        });

        methods.add_method_mut("image", |_, ui, image_path: String| {
            Ok(ui.widgets.push((WidgetType::Image(image_path), None)))
        });

        methods.add_method_mut("separator", |_, ui, (): ()| {
            Ok(ui.widgets.push((WidgetType::Separator, None)))
        });

        methods.add_method_mut("add_space", |_, ui, space: f32| {
            Ok(ui.widgets.push((WidgetType::Space(space), None)))
        });

        methods.add_method_mut("vertical", |lua, ui, gui_fn: Function| {
            ui.children.push(Container {
                container_type: ContainerType::Vertical,
                ..Default::default()
            });

            let child = ui.children.last_mut().unwrap();
            let _ = lua.scope(|scope| {
                let userdata = scope.create_userdata_ref_mut(child);
                let _ = gui_fn.call::<()>(userdata);
                Ok(())
            });

            Ok(())
        });

        methods.add_method_mut("horizontal", |lua, ui, gui_fn: Function| {
            ui.children.push(Container {
                container_type: ContainerType::Horizontal,
                ..Default::default()
            });

            let child = ui.children.last_mut().unwrap();
            let _ = lua.scope(|scope| {
                let userdata = scope.create_userdata_ref_mut(child);
                let _ = gui_fn.call::<()>(userdata);
                Ok(())
            });

            Ok(())
        });
    }

    fn register(registry: &mut mlua::UserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}
