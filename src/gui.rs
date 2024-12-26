use eframe::egui;
use crate::cpu::CPU;

pub(crate) fn gui(cpu: CPU) -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "VM Control Panel",
        options,
        Box::new(|_cc| Ok(Box::new(VmApp::new(cpu)) as Box<dyn eframe::App>)),
    )
}

struct VmApp {
    active_tab: Tab,
    cpu: CPU,
}

// Define an enum to represent the tabs
#[derive(PartialEq)]
enum Tab {
    Registers,
    Memory,
}


impl VmApp {

    pub fn new(cpu: CPU) -> Self{
        VmApp { cpu, active_tab: Tab::Registers }
    }

    // TODO: Make this a table
    // TODO: Add a register dump
    // TODO: Add a register editor, via double-clicking the register
    // TODO: Add common register names
    // TODO: Make font monospace
    // TODO: Add alternate-multiple parallel representations of registers (string, hex, binary)
    fn show_registers(&self, ui: &mut egui::Ui) {
        ui.label("Registers:");
        for i in 0..32 {
            ui.horizontal(|ui| {
                ui.label(format!("x{}: ", i));
                ui.label(format!("{}", self.cpu.registers.get_register(i)));
            });
        }
    }

    // TODO: Add alternate-multiple parallel representations of memory (string, hex, binary)
    // TODO: Make memory view into a table ?
    // TODO: Add memory dump
    // TODO: Add memory editor
    // TODO: Add memory search
    // TODO: Make group size configurable (currently one byte)
    // TODO: Make groups per row (chunk size) configurable
    // TODO: Make font monospace
    fn show_memory(&mut self, ui: &mut egui::Ui) {
        // Memory page size
        const PAGE_SIZE: usize = 16;

        // Total number of pages
        let total_pages = self.cpu.memory.get_memory().len() / PAGE_SIZE;

        // Height of one memory row in pixels
        let row_height = 18.0;

        // Dynamically render memory using a scroll area
        egui::ScrollArea::vertical()
            .show_rows(ui, row_height, total_pages, |ui, range| {
                for page in range {
                    let start = page * PAGE_SIZE;
                    let end = start + PAGE_SIZE;

                    // Safely get the memory chunk for the current page
                    if let Some(chunk) = self.cpu.memory.get_memory().get(start..end) {
                        ui.horizontal(|ui| {
                            ui.label(format!("{:04X}: ", start));
                            for byte in chunk {
                                ui.label(format!("{:02X} ", byte));
                            }
                        });
                    }
                }
            });
    }
}

impl eframe::App for VmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("VM Control Panel");
            });

            egui::TopBottomPanel::top("tabs_panel").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Registers").clicked() {
                        self.active_tab = Tab::Registers;
                    }
                    if ui.button("Memory").clicked() {
                        self.active_tab = Tab::Memory;
                    }
                });
            });

            match self.active_tab {
                Tab::Registers => {
                    ui.heading("Registers");
                    self.show_registers(ui);
                }
                Tab::Memory => {
                    ui.heading("Memory");
                    self.show_memory(ui);
                }
            }
        });
    }
}
