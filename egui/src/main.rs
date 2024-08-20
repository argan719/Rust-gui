#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use calamine::{Reader, Xlsx, DataType};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use umya_spreadsheet::writer;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let mut name = "Arthur".to_owned();
    let mut excel_data = String::new();
    let mut tc_code: Vec<(usize, String)> = Vec::new(); // Type annotation added here

    eframe::run_simple_native("TSH", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Translate SILS to HILS");
            ui.horizontal(|ui| {
                let name_label = ui.label("File name: ");
                ui.text_edit_singleline(&mut name).labelled_by(name_label.id);
            });


            
            // C# TC import
            if ui.button("Load C# File").clicked() {
                // 경로 지정
                let file_path = "/mnt/c/Users/sure/Desktop/TC/SeatHeatVent_ThirdRL.cs";

                // C# 파일 읽어오기
                if let Ok(lines) = read_lines(file_path){
                    tc_code.clear(); // 이전 데이터를 지우기
                    for(index, line) in lines.enumerate(){
                        if let Ok(code) = line {
                            tc_code.push((index + 1, code));  // 튜플 형태로 저장
                        }
                    }
                } else {
                    println!("Failed to read the TC file.");
                }
            }


            // Export to Excel
            // if ui.button("Export to Excel File").clicked() {
            //     let mut book = umya_spreadsheet::new_file();
            //     let sheet_name = "HILS_TC";
            //     book.new_sheet(sheet_name).unwrap();

            //     for (line_number, code) in &tc_code {
            //         let row = (*line_number as u32) + 1; // 1-based index
            //         let col = 1; // First column
            //         book.get_sheet_by_name_mut(sheet_name)
            //             .unwrap()
            //             .get_cell_mut((row, col))
            //             .set_value(code);
            //     }

            //     let file_path = "/mnt/c/Users/sure/Desktop/HILSTCTransformer/ExportedReport.xlsx";
            //     writer::xlsx::write(&book, file_path).unwrap();
            // }

            // ui.separator();  // Add a separator line
            // ui.label("C# code:");
            // for (line_number, code) in &tc_code {
            //     ui.monospace(format!("{}: {}", line_number, code));  // Display line number and code
            // }

        });


    })
}
 


// 파일을 한 줄씩 읽는 헬퍼 함수
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}