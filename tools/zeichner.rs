use minifb::Key;
use std::fs::OpenOptions;
use std::io::Write;

// gui.rs aus der Nachbarschaft einbinden
mod gui;
use gui::starte_gitter_gui;

const SPEICHER_PFAD: &str = "datas/zahlen_hand.csv";

fn main() {
    println!("Daten-Generator im Tool-Ordner aktiv. Tasten [0] oder [1] drücken.");

    starte_gitter_gui(
        "zeichner | [0] o [1] -> speichert",
        |window, grid_data, _leeren| {
            let mut klasse_to_save = None;

            if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
                klasse_to_save = Some(0);
            } else if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
                klasse_to_save = Some(1);
            }

            if let Some(label) = klasse_to_save {
                save_to_csv(label, grid_data);
                grid_data.fill(0); // Nach Speichern Leinwand säubern
                println!("-> Muster erfolgreich als Klasse [{}] gespeichert.", label);
            }
        },
    );
}

// -----------------------------------------------------------------
// DATEI-I/O FUNKTION: SPEICHERN IN CSV
// -----------------------------------------------------------------
/// Schreibt das übergebene Label und die 256 Pixelwerte als neue Zeile in die Datei "zahlen_hand.csv"
/// Die Datei wird im "Append-Modus" geöffnet, das heißt neue Zeilen werden unten angehängt.
fn save_to_csv(label: u8, data: &[u8]) {
    // Öffne die Datei. Falls sie nicht existiert, wird sie automatisch erstellt (.create(true)).
    // Durch .append(true) wird der Dateizeiger an das Ende gesetzt, um Daten anzuhängen.
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(SPEICHER_PFAD)
        .unwrap();

    // Erstelle den Textstring für die Zeile. Wir beginnen mit dem Klassen-Label (0 oder 1)
    // erstes bit in jeder neuen zeile ist der key 0 o 1
    let mut line = format!("{}", label);

    // Iteriere durch alle 64 Pixel des Gitters und hänge sie kommagetrennt an die Zeile an
    for pixel in data {
        // Da es sich um Binärpixel handelt, formatieren wir sie kompakt ohne unnötige Nachkommastellen
        line.push_str(&format!(",{}", pixel));
    }

    // Füge einen Zeilenumbruch hinzu, damit das nächste Muster in einer neuen Zeile startet
    line.push('\n');

    // Schreibe den formatierten String in die Datei und stelle sicher, dass er direkt auf die Festplatte geschrieben wird
    file.write_all(line.as_bytes()).unwrap();
    file.flush().unwrap();
}
