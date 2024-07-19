use eframe::egui::{self, Button, Color32, Frame, Label, Layout, Margin, RichText, Ui};
use serde::{Deserialize, Serialize};
use utils::{load_settings, Settings};

use crate::{
    cryptography::decrypt_text,
    data_base::{
        get::{get_entries, get_master},
        post::{add_entry, save_current_list, save_master},
        Entry, EntryDisplay,
    },
    helpers::timestamp_as_date,
};

mod utils;

const GREEN: egui::Color32 = egui::Color32::from_rgb(105, 219, 124);
const RED: egui::Color32 = egui::Color32::from_rgb(255, 135, 135);
const BLACK: egui::Color32 = egui::Color32::from_rgb(18, 19, 20);
const DEFAULT_COLOR: egui::Color32 = egui::Color32::from_rgb(116, 143, 252);
const DARK_BG: egui::Color32 = egui::Color32::from_rgb(28, 29, 30);
const SECONDARY_DARK: egui::Color32 = egui::Color32::from_rgb(33, 36, 39);
const BORDER_DARK: egui::Color32 = egui::Color32::from_rgb(53, 59, 65);
const BRIGHT_BG: egui::Color32 = egui::Color32::from_rgb(248, 249, 250);
const SECONDARY_BRIGHT: egui::Color32 = egui::Color32::from_rgb(233, 236, 239);
const BORDER_BRIGHT: egui::Color32 = egui::Color32::from_rgb(206, 212, 218);
const BRIGHT_DEFAULT_GREY: egui::Color32 = egui::Color32::from_rgb(134, 142, 150);
const CORNER_ROUND: f32 = 2.0;

#[derive(Default, Clone, Deserialize, Serialize, Debug)]
enum Mode {
    #[default]
    Bright,
    Dark,
}
impl Mode {
    pub fn switch(self) -> Mode {
        match self {
            Mode::Dark => Mode::Bright,
            Mode::Bright => Mode::Dark,
        }
    }
}

pub fn setup() {
    let options = eframe::NativeOptions {
        run_and_return: true,
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([760.0, 380.0])
            .with_inner_size([900.0, 450.0]),
        centered: true,
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "Vault",
        options,
        Box::new(|cc| Ok(Box::new(Vault::new(cc)))),
    )
    .unwrap();
}

#[derive(Default)]
struct Vault {
    entries: Vec<Entry>,
    has_error: Option<bool>,
    selected_entry: Option<EntryDisplay>,
    toast_message: String,
    state: State,
    logged_in: bool,
    password_visible: bool,
    input_name: String,
    input_user_name: String,
    input_password: String,
    master_password: String,
    verify_master: String,
    toast_end_time: i64,
    first_use: bool,
    passwords_match: bool,
    delete_dialog: bool,
    settings: Settings,
}
impl Vault {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let has_error = None;
        let entries = get_entries().unwrap();
        let toast_message = String::new();
        let selected_entry = None;
        let state = State::default();
        let logged_in = false;
        let password_visible = false;
        let input_name = String::new();
        let input_user_name = String::new();
        let input_password = String::new();
        let master_password = String::new();
        let verify_master = String::new();
        let toast_end_time = 0;
        let passwords_match = true;
        let delete_dialog = false;
        let settings = load_settings();
        let first_use = match get_master() {
            Ok(_) => false,
            Err(err) => &err.code() == "[DB_G_GM-3]",
        };

