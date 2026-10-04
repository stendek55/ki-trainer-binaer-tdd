use ki_trainer_binaer::{BitByte, BitNeuralNetwork, Classification, training::load_champion};

// HIER binden wir ebenfalls die gui.rs aus der Nachbarschaft ein
mod gui;
use gui::starte_gitter_gui;

const KI_DATEI: &str = "mein_erstes_netzwerk.json";

fn main() {
    println!("Lade trainierte KI für Live-Test im Tool-Ordner...");
    let ki = load_champion(KI_DATEI).unwrap_or_else(|_| {
        println!("Keine 'trained_champion.json' gefunden! Zufallsmodus aktiv.");
        BitNeuralNetwork::new_random()
    });

    starte_gitter_gui("KI Live Echtzeit-Tester", |window, grid_data, _leeren| {
        // Gitter-Daten kompakt in das 32-BitByte Format der KI umwandeln
        let mut ki_input = [BitByte::new(0); 32];
        for i in 0..256 {
            if grid_data[i] == 1 {
                let byte_index = i / 8;
                let bit_position = (7 - (i % 8)) as u8;
                ki_input[byte_index] = ki_input[byte_index].set_bit(bit_position);
            }
        }

        // Live-Erkennung ausführen
        let ki_vorhersage = ki.forward_pass(&ki_input);
        let vorhersage_text = match ki_vorhersage {
            Classification::NULL => "0 (Null)",
            Classification::EINS => "1 (Eins)",
            Classification::ANDERE => "Unbekannt / Andere",
        };

        window.set_title(&format!("KI sieht gerade: {}", vorhersage_text));
    });
}
