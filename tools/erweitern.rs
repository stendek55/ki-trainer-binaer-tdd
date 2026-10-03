use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const GRID_SIZE: usize = 16;
const TOTAL_PIXELS: usize = GRID_SIZE * GRID_SIZE; // 256

const SPEICHER_PFAD: &str = "datas/mehr_zahlen.csv";
const LADE_PAD: &str = "datas/zahlen_hand.csv";

struct DataRow {
    label: u8,
    grid_data: Vec<u8>,
}

fn main() {
    println!("#######-----DATA AUGMENTATION GESTARTET-----#######");

    // 1. Quelldaten einlesen
    let dataset = load_csv(LADE_PAD);
    if dataset.is_empty() {
        println!("Fehler: 'zahlen_hand.csv' konnte nicht geöffnet werden oder ist leer.");
        return;
    }
    println!("-> {} Ursprungsmuster geladen.", dataset.len());

    // 2. Neue Ausgabedatei vorbereiten
    let write_file = File::create(SPEICHER_PFAD).expect("Kann Ausgabedatei nicht erstellen");
    let mut writer = BufWriter::new(write_file);

    let mut count_original = 0;
    let mut count_augmented = 0;

    // 3. Transformation und Generierung
    for row in &dataset {
        // Schritt A: Das unveränderte Original in die Datei schreiben
        write_row(&mut writer, row);
        count_original += 1;

        // Schritt B: Die Erweiterung (Dilatation) berechnen
        let augmented_row = dilate_left_and_up(row);
        write_row(&mut writer, &augmented_row);
        count_augmented += 1;
        // nächste Erweiterung (Dilatation) berechnen
        let augmented_row = dilate_rechts_unten(row);
        write_row(&mut writer, &augmented_row);
        count_augmented += 1;
        // nächste Erweiterung (Dilatation) berechnen
        let augmented_row = dilate_split_axis(row);
        write_row(&mut writer, &augmented_row);
        count_augmented += 1;
        // nächste Erweiterung (Dilatation) berechnen
        let augmented_row = dilate_zoom_outward_pure(row);
        write_row(&mut writer, &augmented_row);
        count_augmented += 1;
        //nächste erweiterungen
        //-> zahl wird jeweils x pixel in gesetzte richtung verschoben
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        for &dir in &directions {
            // try_translate liefert nur dann Daten, wenn nichts abgeschnitten wird
            for x in 1..=2 {
                if let Some(augmented_row) = try_translate(row, dir, x) {
                    write_row(&mut writer, &augmented_row);
                    count_augmented += 1;
                }
            }
        }
    }

    // Speicher-Buffer physisch auf die Festplatte schreiben
    writer.flush().unwrap();

    println!("============================================================");
    println!("  Erweiterung erfolgreich abgeschlossen!");
    println!("  - Originale exportiert:         {}", count_original);
    println!("  - Erweiterte Muster exportiert: {}", count_augmented);
    println!("     |---> dicker->plus pixel->links-oben");
    println!("     |---> dicker->plus pixel->rechts-unten");
    println!("     |---> dicker->plus pixel->schräg");
    println!("     |---> dicker->neue pixel->zoom-gross");
    println!("     |---> verschiebung -> jeweils alle vier richtungen");
    println!("              |---> 1 und 2 pixel");
    println!("  ----------------------------------------------------------");
    println!(
        "  Gesamte Zeilen in neuer CSV:    {}",
        count_original + count_augmented
    );
    println!("============================================================");
}

/// Transformiert das Muster, indem für jedes aktive Pixel (1)
/// zusätzlich das linke und das obere Nachbarpixel auf 1 gesetzt werden.
fn dilate_left_and_up(source: &DataRow) -> DataRow {
    // Wir klonen das originale Gitter als Basis, damit bestehende Pixel erhalten bleiben
    let mut new_grid = source.grid_data.clone();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let current_idx = y * GRID_SIZE + x;

            // Prüfen, ob das aktuelle Pixel im Original-Datensatz aktiv (1) war
            if source.grid_data[current_idx] == 1 {
                // 1. Pixel LINKS dazusetzen (falls wir nicht am linken Rand x == 0 sind)
                if x > 0 {
                    let left_idx = y * GRID_SIZE + (x - 1);
                    new_grid[left_idx] = 1;
                }

                // 2. Pixel DARÜBER dazusetzen (falls wir nicht am oberen Rand y == 0 sind)
                if y > 0 {
                    let up_idx = (y - 1) * GRID_SIZE + x;
                    new_grid[up_idx] = 1;
                }
            }
        }
    }

    DataRow {
        label: source.label,
        grid_data: new_grid,
    }
}

