use crate::instruments::get_assets::{GetAssetsResponse, Instrument};
use std::error::Error;
use tracing::info;

pub struct Bot {}

impl Bot {
    pub async fn filter_instruments(
        response: GetAssetsResponse,
        class_code: &str,
        instrument_type: &str,
    ) -> Result<Vec<Instrument>, Box<dyn Error>> {
        let mut filtered_instruments = Vec::new();

        for asset in response.assets {
            for instrument in asset.instruments {
                if instrument.class_code == class_code
                    && instrument.instrument_type == instrument_type
                {
                    filtered_instruments.push(instrument);
                }
            }
        }

        info!(
            "filtered instruments: {} with class_code: '{}' and instrument_type: '{}'",
            filtered_instruments.len(),
            class_code,
            instrument_type
        );

        filtered_instruments.sort_by(|a, b| a.ticker.cmp(&b.ticker));

        Ok(filtered_instruments)
    }

    /// Выводит информацию об инструментах в виде таблицы
    pub fn print_instruments(instruments: &[Instrument]) {
        if instruments.is_empty() {
            info!("there are no tools to display");
            return;
        }

        // Определяем ширину для каждой колонки (включая пробелы с обеих сторон)
        let col_widths = [
            ("UID", 38),             // 36 + 2 пробела
            ("TICKER", 12),          // максимальная длина тикера + 2
            ("CLASS_CODE", 14),      // длина + 2
            ("FIGI", 14),            // длина + 2
            ("POSITION_UID", 38),    // 36 + 2 пробела
            ("INSTRUMENT_TYPE", 17), // длина + 2
        ];

        print_table_separator(&col_widths);
        print_table_header(&col_widths);
        print_table_separator(&col_widths);

        for instrument in instruments {
            print_table_row(
                &[
                    &instrument.uid,
                    &instrument.ticker,
                    &instrument.class_code,
                    &instrument.figi,
                    &instrument.position_uid,
                    &instrument.instrument_type,
                ],
                &col_widths,
            );
            print_table_separator(&col_widths);
        }
    }
}

fn print_table_row(data: &[&str], cols: &[(&str, usize)]) {
    print!("|");
    for (value, (_, width)) in data.iter().zip(cols.iter()) {
        let space_width = width - 2; // Учитываем отступы с обеих сторон
        if value.len() > space_width {
            // Обрезаем строку и добавляем ...
            print!(" {:.width$}... |", value, width = space_width - 3);
        } else {
            // Выводим значение с выравниванием по левому краю
            print!(" {:<width$} |", value, width = space_width);
        }
    }
    println!();
}

fn print_table_header(cols: &[(&str, usize)]) {
    print!("|");
    for (title, width) in cols {
        print!(" {:<width$} |", title, width = width - 2);
    }
    println!();
}

fn print_table_separator(cols: &[(&str, usize)]) {
    print!("+");
    for (_, width) in cols {
        print!("{:-<width$}+", "", width = width);
    }
    println!();
}
