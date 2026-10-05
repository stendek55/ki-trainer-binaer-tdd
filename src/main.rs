/*
use ki_trainer_binaer::*;
fn main() {
    println!("Hello, world!");
    // lädt den ersten datensatz aus deiner csv-datei
    let foobar = lade_test_datensatz(1);

    println!("--------------------------------------------------");
    println!("visualisierung des geladenen bildes (16x16 raster):");
    println!("--------------------------------------------------");

    // wir gehen durch alle 32 BitBytes im eingangs-array
    for (index, bit_byte) in foobar.input.iter().enumerate() {
        // wir greifen auf die innere zahl (.0) zu und drucken sie als 8 bits aus
        print!("{:08b}", bit_byte.0);

        // da das bild 16 pixel breit ist, machen wir nach jedem zweiten byte (16 bits) einen zeilenumbruch
        if (index + 1) % 2 == 0 {
            println!();
        }
    }

    println!("--------------------------------------------------");

}
    */
use ki_trainer_binaer::*;
// Wir holen uns zusätzlich die TrainingsKonfiguration und die Trainings-Funktion
use ki_trainer_binaer::training::{TrainingsKonfiguration, save_champion, trainieren};

fn main() {
    println!("--------------------------------------------------");

    // -------------------------------------------------------------------------
    // 2. MEHRERE DATENSÄTZE FÜR DAS TRAINING IN DEN RAM LADEN
    // -------------------------------------------------------------------------
    println!("\n-> Lade Trainings-Datensatz für die Evolution...");
    let mut trainings_daten = Vec::new();

    // Wir laden die ersten 30 Zeilen aus deiner CSV-Datei
    for i in 0..555 {
        trainings_daten.push(lade_test_datensatz(i));
    }
    println!("   {} Muster erfolgreich geladen.\n", trainings_daten.len());

    // -------------------------------------------------------------------------
    // 3. ENTSCHEIDUNG: WIE SOLL DIE EVOLUTION TRAINIEREN?
    // -------------------------------------------------------------------------
    let konfig = TrainingsKonfiguration {
        populations_groesse: 222, // 100 Mutanten kämpfen pro Runde um das Überleben
        maximale_generationen: 555, // Nach max. 400 Runden stoppt der Algorithmus
        basis_mutations_rate: 0.06, // 2% Chance, dass ein Gen (Bit) mutiert
        stagnations_grenze: 9,    // Wenn sich 15 Runden nix tut -> Mutationsrate erhöhen
    };

    // -------------------------------------------------------------------------
    // 4. NETZWERK GENERIEREN & EVOLUTION STARTEN
    // -------------------------------------------------------------------------
    // Diese Funktion erstellt intern ein zufälliges Netz und startet die Evolution
    let champion_netz = trainieren(&trainings_daten, konfig);

    // -------------------------------------------------------------------------
    // 5. DEN GEWINNER ALS JSON DATEI SPEICHERN
    // -------------------------------------------------------------------------
    // Nutzt deine save_champion-Funktion, um das trainierte Gehirn zu sichern
    let dateiname = "mein_erstes_netzwerk.json";
    if let Err(e) = save_champion(&champion_netz, dateiname) {
        println!("Fehler beim Speichern des Netzwerks: {}", e);
    } else {
        println!(
            "Das trainierte Netzwerk wurde in '{}' gesichert!",
            dateiname
        );
    }
}