/// Transformiert das Muster, indem für jedes aktive Pixel (1)
/// zusätzlich das rechte und das untere Nachbarpixel auf 1 gesetzt werden.
fn dilate_rechts_unten(source: &DataRow) -> DataRow {
    // Wir klonen das originale Gitter als Basis, damit bestehende Pixel erhalten bleiben
    let mut new_grid = source.grid_data.clone();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let current_idx = y * GRID_SIZE + x;

            // Prüfen, ob das aktuelle Pixel im Original-Datensatz aktiv (1) war
            if source.grid_data[current_idx] == 1 {
                // 1. Pixel RECHTS dazusetzen (falls wir nicht am rechten Rand x == GRID_SIZE sind)
                if x < GRID_SIZE - 1 {
                    let rechts_idx = y * GRID_SIZE + (x + 1);
                    new_grid[rechts_idx] = 1;
                }

                // 2. Pixel DRUNTER dazusetzen (falls wir nicht am unteren Rand y == GRID_SIZE sind)
                if y < GRID_SIZE - 1 {
                    let unten_idx = (y + 1) * GRID_SIZE + x;
                    new_grid[unten_idx] = 1;
                }
            }
        }
    }

    DataRow {
        label: source.label,
        grid_data: new_grid,
    }
}

/// Transformiert das Muster, indem für jedes aktive Pixel (1)
/// in oberer hälfte -> zusätzlich das rechte und das obere Nachbarpixel auf 1 gesetzt werden.
/// in unterer hälfte -> zusätzlich das linke und das untere Nachbarpixel auf 1 gesetzt werden.
fn dilate_split_axis(source: &DataRow) -> DataRow {
    let mut new_grid = source.grid_data.clone();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let current_idx = y * GRID_SIZE + x;

            // 1. Früher Abbruch (Early Exit) für inaktive Pixel
            if source.grid_data[current_idx] != 1 {
                continue;
            }

            // 2. Logik-Splittung anhand der Y-Achse
            if y < GRID_SIZE / 2 {
                // Obere Hälfte: Aufdickung nach Oben-Rechts
                if x < GRID_SIZE - 1 {
                    new_grid[y * GRID_SIZE + (x + 1)] = 1;
                }
                if y > 0 {
                    new_grid[(y - 1) * GRID_SIZE + x] = 1;
                }
            } else {
                // Untere Hälfte: Aufdickung nach Unten-Links
                if x > 0 {
                    new_grid[y * GRID_SIZE + (x - 1)] = 1;
                }
                if y < GRID_SIZE - 1 {
                    new_grid[(y + 1) * GRID_SIZE + x] = 1;
                }
            }
        }
    }

    DataRow {
        label: source.label,
        grid_data: new_grid,
    }
}

