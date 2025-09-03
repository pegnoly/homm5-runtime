use std::path::PathBuf;

use calamine::{open_workbook, Reader, Xlsx};

pub fn read_xlsx_file() -> Result<(), crate::error::Error> {
    let mut xlsx: Xlsx<_> = open_workbook("D:\\1.xlsx")?;
    if let Ok(r) = xlsx.worksheet_range("Общая статистика по расам") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    } 
    Ok(())
}