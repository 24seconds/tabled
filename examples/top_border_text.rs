//! The example can be run by this command
//! `cargo run --example width`

use tabled::{style::TopBorderText, Alignment, Full, Modify, Style, Table};

fn main() {
    let data = [[5, 6, 7, 8, 9], [10, 11, 12, 13, 14]];

    let table = Table::new(&data)
        .with(Style::modern().horizontal_off())
        .with(TopBorderText::new("┌ Columns "))
        .with(Modify::new(Full).with(Alignment::left()));

    println!("{}", table);
}
