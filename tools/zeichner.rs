use minifb::{Key, MouseButton, Window, WindowOptions};
use std::fs::OpenOptions;
use std::io::Write;

// Architektonische Konstanten für das Grid
const GRID_SIZE: usize = 16;
const TOTAL_PIXELS: usize = GRID_SIZE * GRID_SIZE;

const PIXEL_SCALE: usize = 23;
const WINDOW_WIDTH: usize = GRID_SIZE * PIXEL_SCALE; // = 368
const WINDOW_HEIGHT: usize = GRID_SIZE * PIXEL_SCALE;

// Farbdefinitionen im ARGB-Format (Hexadezimal: 0xAARRGGBB)
const COLOR_WHITE: u32 = 0xFFFFFFFF; // Hintergrundfarbe (Inaktiv -> Wert 0)
const COLOR_BLACK: u32 = 0xFF000000; // Zeichenfarbe (Aktiv -> Wert 1)
const COLOR_GRID: u32 = 0xFFE0E0E0; // Hellgraue Gitterlinien zur Orientierung

const SPEICHER_PFAD: &str = "datas/zahlen_hand.csv";
//###########################################################################################
//######-----eigene FESTLEGEUNG-----#########################################################
// #####-----zu beginn wird das ganze projekt erstmal nur auf die 0 und 1 trainiert-----#####
// ##########################################################################################
fn main() {
    println!("ich bin im zeichner");
    // -----------------------------------------------------------------
    // INITIALISIERUNG DES INTERNEN ZUSTANDES (DAS SPEICHERLAYOUT)
    // -----------------------------------------------------------------
    // ein flaches Array für 16x16 Gitter (analog zur Matrix-Struktur).
    // 0 bedeutet "Weiß" (Hintergrund), 1 bedeutet "Schwarz" (Gezeichnet).
    let mut grid_data = vec![0u8; TOTAL_PIXELS];

    // Der Framebuffer speichert die echten Pixel-Farbwerte, die minifb auf dem Bildschirm anzeigt.
    // Größe entspricht der physischen Fenstergröße (368 * 368)
    let mut window_buffer = vec![COLOR_WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];

    // Erstellen des Anwendungsfensters mit Standardoptionen
    let mut window = Window::new(
        "handschriftzahlen - [0] | [1] -> speichern",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    println!("============================================================");
    println!("#######-----GRAFISCHER DATEN-GENERATOR GESTARTET-----#######");
    println!("============================================================");
    println!("  Bedienung:");
    println!("  - Linke Maustaste  : Pixel SCHWARZ zeichnen (Wert 1)");
    println!("  - Rechte Maustaste : Pixel WEISS radieren (Wert 0)");
    println!("  - Taste [0]        : Als Ziffer '0' in CSV speichern + Feld leeren");
    println!("  - Taste [1]        : Als Ziffer '1' in CSV speichern + Feld leeren");
    println!("  - Taste [C]        : Zeichenfeld komplett löschen (Clear)");
    println!("  - Taste [ESC]      : Programm sicher beenden");
    println!("============================================================");

    // -----------------------------------------------------------------
    // HAUPTSCHLEIFE DES PROGRAMMS (GUI-LOOP)
    // -----------------------------------------------------------------
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // --- MAUS-ABFRAGE & ZEICHENLOGIK ---
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            // Berechne aus der physischen Pixel-Mausposition (z.B. X=142, Y=55),
            // in welchem der 16x16 logischen Gitterfelder sich die Maus befindet.
            let grid_x = (mouse_x as usize) / PIXEL_SCALE;
            let grid_y = (mouse_y as usize) / PIXEL_SCALE;

            // Ermittlung des flachen Array-Index nach der Formel: Index = Zeile * Spalten + Spalte
            let flat_index = grid_y * GRID_SIZE + grid_x;

            if window.get_mouse_down(MouseButton::Left) {
                // Linksklick: Pixel aktivieren (schwarz)
                grid_data[flat_index] = 1;
            } else if window.get_mouse_down(MouseButton::Right) {
                // Rechtsklick: Pixel deaktivieren (weiß / radieren)
                grid_data[flat_index] = 0;
            }
        }

        // --- TASTATUR-ABFRAGE & SPEICHERLOGIK ---
        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
            // Taste [0] gedrückt: Speichere mit Label '0'
            save_to_csv(0, &grid_data);
            grid_data.fill(0); // Feld automatisch leeren
            println!("-> Muster erfolgreich als Klasse [0] gespeichert.");
        } else if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            // Taste [1] gedrückt: Speichere mit Label '1'
            save_to_csv(1, &grid_data);
            grid_data.fill(0); // Feld automatisch leeren
            println!("-> Muster erfolgreich als Klasse [1] gespeichert.");
        } else if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            // Taste [C] gedrückt: Feld manuell leeren
            grid_data.fill(0);
            println!("-> Zeichenfeld geleert.");
        }

        // -----------------------------------------------------------------
        // RENDERING (ZEICHNEN DES GITTERS IN DEN FRAMEBUFFER)
        // -----------------------------------------------------------------
        // transformiere 16x16-Gitter in den großen 368x368 Pixel-Puffer
        for y in 0..WINDOW_HEIGHT {
            // Bestimme das dazugehörige logische Gitterfeld für diese Zeile
            let grid_y = y / PIXEL_SCALE;
            // Prüfe, ob wir uns exakt auf einer horizontalen Gitterlinie befinden
            let is_grid_line_y = (y % PIXEL_SCALE == 0) || (y == WINDOW_HEIGHT - 1);

            for x in 0..WINDOW_WIDTH {
                // Bestimme das dazugehörige logische Gitterfeld für diese Spalte
                let grid_x = x / PIXEL_SCALE;
                // Prüfe, ob wir uns exakt auf einer vertikalen Gitterlinie befinden
                let is_grid_line_x = (x % PIXEL_SCALE == 0) || (x == WINDOW_WIDTH - 1);

                let window_index = y * WINDOW_WIDTH + x;

                if is_grid_line_y || is_grid_line_x {
                    // Wenn wir uns auf einer Gitterlinie befinden, zeichne sie grau
                    window_buffer[window_index] = COLOR_GRID;
                } else {
                    // Ansonsten bestimme die Pixelfarbe basierend auf den Zeichendaten (0 oder 1)
                    let flat_grid_index = grid_y * GRID_SIZE + grid_x;
                    if grid_data[flat_grid_index] == 1 {
                        window_buffer[window_index] = COLOR_BLACK;
                    } else {
                        window_buffer[window_index] = COLOR_WHITE;
                    }
                }
            }
        }

        // Aktualisiere das Fenster mit dem neu berechneten Framebuffer
        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
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
