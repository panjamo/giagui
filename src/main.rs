use arboard::Clipboard;
use eframe::egui;
use std::process::Command;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "GIA GUI",
        options,
        Box::new(|_cc| Ok(Box::new(GiaApp::default()))),
    )
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../icons/gia.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

struct GiaApp {
    prompt: String,
    use_clipboard: bool,
    browser_output: bool,
    resume: bool,
    response: String,
}

impl Default for GiaApp {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            use_clipboard: false,
            browser_output: false,
            resume: false,
            response: String::new(),
        }
    }
}

impl eframe::App for GiaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.ctrl) {
            self.send_prompt();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::L) && i.modifiers.ctrl) {
            self.clear_form();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.ctrl && i.modifiers.shift) {
            self.copy_response();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Prompt input
                ui.label("Prompt:");
                ui.add(
                    egui::TextEdit::multiline(&mut self.prompt)
                        .desired_width(f32::INFINITY)
                        .desired_rows(5),
                );

                ui.add_space(10.0);

                // Options group
                ui.group(|ui| {
                    ui.label("Options");
                    ui.checkbox(&mut self.use_clipboard, "Use clipboard input (-c)");
                    ui.checkbox(
                        &mut self.browser_output,
                        "Browser output (--browser-output)",
                    );
                    ui.checkbox(&mut self.resume, "Resume last conversation (-R)");
                });

                ui.add_space(10.0);

                // Response output
                ui.label("Response:");
                egui::ScrollArea::vertical()
                    .max_height(250.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.response)
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace),
                        );
                    });

                ui.add_space(10.0);

                // Buttons
                ui.horizontal(|ui| {
                    if ui.button("Send (Ctrl+Enter)").clicked() {
                        self.send_prompt();
                    }
                    if ui.button("Clear (Ctrl+L)").clicked() {
                        self.clear_form();
                    }
                    if ui.button("Copy (Ctrl+Shift+C)").clicked() {
                        self.copy_response();
                    }
                });
            });
        });
    }
}

impl GiaApp {
    fn send_prompt(&mut self) {
        let mut args = vec![];

        if self.use_clipboard {
            args.push("-c");
        }
        if self.browser_output {
            args.push("--browser-output");
        }
        if self.resume {
            args.push("-R");
        }

        if !self.prompt.is_empty() {
            args.push(&self.prompt);
        }

        match Command::new("gia").args(&args).output() {
            Ok(output) => {
                self.response = String::from_utf8_lossy(&output.stdout).to_string();
                if !output.stderr.is_empty() {
                    self.response.push_str("\n\nErrors:\n");
                    self.response
                        .push_str(&String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                self.response = format!("Error executing gia: {}", e);
            }
        }
    }

    fn clear_form(&mut self) {
        self.prompt.clear();
        self.response.clear();
        self.use_clipboard = false;
        self.browser_output = false;
        self.resume = false;
    }

    fn copy_response(&mut self) {
        if let Ok(mut clipboard) = Clipboard::new() {
            let _ = clipboard.set_text(&self.response);
        }
    }
}
