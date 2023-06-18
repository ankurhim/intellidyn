use chrono::{Local, NaiveDate};
use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Image, Workbook, XlsxError};

pub fn mmc_file() -> Result<(), XlsxError> {
// Create a new Excel file object.
let mut workbook = Workbook::new();
let worksheet = workbook.add_worksheet();

// Write some merged cells with centering.
let format_1 = Format::new()
.set_align(FormatAlign::VerticalCenter)
.set_align(FormatAlign::Center)
.set_border(FormatBorder::Thin)
.set_bold()
.set_font_size(16);

worksheet.merge_range(0, 1, 3, 6, "Mass Wire and Steels Pvt. Ltd., Bhiwadi", &format_1)?;

let format_2 = Format::new()
.set_align(FormatAlign::VerticalCenter)
.set_border(FormatBorder::Thin);

worksheet.write_with_format(0, 7, "Doc. No.", &format_2)?.autofit();
worksheet.write_with_format(1, 7, "Issue Date", &format_2)?.autofit();
worksheet.write_with_format(2, 7, "Rev. No.", &format_2)?.autofit();
worksheet.write_with_format(3, 7, "Rev Date", &format_2)?.autofit();

worksheet.write_with_format(0, 8, "F/STR/21", &format_2)?.autofit();
worksheet.write_with_format(1, 8, "01-04-2014", &format_2)?.autofit();
worksheet.write_with_format(2, 8, "02", &format_2)?.autofit();
worksheet.write_with_format(3, 8, "15-05-2023", &format_2)?.autofit();

worksheet.merge_range(4, 0, 5, 0, "Sr. No.", &format_2)?;
worksheet.merge_range(4, 1, 5, 5, "Cutting Requisition Slip Cum Cutting Approval", &format_2)?;

let format_3 = Format::new()
.set_num_format("dd-mm-yyyy")
.set_align(FormatAlign::VerticalCenter)
.set_border(FormatBorder::Thin);
worksheet.write_with_format(4, 6, "Date", &format_2)?;
worksheet.write_date(5, 6, &Local::now().date_naive(), &format_3)?.autofit();

worksheet.merge_range(4, 7, 5, 8, "Remarks (if any)", &format_2)?;

worksheet.set_column_width(1, 20)?;
worksheet.set_column_width(2, 20)?;
worksheet.set_column_width(3, 20)?;
worksheet.set_column_width(4, 20)?;
worksheet.set_column_width(5, 20)?;
worksheet.set_column_width(6, 20)?;

worksheet.write_with_format(6, 0, "1", &format_2)?;
worksheet.write_with_format(7, 0, "2", &format_2)?;
worksheet.write_with_format(8, 0, "3", &format_2)?;
worksheet.write_with_format(9, 0, "4", &format_2)?;
worksheet.write_with_format(10, 0, "5", &format_2)?;
worksheet.write_with_format(11, 0, "6", &format_2)?;
worksheet.write_with_format(12, 0, "7", &format_2)?;
worksheet.write_with_format(13, 0, "8", &format_2)?;
worksheet.write_with_format(14, 0, "9", &format_2)?;
worksheet.write_with_format(15, 0, "10", &format_2)?;
worksheet.write_with_format(16, 0, "11", &format_2)?;
worksheet.write_with_format(17, 0, "12", &format_2)?;

worksheet.write_with_format(6, 1, "Part No.", &format_2)?;
worksheet.write_with_format(7, 1, "Grade", &format_2)?;
worksheet.write_with_format(8, 1, "Heat No./ Heat Code", &format_2)?;
worksheet.write_with_format(9, 1, "Section Size (mm)", &format_2)?;
worksheet.write_with_format(10, 1, "Part New/ Regular", &format_2)?;
worksheet.write_with_format(11, 1, "Cut Length (mm)/ Weight (Kgs)", &format_2)?;
worksheet.write_with_format(12, 1, "Color Code (As per color coding chart WI/LAB/22)", &format_2)?;
worksheet.write_with_format(13, 1, "Planning Cutting Qty (Nos.)", &format_2)?;
worksheet.write_with_format(14, 1, "Actual Qty (Nos.)", &format_2)?;
worksheet.write_with_format(15, 1, "Total Weight (MT)", &format_2)?;
worksheet.write_with_format(16, 1, "Sample Size", &format_2)?;
worksheet.write_with_format(17, 1, "Visual (No Burr, No Taper, No chips)", &format_2)?;

worksheet.merge_range(6, 2, 6, 6, "", &format_2)?;
worksheet.merge_range(7, 2, 7, 6, "", &format_2)?;
worksheet.merge_range(8, 2, 8, 6, "", &format_2)?;
worksheet.merge_range(9, 2, 9, 6, "", &format_2)?;
worksheet.merge_range(10, 2, 10, 6, "", &format_2)?;
worksheet.merge_range(11, 2, 11, 6, "", &format_2)?;
worksheet.merge_range(12, 2, 12, 6, "", &format_2)?;
worksheet.merge_range(13, 2, 13, 6, "", &format_2)?;
worksheet.merge_range(14, 2, 14, 6, "", &format_2)?;
worksheet.merge_range(15, 2, 15, 6, "", &format_2)?;


// Save the file to disk.
workbook.save("merge_range.xlsx")?;

Ok(())
}