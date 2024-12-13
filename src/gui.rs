use eframe::egui;

use cpu::CPU;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "VM Control Panel",
        options,
        Box::new(|_cc| Box::new(VmApp::default())),
    )
}

struct VmApp {
    cpu: cpu::CPU,
}

impl Default for VmApp {
    fn default(cpu: cpu::CPU) -> Self {
        this.cpu = cpu;
    }
}

impl VmApp {
    fn show_registers(&self, ui: &mut egui::Ui) {
        ui.label("Registers:");
        for (reg, value) in &self.cpu.reg{
            ui.horizontal(|ui| {
                ui.label(format!("{}: ", reg));
                ui.label(format!("{}", value));
            });
        }
    }

    fn show_memory(&mut self, ui: &mut egui::Ui) {
        ui.label("Memory:");
        for (i, chunk) in self.memory.chunks(16).enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{:04X}: ", i * 16));
                for byte in chunk {
                    ui.label(format!("{:02X} ", byte));
                }
            });
        }
    }
}

impl eframe::App for VmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("VM Control Panel");
            });

            egui::TopBottomPanel::top("tabs_panel").show_inside(ui, |ui| {
                if ui.button("Registers").clicked() {
                    ui.heading("Registers");
                    self.show_registers(ui);
                }
                if ui.button("Memory").clicked() {
                    ui.heading("Memory");
                    self.show_memory(ui);
                }
            });
        });
    }
}
