use eframe::egui;
use eframe::egui::{Color32, Stroke};
use crate::cpu::CPU;

pub(crate) fn gui(cpu: CPU) -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "VM Control Panel",
        options,
        Box::new(|_cc| Ok(Box::new(VmApp::new(cpu)) as Box<dyn eframe::App>)),
    )
}

const REG_ALIASES: [&str; 32] = [
    "zero",
    "ra",
    "sp",
    "gp",
    "tp",
    "t0",
    "t1",
    "t2",
    "s0",
    "s1",
    "s2",
    "a0",
    "a1",
    "a2",
    "a3",
    "a4",
    "a5",
    "a6",
    "a7",
    "s3",
    "s4",
    "s5",
    "s6",
    "s7",
    "s8",
    "s9",
    "s10",
    "s11",
    "t3",
    "t4",
    "t5",
    "t6"
];

struct VmApp {
    register_aliases: bool,
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
        VmApp { register_aliases: true, active_tab: Tab::Registers, cpu }
    }

    // TODO: Draw grid cell lines.
    // TODO: Add a register dump
    // TODO: Add a register editor, via double-clicking the register
    // TODO: Make font monospace
    // TODO: Add alternate-multiple parallel representations of registers (string, hex, binary)
    fn show_registers(&mut self, ui: &mut egui::Ui) {
        ui.label("Registers:");
        ui.checkbox(&mut self.register_aliases, "Show aliases");

        let line_color = Color32::DARK_GRAY;
        let line_thickness = 1.0;
        let mut row_rects = vec![]; // To track the cell rectangles for drawing lines
        let mut col_rects = vec![]; // To track the cell rectangles for drawing lines

        egui::Grid::new("registers").striped(true).show(ui, |ui| {
            col_rects.push(ui.cursor().left());
            ui.label("Register");
            col_rects.push(ui.cursor().left());
            ui.label("Value");
            col_rects.push(ui.cursor().left());
            ui.label("ASCII");
            col_rects.push(ui.cursor().left());
            ui.label("Decimal");
            col_rects.push(ui.cursor().left());
            ui.end_row();
            for i in 0..32 {
                let row_start = ui.cursor();
                if self.register_aliases {
                    ui.label(format!(" {} ", REG_ALIASES[i]));
                } else {
                    ui.label(format!(" x{} ", i));
                }
                let value = format!(" 0x{:0>4} ", self.cpu.registers.get_register(i as u8));
                let ascii = format!(" {} ", self.cpu.registers.get_register(i as u8) as u8 as char);
                let dec = format!(" {} ", self.cpu.registers.get_register(i as u8) as u8);
                ui.label(value);
                ui.label(ascii);
                ui.label(dec);
                row_rects.push(row_start);
                ui.end_row();
            }
        });

        // Draw grid lines
        let painter = ui.painter();
        for rect in &row_rects {
            // Draw horizontal line at the top of each row
            /*painter.line_segment(
                [
                    egui::pos2(rect.left(), rect.top()),
                    egui::pos2(rect.right(), rect.top()),
                ],
                Stroke::new(line_thickness, line_color),
            );*/

            // Draw vertical lines for columns
            let column_positions = [
                rect.left(),
                rect.right(),
            ];
            for &x in &col_rects {
                painter.line_segment(
                    [
                        egui::pos2(x, rect.top()),
                        egui::pos2(x, rect.bottom()),
                    ],
                    Stroke::new(line_thickness, line_color),
                );
            }
        }

        // Draw border around the entire grid
        //let grid_bounds = ui.min_rect();
        //painter.rect_stroke(grid_bounds, 0.0, Stroke::new(line_thickness, line_color));
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
                            for mem_page in chunk {
                                mem_page.get_page().iter().for_each(|byte| {
                                    ui.label(format!("{:02X} ", byte));
                                });
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
