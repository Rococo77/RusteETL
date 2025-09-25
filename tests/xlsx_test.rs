use ruste_etl::extractors::XlsxExtractor;

#[test]
fn test_xlsx_extract() {
    // create a temporary xlsx file using umya-spreadsheet
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    sheet.get_cell_by_column_and_row_mut(1, 1).set_value("A");
    sheet.get_cell_by_column_and_row_mut(2, 1).set_value("B");

    let tmp = std::env::temp_dir().join(format!(
        "ruste_etl_test_{}.xlsx",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    ));
    umya_spreadsheet::writer::xlsx::write(&book, &tmp).unwrap();

    let extractor = XlsxExtractor {
        path: tmp.to_string_lossy().to_string(),
        sheet: None,
    };
    let data = extractor.extract().unwrap();
    assert!(!data.is_empty());
    assert_eq!(data[0][0], "A");

    let _ = std::fs::remove_file(&tmp);
}
