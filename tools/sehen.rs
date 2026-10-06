use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Dimension der quadratischen Quell-Matrix (16x16 Pixel).
const GRID_SIZE: usize = 16;
// Skalierungsfaktor für das Upsampling des Render-Targets.
const PIXEL_SCALE: usize = 23;
const TOTAL_PIXELS: usize = GRID_SIZE * GRID_SIZE;

// Abgeleitete Dimensionen des Framebuffers (368x368 Pixel).
const WINDOW_WIDTH: usize = GRID_SIZE * PIXEL_SCALE;
const WINDOW_HEIGHT: usize = GRID_SIZE * PIXEL_SCALE;

// ARGB-Farbwerte (32-Bit Unsigned Integer: 0xAARRGGBB).
const COLOR_PAPIER: u32 = 0xFFCCCCCC; // Hintergrundfarbe (Grau)
const COLOR_STIFT: u32 = 0xFF554223; // Vordergrundfarbe (Braun)

const LADE_DATEI: &str = "datas/fertige_zahlen_hand.csv";

/// Heap-allokierte Datenstruktur für die Repräsentation einer CSV-Zeile.
struct DataRow {
    label: u8,          // Die klassifizierte Ziffer (Metadaten).
    grid_data: Vec<u8>, // Dynamisches Array der Pixelzustände (0 oder 1). Size: 256 Bytes.
}

fn main() {
    // I/O-Operation: Lädt den Datensatz sequentiell in den Hauptspeicher.
    let datei_sehen = env::args().nth(1).unwrap_or(LADE_DATEI.to_string());
    let dataset = load_csv(&datei_sehen);
    // Early Return bei fehlgeschlagenem I/O oder invalidem Dateiinhalt.
    if dataset.is_empty() {
        return;
    }

    // Index-Pointer für die Navigation innerhalb des `dataset`-Vektors.
    let mut current_index = 0;

    // Framebuffer-Allokation auf dem Heap (Größe: 368 * 368 * 4 Bytes = 541.696 Bytes).
    // Initialisiert mit dem Default-Farbwert `COLOR_PAPIER`.
    let mut window_buffer = vec![COLOR_PAPIER; WINDOW_WIDTH * WINDOW_HEIGHT];

    // Initialisierung des OS-Fenster-Kontexts über die FFI-Schnittstelle von `minifb`.
    let mut window = Window::new(
        "CSV Viewer",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Haupt-Event-Loop: Läuft, solange das OS-Fenster aktiv ist und kein ESC-Signal anliegt.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // --- INPUT-HANDLING (Nicht-blockierende Tastatur-Abfrage) ---
        // `KeyRepeat::No` fungiert als Entprellung (Debouncing) – triggert exakt einmal pro Tastendruck.
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::No)
            && current_index < dataset.len() - 1
        {
            current_index += 1; // Inkrementierung des Daten-Pointers
        } else if window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) && current_index > 0 {
            current_index -= 1; // Dekrementierung des Daten-Pointers
        }

        // Immutable Borrow auf das aktuelle `DataRow`-Strukturelement.
        let current_row = &dataset[current_index];

        // Dynamische String-Formatierung für den OS-Window-Title (erzeugt temporäre Heap-Allokation).
        window.set_title(&format!(
            "Muster {}/{} | Label: [{}]",
            current_index + 1,
            dataset.len(),
            current_row.label
        ));

        // --- SOFTWARE-RENDERING (Upsampling via Nearest-Neighbor-Interpolation) ---
        // Verschachtelte O(N²)-Schleife über alle Pixel des Ziel-Fensters (368x368 Iterationen).
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                // Ganzzahl-Division (Integer Division) projiziert die Ziel-Koordinate (x,y)
                // zurück auf die Koordinaten der ursprünglichen 16x16 Quell-Matrix.
                let grid_y = y / PIXEL_SCALE;
                let grid_x = x / PIXEL_SCALE;

                // Linearisierung der zweidimensionalen Quell-Koordinaten in einen 1D-Array-Index.
                let flat_grid_index = grid_y * GRID_SIZE + grid_x;

                // Abfrage des Binärwerts aus der Datenstruktur und bedingte Zuweisung der Pixelfarbe.
                let pixel_color = if current_row.grid_data[flat_grid_index] == 1 {
                    COLOR_STIFT
                } else {
                    COLOR_PAPIER
                };

                // Linearisierung der Ziel-Koordinaten zur Adressierung des 1D-Window-Buffers.
                window_buffer[y * WINDOW_WIDTH + x] = pixel_color;
            }
        }

        // FFI-Call: Überträgt den linearisierten u32-Buffer an das Grafiksubsystem des Betriebssystems
        // und erzwingt einen Redraw des Fensters (V-Sync-abhängig je nach OS-Konfiguration).
        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

/// System-I/O und CSV-Parsing
fn load_csv(path: &str) -> Vec<DataRow> {
    // Öffnet den System-File-Descriptor. Fehlerzustände (z.B. NotFound, PermissionDenied)
    // werden via Pattern Matching abgefangen und in ein leeres Fallback-Array gewandelt.
    let file = if let Ok(f) = File::open(path) {
        f
    } else {
        return Vec::new();
    };

    // Schaltet einen 8KB großen Stream-Buffer (`BufReader`) vor das File-Handle,
    // um die Anzahl teurer System-Calls (`read`) durch Block-Inlesung zu minimieren.
    let reader = BufReader::new(file);
    let mut dataset = Vec::new();

    // Iterator-Kette: `lines()` erzeugt `Result<String, Error>`.
    // `map_while(Result::ok)` filtert `Err` und transformiert `Ok(String)` in `String`.
    // Verhindert Panics und unendliche Schleifen bei I/O-Unterbrechungen während des Lesens.
    for line_text in reader.lines().map_while(Result::ok) {
        // Entfernt führende/nachgelagerte Whitespaces und Steuerzeichen (\r, \n).
        let trimmed = line_text.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Allokiert einen temporären Vektor aus String-Slices (`&str`), die als Pointer
        // direkt auf Segmente des validierten `line_text`-Strings im Stack verweisen.
        let parts: Vec<_> = trimmed.split(',').collect();

        // Validierung der Datenintegrität: Erwartet exakt 1 Label-Byte + 256 Pixel-Bytes.
        if parts.len() == TOTAL_PIXELS + 1 {
            // Parsen des Labels (Index 0). Fehlerhafte Zeichenfolgen werden zu 0 evaluiert.
            let label = parts[0].parse().unwrap_or(0);

            // Deklarative Iterator-Transformation für die Pixel-Daten (Indices 1 bis 256):
            // 1. Slicing `parts[1..]` trennt das Label ab.
            // 2. `.iter()` erzeugt Pointer-Stream.
            // 3. `.map()` konvertiert ASCII-Strings in u8-Integer.
            // 4. `.collect()` allokiert das fertige `Vec<u8>` auf dem Heap.
            let grid_data: Vec<u8> = parts[1..].iter().map(|p| p.parse().unwrap_or(0)).collect();

            // Verschiebung (Move) der allokierten Instanz in den globalen Dataset-Vektor.
            dataset.push(DataRow { label, grid_data });
        }
    }

    // Rückgabe des Vektors. Übergibt den Heap-Pointer an den Caller (Zero-Copy Transfer).
    dataset
}