/// Erzeugt einen nahtlosen Zoom nach außen, schließt die Achsen-Lücken
/// und sichert die Pixel im absoluten Zentrum des 16x16-Rasters.
fn dilate_zoom_outward_pure(source: &DataRow) -> DataRow {
    let mut new_grid = vec![0; TOTAL_PIXELS];
    let mitte = GRID_SIZE / 2; // Grenze zwischen Index 7 und 8

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let current_idx = y * GRID_SIZE + x;

            if source.grid_data[current_idx] != 1 {
                continue;
            }

            // Standard-Richtungsvektor nach außen
            let dx = if x < mitte { -1 } else { 1 };
            let dy = if y < mitte { -1 } else { 1 };

            // Zielkoordinate für den normalen Zoom-Schritt
            let target_x = x as i32 + dx;
            let target_y = y as i32 + dy;

            // 1. Den normalen verschobenen Pixel setzen (mit Randschutz)
            if target_x >= 0
                && target_x < GRID_SIZE as i32
                && target_y >= 0
                && target_y < GRID_SIZE as i32
            {
                let target_idx = (target_y as usize) * GRID_SIZE + (target_x as usize);
                new_grid[target_idx] = 1;
            }

            // 2. KORREKTUR FÜR DIE ACHSEN (Verhindert das Aufreißen der Linien)
            let ist_an_x_mitte = x == mitte - 1 || x == mitte;
            let ist_an_y_mitte = y == mitte - 1 || y == mitte;

            // Wenn das Pixel an der vertikalen Mitte liegt -> X fixieren, Y verschieben
            if ist_an_x_mitte {
                let fix_x_target_y = y as i32 + dy;
                if fix_x_target_y >= 0 && fix_x_target_y < GRID_SIZE as i32 {
                    new_grid[(fix_x_target_y as usize) * GRID_SIZE + x] = 1;
                }
            }

            // Wenn das Pixel an der horizontalen Mitte liegt -> Y fixieren, X verschieben
            if ist_an_y_mitte {
                let fix_y_target_x = x as i32 + dx;
                if fix_y_target_x >= 0 && fix_y_target_x < GRID_SIZE as i32 {
                    new_grid[(y * GRID_SIZE) + (fix_y_target_x as usize)] = 1;
                }
            }

            // 3. NEU: ZENTRUMS-SICHERUNG
            // Wenn das Pixel im absoluten 2x2-Zentrum liegt, darf es nicht komplett
            // wegwandern. Wir halten es auf seiner Ursprungsposition fest.
            if ist_an_x_mitte && ist_an_y_mitte {
                new_grid[current_idx] = 1;
            }
        }
    }

    DataRow {
        label: source.label,
        grid_data: new_grid,
    }
}

/// Schreibt eine 'DataRow'-Struktur im schnellen, sequentiellen CSV-Format in den Stream
fn write_row(writer: &mut BufWriter<File>, row: &DataRow) {
    write!(writer, "{}", row.label).unwrap();
    for pixel in &row.grid_data {
        write!(writer, ",{}", pixel).unwrap();
    }
    writeln!(writer).unwrap();
}

/// Optimierter CSV-Reader aus den vorherigen Schritten
fn load_csv(path: &str) -> Vec<DataRow> {
    let file = if let Ok(f) = File::open(path) {
        f
    } else {
        return Vec::new();
    };
    let reader = BufReader::new(file);
    let mut dataset = Vec::new();

    for line_text in reader.lines().map_while(Result::ok) {
        let trimmed = line_text.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parts: Vec<_> = trimmed.split(',').collect();

        if parts.len() == TOTAL_PIXELS + 1 {
            let label = parts[0].parse().unwrap_or(0);
            let grid_data = parts[1..].iter().map(|p| p.parse().unwrap_or(0)).collect();

            dataset.push(DataRow { label, grid_data });
        }
    }
    dataset
}

//zur richtungsangabe im translate benutzen
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
//verschiebt alle pixel in gesetzte richtung wenn diese nicht am rand sind
fn try_translate(source: &DataRow, direction: Direction, pixel: i32) -> Option<DataRow> {
    let mut new_grid = vec![0; TOTAL_PIXELS];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if source.grid_data[y * GRID_SIZE + x] == 1 {
                // 1. Zielkoordinaten direkt als i32 berechnen
                let (tx, ty) = match direction {
                    Direction::Up => (x as i32, y as i32 - pixel),
                    Direction::Down => (x as i32, y as i32 + pixel),
                    Direction::Left => (x as i32 - pixel, y as i32),
                    Direction::Right => (x as i32 + pixel, y as i32),
                };

                // 2. Randschutz: Sobald ein Pixel rausfliegt -> Sofortiger Abbruch!
                if tx < 0 || tx >= GRID_SIZE as i32 || ty < 0 || ty >= GRID_SIZE as i32 {
                    return None;
                }

                // 3. Wenn sicher, im neuen Grid platzieren
                new_grid[(ty as usize) * GRID_SIZE + (tx as usize)] = 1;
            }
        }
    }

    Some(DataRow {
        label: source.label,
        grid_data: new_grid,
    })
}