        Self {
            entries,
            has_error,
            toast_message,
            selected_entry,
            state,
            logged_in,
            password_visible,
            input_name,
            input_user_name,
            input_password,
            master_password,
            verify_master,
            toast_end_time,
            first_use,
            passwords_match,
            delete_dialog,
            settings,
        }
    }

    fn show_entry_list(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("Entry List")
            .frame(self.panel_frame())
            .max_width(ctx.available_rect().width() * 0.5)
            .min_width(ctx.available_rect().width() * 0.25)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|panel_ui| {
                    egui::ScrollArea::vertical()
                        .max_width(f32::INFINITY)
                        .max_height(ctx.available_rect().height() * 0.8)
                        .auto_shrink(false)
                        .show(panel_ui, |sa_ui| {
                            sa_ui.vertical_centered_justified(|centered_ui| {
                                for entry in &self.entries {
                                    let is_selected = if self.selected_entry.is_none() {
                                        false
                                    } else {
                                        self.selected_entry.clone().unwrap().id == entry.id()
                                    };
                                    let selected = Vault::selectable_text(
                                        centered_ui,
                                        &entry.name(),
                                        is_selected,
                                        &self.settings.mode,
                                    );
                                    if selected.clicked() {
                                        self.password_visible = false;
                                        self.state = State::DisplayEntry;
                                        self.selected_entry = Some(entry.get_details().unwrap());
                                    }
                                }
                            });
                        });

                    panel_ui.separator();

                    let add_entry =
                        Vault::app_button("Add Entry", 16.0, None, self.settings.mode.clone());
                    let toggle_mode = Vault::app_button(
                        match self.settings.mode {
                            Mode::Bright => "Dark Mode",
                            Mode::Dark => "Bright Mode",
                        },
                        16.0,
                        match self.settings.mode {
                            Mode::Dark => Some(BRIGHT_BG),
                            Mode::Bright => Some(DARK_BG),
                        },
                        self.settings.mode.clone(),
                    );
                    let add_entry_button = panel_ui.add(add_entry);
                    panel_ui.add_space(5.0);
                    let toggle_mode = panel_ui.add(toggle_mode);
                    if add_entry_button.clicked() {
                        self.selected_entry = None;
                        self.password_visible = false;
                        self.state = State::AddingEntry;
                        self.input_name = String::new();
                        self.input_user_name = String::new();
                        self.input_password = String::new();
                    }
                    if toggle_mode.clicked() {
                        self.settings.mode = self.settings.mode.clone().switch();
                        match utils::save_settings(&self.settings) {
                            Ok(_) => {}
                            Err(_) => match utils::save_settings(&self.settings) {
                                Ok(_) => {}
                                Err(err) => {
                                    self.has_error = Some(true);
                                    self.toast_message = err.toast_message();
                                    self.set_toast_time();
                                }
                            },
                        }
                    }
                });
            });
    }

    fn show_entry(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        match &self.selected_entry {
            Some(entry) => {
                if self.delete_dialog {
                    let text = Vault::app_text(
                        &format!(
                            "Do you want to delete the Entry \"{}\"?",
                            &self.selected_entry.clone().unwrap().name
                        ),
                        20.0,
                        None,
                        self.settings.mode.clone(),
                    );
                    let mut pop_up = egui::Frame::popup(&egui::Style::default()).begin(ui);
                    {
                        pop_up.content_ui.label(text);
                        pop_up.content_ui.add_space(30.0);
                        pop_up.content_ui.with_layout(
                            Layout::left_to_right(egui::Align::Min),
                            |ui| {
                                let confirm = Vault::app_button(
                                    "Confirm",
                                    18.0,
                                    None,
                                    self.settings.mode.clone(),
                                );
                                let cancel = Vault::app_button(
                                    "Cancel",
                                    18.0,
                                    Some(BRIGHT_DEFAULT_GREY),
                                    self.settings.mode.clone(),
                                );

                                let confirm_button = ui.add(confirm);
                                let cancel_button = ui.add(cancel);

                                if cancel_button.clicked() {
                                    self.delete_dialog = false;
                                }
                                if confirm_button.clicked() {
                                    let mut entry_index = 0;
                                    for i in 0..self.entries.len() {
                                        if self.entries[i].id()
                                            == self.selected_entry.clone().unwrap().id
                                        {
                                            entry_index = i;
                                        }
                                    }
                                    let mut clone = self.entries.clone();
                                    clone.remove(entry_index);
                                    match save_current_list(clone.clone()) {
                                        Ok(_) => {
                                            self.toast_message = String::from("Entry Deleted");
                                            self.entries = clone;
                                            self.selected_entry = None;
                                            self.state = State::None;
                                            self.delete_dialog = false;
                                            self.set_toast_time();
                                        }
                                        Err(err) => {
                                            self.toast_message = format!(
                                                "{}\nPlease try again",
                                                err.toast_message()
                                            );
                                            self.has_error = Some(true);
                                            self.set_toast_time();
                                        }
                                    };
                                }
                            },
                        );
                    }
                } else {
                    ui.label(Vault::app_text(
                        "Name",
                        12.0,
                        None,
                        self.settings.mode.clone(),
                    ));
                    ui.label(Vault::app_text(
                        &entry.name,
                        18.0,
                        None,
                        self.settings.mode.clone(),
                    ));
                    ui.separator();

                    ui.label(Vault::app_text(
                        "Username",
                        12.0,
                        None,
                        self.settings.mode.clone(),
                    ));
                    ui.with_layout(
                        Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
                        |l_ui| {
                            let ui_width = l_ui.available_width();

                            let text_label = Label::new(Vault::app_text(
                                &entry.user_name,
                                18.0,
                                None,
                                self.settings.mode.clone(),
                            ))
                            .wrap_mode(egui::TextWrapMode::Wrap);

                            let label_r = l_ui.add(text_label);
                            let label_rect_width = label_r.rect.width();

                            let added_space = if label_rect_width > ui_width * 0.825 {
                                ui_width * 0.825 - (label_rect_width - ui_width)
                            } else {
                                ui_width * 0.825 - label_r.rect.width()
                            };

                            l_ui.add_space(added_space);

                            let copy =
                                Vault::app_button("Copy", 14.0, None, self.settings.mode.clone());
                            let copy_button = l_ui.add(copy);

                            if copy_button.clicked() {
                                let mut clipboard = clippers::Clipboard::get();
                                clipboard.write_text(&entry.user_name).unwrap();
                            }
                        },
                    );
                    ui.separator();

                    ui.label(Vault::app_text(
                        "Password",
                        12.0,
                        None,
                        self.settings.mode.clone(),
                    ));
                    ui.with_layout(
                        Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
                        |l_ui| {
                            let pw_text = if self.password_visible {
                                &entry.password
                            } else {
                                "********"
                            };
                            let display_button_text = if self.password_visible {
                                "Hide"
                            } else {
                                "Show"
                            };

                            let ui_width = l_ui.available_width();

                            let text_label = Label::new(Vault::app_text(
                                pw_text,
                                18.0,
                                None,
                                self.settings.mode.clone(),
                            ))
                            .wrap_mode(egui::TextWrapMode::Wrap);

                            let label_r = l_ui.add(text_label);
                            let label_rect_width = label_r.rect.width();

                            let added_space = if label_rect_width > ui_width * 0.825 {
                                ui_width * 0.825 - (label_rect_width - ui_width * 0.825)
                            } else {
                                ui_width * 0.825 - label_r.rect.width()
                            };

                            l_ui.add_space(added_space);

                            let copy =
                                Vault::app_button("Copy", 14.0, None, self.settings.mode.clone());
                            let visibility = Vault::app_button(
                                display_button_text,
                                14.0,
                                None,
                                self.settings.mode.clone(),
                            );
                            let copy_button = l_ui.add(copy);
                            let visibility_button = l_ui.add(visibility);

                            if copy_button.clicked() {
                                let mut clipboard = clippers::Clipboard::get();
                                clipboard.write_text(&entry.password).unwrap();
                            }
                            if visibility_button.clicked() {
                                self.password_visible = !self.password_visible;
                            }
                        },
                    );
                    ui.add_space(30.0);

                    let edit = Vault::app_button("Edit", 16.0, None, self.settings.mode.clone());
                    let delete =
                        Vault::app_button("Delete", 16.0, Some(RED), self.settings.mode.clone());
                    ui.with_layout(Layout::left_to_right(egui::Align::Min), |b_ui| {
                        let edit_button = b_ui.add(edit);
                        let delete_button = b_ui.add(delete);
                        if edit_button.clicked() {
                            self.password_visible = false;
                            let entry = &self.selected_entry.clone().unwrap();
                            self.input_name = entry.name.clone();
                            self.input_user_name = entry.user_name.clone();
                            self.input_password = entry.password.clone();
                            self.state = State::EditEntry;
                        }
                        if delete_button.clicked() {
                            self.delete_dialog = true;
                        }
                    });
                    ui.add_space(15.0);

                    ui.label(Vault::app_text(
                        &format!("Created At: {}", timestamp_as_date(entry.created_at)),
                        12.0,
                        None,
                        self.settings.mode.clone(),
                    ));
                    ui.label(Vault::app_text(
                        &format!("Last Edited: {}", timestamp_as_date(entry.last_edited)),
                        12.0,
                        None,
                        self.settings.mode.clone(),
                    ));
                }
            }
            None => {}
        }
    }

    fn edit_entry(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        let visibility_text = if self.password_visible {
            "Hide"
        } else {
            "Show"
        };
        ui.label(Vault::app_text(
            "Name",
            12.0,
            None,
            self.settings.mode.clone(),
        ));
        Vault::line_edit_frame(&self.settings.mode).show(ui, |f_ui| {
            Vault::single_line_edit(f_ui, &mut self.input_name, None, &self.settings.mode)
        });
        ui.separator();

        ui.label(Vault::app_text(
            "Username",
            12.0,
            None,
            self.settings.mode.clone(),
        ));
        Vault::line_edit_frame(&self.settings.mode).show(ui, |f_ui| {
            Vault::single_line_edit(f_ui, &mut self.input_user_name, None, &self.settings.mode)
        });
        ui.separator();

        let visibility = Vault::app_button(visibility_text, 14.0, None, self.settings.mode.clone());
        ui.label(Vault::app_text(
            "Password",
            12.0,
            None,
            self.settings.mode.clone(),
        ));
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |l_ui| {
            Vault::line_edit_frame(&self.settings.mode).show(l_ui, |f_ui| {
                Vault::single_line_edit(
                    f_ui,
                    &mut self.input_password,
                    Some(!self.password_visible),
                    &self.settings.mode,
                );
            });
            let visibility_button = l_ui.add(visibility);

            if visibility_button.clicked() {
                self.password_visible = !self.password_visible;
            }
        });
        ui.add_space(30.0);

        let save = Vault::app_button("Save", 16.0, Some(GREEN), self.settings.mode.clone());
        let cancel = Vault::app_button(
            "Cancel",
            16.0,
            Some(BRIGHT_DEFAULT_GREY),
            self.settings.mode.clone(),
        );

        ui.with_layout(Layout::left_to_right(egui::Align::Min), |b_ui| {
            let save_button = b_ui.add(save);
            let cancel_button = b_ui.add(cancel);

            if save_button.clicked() {
                let mut entry_index = 0;
                for i in 0..self.entries.len() {
                    if self.entries[i].id() == self.selected_entry.clone().unwrap().id {
                        entry_index = i;
                    }
                }
                match self.entries[entry_index].edit(
                    &self.input_name,
                    &self.input_user_name,
                    &self.input_password,
                ) {
                    Ok(_) => match save_current_list(self.entries.clone()) {
                        Ok(_) => {
                            self.password_visible = false;
                            self.toast_message = String::from("Entry Saved");
                            self.has_error = Some(false);
                            self.selected_entry =
                                Some(self.entries[entry_index].get_details().unwrap());
                            self.state = State::DisplayEntry;
                            self.input_name = String::new();
                            self.input_user_name = String::new();
                            self.input_password = String::new();
                            self.set_toast_time();
                        }
                        Err(err) => {
                            self.toast_message =
                                format!("{}\nPlease try again", err.toast_message());
                            self.has_error = Some(true);
                            self.set_toast_time();
                        }
                    },
                    Err(err) => {
                        self.toast_message = format!("{}\nPlease try again", err.toast_message());
                        self.has_error = Some(true);
                        self.set_toast_time();
                    }
                }
            }
            if cancel_button.clicked() {
                self.password_visible = false;
                self.state = State::DisplayEntry;
                self.input_name = String::new();
                self.input_user_name = String::new();
                self.input_password = String::new();
            }
        });
    }

    fn add_entry(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        let visibility_text = if self.password_visible {
            "Hide"
        } else {
            "Show"
        };

        ui.label(Vault::app_text(
            "Name",
            12.0,
            None,
            self.settings.mode.clone(),
        ));
        Vault::line_edit_frame(&self.settings.mode).show(ui, |f_ui| {
            Vault::single_line_edit(f_ui, &mut self.input_name, None, &self.settings.mode)
        });
        ui.separator();

        ui.label(Vault::app_text(
            "Username",
            12.0,
            None,
            self.settings.mode.clone(),
        ));
        Vault::line_edit_frame(&self.settings.mode).show(ui, |f_ui| {
            Vault::single_line_edit(f_ui, &mut self.input_user_name, None, &self.settings.mode)
        });
        ui.separator();

        let visibility = Vault::app_button(visibility_text, 14.0, None, self.settings.mode.clone());
        ui.label(Vault::app_text(
            "Password",
            12.0,
            None,
            self.settings.mode.clone(),
        ));
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |l_ui| {
            Vault::line_edit_frame(&self.settings.mode).show(l_ui, |f_ui| {
                Vault::single_line_edit(
                    f_ui,
                    &mut self.input_password,
                    Some(!self.password_visible),
                    &self.settings.mode,
                )
            });
            let visibility_button = l_ui.add(visibility);

            if visibility_button.clicked() {
                self.password_visible = !self.password_visible;
            }
        });
        ui.add_space(30.0);

        let save = Vault::app_button("Save", 16.0, Some(GREEN), self.settings.mode.clone());
        let cancel = Vault::app_button(
            "Cancel",
            16.0,
            Some(BRIGHT_DEFAULT_GREY),
            self.settings.mode.clone(),
        );

        ui.with_layout(Layout::left_to_right(egui::Align::Min), |b_ui| {
            let save_button = b_ui.add(save);
            let cancel_button = b_ui.add(cancel);

            if save_button.clicked() {
                if String::is_empty(&self.input_name)
                    && String::is_empty(&self.input_user_name)
                    && String::is_empty(&self.input_password)
                {
                    return;
                }

                match add_entry(
                    &self.input_name,
                    &self.input_user_name,
                    &self.input_password,
                    self.entries.clone(),
                ) {
                    Ok(entries) => {
                        self.toast_message = String::from("Entry Saved");
                        self.has_error = Some(false);
                        self.entries = entries;
                        self.selected_entry =
                            Some(self.entries[self.entries.len() - 1].get_details().unwrap());
                        self.password_visible = false;
                        self.state = State::DisplayEntry;
                        self.input_name = String::new();
                        self.input_user_name = String::new();
                        self.input_password = String::new();
                        self.set_toast_time();
                    }
                    Err(err) => {
                        self.toast_message = format!("{}\nPlease try again", err.toast_message());
                        self.has_error = Some(true);
                        self.set_toast_time();
                    }
                }
            }
            if cancel_button.clicked() {
                self.password_visible = false;
                self.state = State::None;
                self.input_name = String::new();
                self.input_user_name = String::new();
                self.input_password = String::new();
            }
        });
    }

    // fn show_toast(&mut self, ctx: &egui::Context) {
    //     let ctx_right_bottom = ctx.available_rect().right_bottom();
    //     let width = 150.0;
    //     let x = ctx_right_bottom.x - width;
    //     let y = ctx_right_bottom.y - 50.0;
    // }

    fn authenticate(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(20.0),
                fill: match &self.settings.mode {
                    Mode::Bright => BRIGHT_BG,
                    Mode::Dark => BLACK,
                },
                ..Default::default()
            })
            .show(ctx, |ui| {
                let display_button_text = if self.password_visible {
                    "Hide"
                } else {
                    "Show"
                };

                let visibility =
                    Vault::app_button(display_button_text, 14.0, None, self.settings.mode.clone());
                let login =
                    Vault::app_button("Login", 16.0, Some(GREEN), self.settings.mode.clone());
                let fail = Vault::app_text(
                    "The password is not correct!",
                    20.0,
                    Some(RED),
                    self.settings.mode.clone(),
                );

                ui.label(RichText::new("Master Password").size(16.0).color(
                    match self.settings.mode {
                        Mode::Dark => BRIGHT_BG,
                        Mode::Bright => DARK_BG,
                    },
                ));

                ui.with_layout(Layout::left_to_right(egui::Align::Min), |l_ui| {
                    let input_frame = Vault::line_edit_frame(&self.settings.mode);
                    input_frame.show(l_ui, |f_ui| {
                        Vault::single_line_edit(
                            f_ui,
                            &mut self.master_password,
                            Some(!self.password_visible),
                            &self.settings.mode,
                        );
                    });
                    let visibility_button = l_ui.add(visibility);
                    if visibility_button.clicked() {
                        self.password_visible = !self.password_visible;
                    }
                });
                ui.add_space(5.0);
                let login_button = ui.add(login);
                if !self.passwords_match {
                    ui.label(fail);
                }

                if login_button.clicked() {
                    if String::is_empty(&self.master_password) {
                        return;
                    }
                    let master = get_master().unwrap();
                    let master_decrypt = decrypt_text(&master, true, true).unwrap();

                    if master_decrypt == self.master_password {
                        self.passwords_match = true;
                        self.password_visible = false;
                        self.logged_in = true;
                    } else {
                        self.passwords_match = false;
                    }
                }
            });
    }

    fn set_master(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(20.0),
                fill: match self.settings.mode {
                    Mode::Bright => BRIGHT_BG,
                    Mode::Dark => Color32::BLACK,
                },
                ..Default::default()
            })
            .show(ctx, |ui| {
                let display_button_text = if self.password_visible {
                    "Hide"
                } else {
                    "Show"
                };

                let fail_text = if self.passwords_match {
                    "Something went wrong, please try again"
                } else {
                    "The Passwords do not match!"
                };

                let visibility =
                    Vault::app_button(display_button_text, 14.0, None, self.settings.mode.clone());
                let login =
                    Vault::app_button("Login", 16.0, Some(GREEN), self.settings.mode.clone());
                let fail = Vault::app_text(fail_text, 20.0, Some(RED), self.settings.mode.clone());

                ui.label(RichText::new("Set your Vault Password").color(
                    match self.settings.mode {
                        Mode::Dark => BRIGHT_BG,
                        Mode::Bright => DARK_BG,
                    },
                ));
                Vault::line_edit_frame(&self.settings.mode).show(ui, |f_ui| {
                    Vault::single_line_edit(
                        f_ui,
                        &mut self.master_password,
                        Some(!self.password_visible),
                        &self.settings.mode,
                    );
                });
                Vault::line_edit_frame(&self.settings.mode).show(ui, |f_ui| {
                    Vault::single_line_edit(
                        f_ui,
                        &mut self.verify_master,
                        Some(!self.password_visible),
                        &self.settings.mode,
                    );
                });
                ui.add_space(5.0);
                let visibility_button = ui.add(visibility);
                ui.add_space(5.0);
                let login_button = ui.add(login);
                if !self.passwords_match || self.has_error == Some(true) {
                    ui.label(fail);
                }

                if visibility_button.clicked() {
                    self.password_visible = !self.password_visible;
                }
                if login_button.clicked() {
                    self.passwords_match = self.verify_master == self.master_password;
                    if self.passwords_match {
                        if String::is_empty(&self.master_password) {
                            return;
                        }
                        match save_master(&self.master_password) {
                            Ok(_) => self.logged_in = true,
                            Err(_) => {
                                self.has_error = Some(true);
                            }
                        }
                    }
                }
            });
    }

    fn show_search(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("Search Panel")
            .frame(Frame {
                inner_margin: Margin::symmetric(30.0, 10.0),
                fill: match self.settings.mode {
                    Mode::Dark => SECONDARY_DARK,
                    Mode::Bright => SECONDARY_BRIGHT,
                },
                stroke: egui::Stroke {
                    width: 1.0,
                    color: match self.settings.mode {
                        Mode::Dark => BORDER_DARK,
                        Mode::Bright => BORDER_BRIGHT,
                    },
                },
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label("Search Panel");
            });
    }

    fn set_toast_time(&mut self) {
        let end = chrono::Local::now().timestamp_millis() + 2000;
        self.toast_end_time = end;
    }

    fn app_text(text: &str, font_size: f32, color: Option<egui::Color32>, mode: Mode) -> RichText {
        match color {
            Some(color) => RichText::new(text).size(font_size).color(color),
            None => RichText::new(text).size(font_size).color(match mode {
                Mode::Bright => DARK_BG,
                Mode::Dark => BRIGHT_BG,
            }),
        }
    }

    fn app_button(text: &str, font_size: f32, color: Option<egui::Color32>, mode: Mode) -> Button {
        match color {
            Some(color) => {
                let stroke = egui::Stroke { width: 1.0, color };
                Button::new(Vault::app_text(text, font_size, Some(color), mode))
                    .stroke(stroke)
                    .fill(egui::Color32::TRANSPARENT)
            }
            None => {
                let stroke = egui::Stroke {
                    width: 1.0,
                    color: DEFAULT_COLOR,
                };
                Button::new(Vault::app_text(text, font_size, Some(DEFAULT_COLOR), mode))
                    .stroke(stroke)
                    .fill(egui::Color32::TRANSPARENT)
            }
        }
    }

    fn line_edit_frame(mode: &Mode) -> egui::Frame {
        egui::Frame::none()
            .fill(match mode {
                Mode::Dark => Color32::BLACK,
                Mode::Bright => Color32::WHITE,
            })
            .stroke(egui::Stroke::new(
                1.0,
                match mode {
                    Mode::Dark => BORDER_DARK,
                    Mode::Bright => BORDER_BRIGHT,
                },
            ))
            .rounding(egui::Rounding::same(CORNER_ROUND))
    }

    fn single_line_edit(
        ui: &mut Ui,
        text: &mut dyn egui::TextBuffer,
        password_visible: Option<bool>,
        mode: &Mode,
    ) -> egui::Response {
        let text = egui::TextEdit::singleline(text)
            .frame(false)
            .password(password_visible.unwrap_or(false))
            .text_color(match mode {
                Mode::Dark => BRIGHT_BG,
                Mode::Bright => DARK_BG,
            })
            .font(egui::TextStyle::Heading)
            .vertical_align(egui::Align::Center);

        ui.add(text)
    }

    fn panel_frame(&self) -> egui::Frame {
        egui::Frame::none()
            .fill(match self.settings.mode {
                Mode::Bright => SECONDARY_BRIGHT,
                Mode::Dark => SECONDARY_DARK,
            })
            .inner_margin(Margin::same(5.0))
            .stroke(egui::Stroke {
                width: 1.0,
                color: match self.settings.mode {
                    Mode::Bright => BORDER_BRIGHT,
                    Mode::Dark => BORDER_DARK,
                },
            })
    }

    fn selectable_text(ui: &mut Ui, text: &str, active: bool, mode: &Mode) -> egui::Response {
        let frame = egui::Frame::none()
            .fill(match mode {
                Mode::Dark => {
                    if active {
                        Color32::BLACK
                    } else {
                        DARK_BG
                    }
                }
                Mode::Bright => {
                    if active {
                        Color32::WHITE
                    } else {
                        BRIGHT_BG
                    }
                }
            })
            .inner_margin(Margin::same(3.0))
            .outer_margin(Margin::symmetric(0.0, 3.0))
            .rounding(egui::Rounding::same(CORNER_ROUND))
            .stroke(egui::Stroke::new(
                if active { 2.0 } else { 0.0 },
                match mode {
                    Mode::Bright => BORDER_BRIGHT,
                    Mode::Dark => BORDER_DARK,
                },
            ));

        let response = frame.show(ui, |f_ui| {
            let text = egui::RichText::new(text)
                .color(match mode {
                    Mode::Dark => BRIGHT_BG,
                    Mode::Bright => DARK_BG,
                })
                .size(18.0);
            let label = egui::Label::new(text)
                .selectable(false)
                .sense(egui::Sense::click());
            f_ui.add(label)
        });

        if response.response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response.inner
    }
}
impl eframe::App for Vault {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.logged_in {
            // if self.toast_end_time < 1 {
            //     self.show_toast(ctx);
            // }
            // self.show_search(ctx);
            self.show_entry_list(ctx);

            egui::CentralPanel::default()
                .frame(Frame {
                    inner_margin: Margin::same(20.0),
                    fill: match self.settings.mode {
                        Mode::Dark => DARK_BG,
                        Mode::Bright => BRIGHT_BG,
                    },
                    stroke: egui::Stroke {
                        width: 1.0,
                        color: match self.settings.mode {
                            Mode::Dark => BORDER_DARK,
                            Mode::Bright => BORDER_BRIGHT,
                        },
                    },
                    ..Default::default()
                })
                .show(ctx, |ui| match self.state {
                    State::DisplayEntry => self.show_entry(ctx, ui),
                    State::EditEntry => self.edit_entry(ctx, ui),
                    State::AddingEntry => self.add_entry(ctx, ui),
                    State::None => {}
                    _ => {
                        eprintln!("State {:#?} not implemented", self.state);
                    }
                });
        } else if self.first_use {
            self.set_master(ctx);
        } else {
            self.authenticate(ctx);
        }
    }
}

#[derive(Default, PartialEq, Debug)]
enum State {
    AddingEntry,
    DisplayEntry,
    EditEntry,
    #[default]
    None,
}
