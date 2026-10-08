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
    for i in 0..1626 {
        trainings_daten.push(lade_test_datensatz(i));
    }
    println!("   {} Muster erfolgreich geladen.\n", trainings_daten.len());

    // -------------------------------------------------------------------------
    // 3. ENTSCHEIDUNG: WIE SOLL DIE EVOLUTION TRAINIEREN?
    // -------------------------------------------------------------------------
    let konfig = TrainingsKonfiguration {
        populations_groesse: 100, // 100 Mutanten kämpfen pro Runde um das Überleben
        maximale_generationen: 400, // Nach max. 400 Runden stoppt der Algorithmus
        basis_mutations_rate: 0.05, // 5% Chance, dass ein Gen (Bit) mutiert
        stagnations_grenze: 9,    // Wenn sich 9 Runden nix tut -> Mutationsrate erhöhen
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
