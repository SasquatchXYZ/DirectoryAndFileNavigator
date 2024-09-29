use std::{
    env::current_dir,
    fs::read_dir,
    path::PathBuf,
};
use eframe::egui;
use egui::{Color32, RichText};

struct DirectoryApp {
    current_directory: PathBuf,
}

impl DirectoryApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            current_directory: current_dir().unwrap(),
        }
    }
}

impl eframe::App for DirectoryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button(" .. ").clicked() {
                self.current_directory.pop();
            }
            let read_dir = read_dir(&self.current_directory).unwrap();
            for entry in read_dir.flatten() {
                let metadata = entry.metadata().unwrap();
                let name = entry.file_name().into_string().unwrap();
                if metadata.is_dir() {
                    if ui
                        .button(RichText::new(&name).color(Color32::GRAY))
                        .clicked() {
                        self.current_directory.push(&name);
                    }
                } else if metadata.is_file() {
                    if ui
                        .button(RichText::new(&name).color(Color32::GOLD))
                        .clicked() {}
                } else {
                    ui.label(name);
                }
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "File Explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(DirectoryApp::new(cc)))),
    );
}
