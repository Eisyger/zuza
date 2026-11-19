use std::fs;
use std::fs::File;
use std::io::Write;

pub fn read_input(path: &str) -> Vec<String> {
    let bytes = fs::read(path).expect("Datei konnte nicht gelesen werden");
    String::from_utf16le(&bytes)
        .expect("Ungültiges UTF-16 LE")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

pub fn write_output(lines: Vec<String>, path: &str) -> std::io::Result<()> {
    File::create(path)?.write_all(lines.join("\n").as_bytes())?;
    Ok(())
}

pub fn complement(mut lines: Vec<String>) -> Vec<String> {
    let mut i = 0;
    let mut zusatztext = String::new();
    while i < lines.len() {
        let cells: Vec<&str> = lines[i].split(';').collect();
        if cells.len() < 2 {
            i += 1;
            continue;
        }
        match lines[i].split(' ').next().unwrap_or("").trim_matches('"') {
            "Meldergruppe" => {
                zusatztext = cells[1].to_string();
            }
            "Melder" => {
                let mut new_line = Vec::<&str>::new();
                for (idx, value) in cells.iter().enumerate() {
                    if idx == 1 {
                        new_line.push(&zusatztext);
                    } else {
                        new_line.push(*value);
                    }
                }
                lines[i] = new_line.join(";");
            }
            _ => {}
        }
        i += 1;
    }
    lines
}
