use arboard::Clipboard;
use eframe::egui;
use std::process::Command;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
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
    options: String,
    use_clipboard: bool,
    browser_output: bool,
    resume: bool,
    response: String,
    first_frame: bool,
}

impl Default for GiaApp {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            options: String::new(),
            use_clipboard: false,
            browser_output: false,
            resume: false,
            response: String::new(),
            first_frame: true,
        }
    }
}

impl eframe::App for GiaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.ctrl) {
            self.send_prompt();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::R) && i.modifiers.ctrl) {
            self.send_prompt_with_audio();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::L) && i.modifiers.ctrl) {
            self.clear_form();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.ctrl && i.modifiers.shift) {
            self.copy_response();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::O) && i.modifiers.ctrl) {
            self.show_conversation();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.show_help();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // Prompt input
                ui.label("Prompt:");
                let prompt_response = ui.add(
                    egui::TextEdit::multiline(&mut self.prompt)
                        .desired_width(f32::INFINITY)
                        .desired_rows(5),
                );

                // Request focus on first frame
                if self.first_frame {
                    prompt_response.request_focus();
                    self.first_frame = false;
                }

                ui.add_space(10.0);

                // Options group and custom options side by side
                ui.horizontal(|ui| {
                    // Checkboxes
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Options");
                            ui.checkbox(&mut self.use_clipboard, "Use clipboard input (-c)");
                            ui.checkbox(
                                &mut self.browser_output,
                                "Browser output (--browser-output)",
                            );
                            ui.checkbox(&mut self.resume, "Resume last conversation (-R)");
                        });
                    });

                    // GIA logo
                    let logo_bytes = include_bytes!("../icons/gia.png");
                    if let Ok(image) = image::load_from_memory(logo_bytes) {
                        let size = [80.0, 80.0];
                        let image =
                            image.resize_exact(80, 80, image::imageops::FilterType::Lanczos3);
                        let rgba_image = image.to_rgba8();
                        let pixels = rgba_image.as_flat_samples();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [size[0] as usize, size[1] as usize],
                            pixels.as_slice(),
                        );
                        let texture = ui.ctx().load_texture(
                            "gia_logo",
                            color_image,
                            egui::TextureOptions::LINEAR,
                        );
                        ui.add_space(10.0);
                        ui.image(&texture);
                        ui.add_space(10.0);
                    }

                    // Custom options input
                    ui.vertical(|ui| {
                        ui.label("Options:");
                        ui.add(
                            egui::TextEdit::multiline(&mut self.options)
                                .desired_width(f32::INFINITY)
                                .desired_rows(4),
                        );
                    });
                });

                ui.add_space(10.0);

                // Buttons
                ui.horizontal(|ui| {
                    if ui.button("Send (Ctrl+Enter)").clicked() {
                        self.send_prompt();
                    }
                    if ui.button("Record (Ctrl+R)").clicked() {
                        self.send_prompt_with_audio();
                    }
                    if ui.button("Clear (Ctrl+L)").clicked() {
                        self.clear_form();
                    }
                    if ui.button("Copy (Ctrl+Shift+C)").clicked() {
                        self.copy_response();
                    }
                    if ui.button("Conversation (Ctrl+O)").clicked() {
                        self.show_conversation();
                    }
                    if ui.button("Help (F1)").clicked() {
                        self.show_help();
                    }
                });

                ui.add_space(5.0);

                // Response output
                ui.label("Response:");

                ui.add_space(5.0);

                // Response box - use remaining space
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.response)
                            .font(egui::TextStyle::Monospace),
                    );
                });
            });
        });
    }
}

impl GiaApp {
    fn send_prompt(&mut self) {
        self.execute_gia(false);
    }

    fn send_prompt_with_audio(&mut self) {
        self.execute_gia(true);
    }

    fn execute_gia(&mut self, with_audio: bool) {
        let mut args = vec![];

        if with_audio {
            args.push("--record-audio".to_string());
        }
        if self.use_clipboard {
            args.push("-c".to_string());
        }
        if self.browser_output {
            args.push("--browser-output".to_string());
        }
        if self.resume {
            args.push("-R".to_string());
        }

        // Add custom options from options field
        for line in self.options.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                args.push(trimmed.to_string());
            }
        }

        if !self.prompt.is_empty() {
            args.push(self.prompt.clone());
        }

        match Command::new("gia").args(args).output() {
            Ok(output) => {
                self.response = String::from_utf8_lossy(&output.stdout).to_string();
                if !output.stderr.is_empty() {
                    self.response.push_str("\n\nErrors:\n");
                    self.response
                        .push_str(&String::from_utf8_lossy(&output.stderr));
                }
                // Auto-enable Resume after successful execution
                self.resume = true;
            }
            Err(e) => {
                self.response = format!("Error executing gia: {}", e);
            }
        }
    }

    fn clear_form(&mut self) {
        self.prompt.clear();
        self.options.clear();
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

    fn show_conversation(&mut self) {
        let _ = Command::new("gia")
            .args(&["--browser-output", "--show-conversation"])
            .spawn();
    }

    fn show_help(&mut self) {
        match Command::new("gia").arg("--help").output() {
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
}
