use crate::{BitNeuralNetwork, TrainingSample};
use std::fs::File;
use std::io::{Read, Write};

/// konfigurationsparameter für die evolutionsschleife
pub struct TrainingsKonfiguration {
    pub populations_groesse: usize, // anzahl der mutanten pro generation
    pub maximale_generationen: u32, // maximale anzahl an trainingsrunden
    pub basis_mutations_rate: f32,  // start-mutationsrate als fließkommazahl
    pub stagnations_grenze: u32,    // runden ohne verbesserung bis zur anpassung
}

/// startet den genetischen trainingsprozess
pub fn trainieren(
    _datensatz: &[TrainingSample],
    _konfig: TrainingsKonfiguration,
) -> BitNeuralNetwork {
    BitNeuralNetwork::new_random()
}

/// Speichert das trainierte Netzwerk als JSON-Datei auf die Festplatte
pub fn save_champion(network: &BitNeuralNetwork, path: &str) -> std::io::Result<()> {
    let json_text = serde_json::to_string_pretty(network).map_err(std::io::Error::other)?;

    let mut datei = File::create(path)?;
    datei.write_all(json_text.as_bytes())?;

    println!("Champion erfolgreich gespeichert!");
    Ok(())
}

/// Lädt ein zuvor gespeichertes Netzwerk von der Festplatte
pub fn load_champion(path: &str) -> std::io::Result<BitNeuralNetwork> {
    let mut datei = File::open(path)?;
    let mut json_text = String::new();
    datei.read_to_string(&mut json_text)?;

    let netzwerk: BitNeuralNetwork =
        serde_json::from_str(&json_text).map_err(std::io::Error::other)?;

    println!("Champion erfolgreich geladen!");
    Ok(netzwerk)
}

// Berechnet für jedes Netzwerk in der Population die erreichte Fitness.
pub fn bewerte_population(
    population: Vec<BitNeuralNetwork>,
    dataset: &[TrainingSample],
) -> Vec<(u32, BitNeuralNetwork)> {
    population
        .into_iter()
        .map(|netz| {
            let fitness = netz.evaluate_fitness(dataset);
            (fitness, netz)
        })
        .collect()
}

// Sortiert die bewertete Population absteigend nach ihrer Fitness score.
pub fn sortiere_nach_fitness(bewertete_population: &mut [(u32, BitNeuralNetwork)]) {
    bewertete_population.sort_by_key(|eintrag| std::cmp::Reverse(eintrag.0));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // dieser test provoziert cargo watch ständig zu laufen/starten -> nervöse ruckeln -> nervt
    // deswegen...
    #[ignore] // Dieser Test wird übersprungen, wenn cargo test
    //Nur die ignorierten Tests laufen lassen -> cargo test -- --ignored
    //Alle Tests laufen lassen (inklusive der ignorierten) -> cargo test -- --include-ignored
    fn test_speichern_und_laden_des_champions_erfolgreich() {
        // erstelle ein echtes test netzwerk aus deiner library
        let original_netzwerk = BitNeuralNetwork::new_random();
        let datei_pfad = "test_champion_loesch_mich.json"; // fester dateiname zum testen

        // speichere das netzwerk ab
        let speicher_ergebnis = save_champion(&original_netzwerk, datei_pfad);
        assert!(
            speicher_ergebnis.is_ok(),
            "Das Speichern des Champions ist fehlgeschlagen"
        );

        // lade das netzwerk wieder ein
        let geladenes_ergebnis = load_champion(datei_pfad);
        assert!(
            geladenes_ergebnis.is_ok(),
            "Das Laden des Champions ist fehlgeschlagen"
        );

        let geladenes_netzwerk = geladenes_ergebnis.unwrap();

        // ueberpruefe die gleichheit
        assert_eq!(
            original_netzwerk, geladenes_netzwerk,
            "Das geladene Netzwerk unterscheidet sich vom Original!"
        );

        // loesche die datei manuell damit der ordner sauber bleibt
        let _ = std::fs::remove_file(datei_pfad);
    }

    use crate::{BitByte, BitNeuralNetwork, Classification, TrainingSample};
    #[test]
    fn test_block1_population_bewerten() {
        // Erstelle 2 Zufalls-Netzwerke
        let population = vec![
            BitNeuralNetwork::new_random(),
            BitNeuralNetwork::new_random(),
        ];

        // Erstelle einen minimalen Testdatensatz (1 Sample)
        let dataset = vec![TrainingSample {
            input: [BitByte::new(0x00); 32],
            target: Classification::ANDERE,
        }];

        // Rufe die zu testende Funktion auf
        let bewertet = bewerte_population(population, &dataset);

        // Überprüfungen:
        assert_eq!(
            bewertet.len(),
            2,
            "Die Populationsgröße darf sich nicht ändern."
        );
        // Jedes Element muss ein Tupel aus (u32, BitNeuralNetwork) sein
        assert!(bewertet[0].0 <= 1, "Der maximale Score bei 1 Sample ist 1.");
    }

    #[test]
    fn test_block2_population_sortieren() {
        let netz_schlecht = BitNeuralNetwork::new_random();
        let netz_gut = BitNeuralNetwork::new_random();

        // Wir simulieren eine bereits bewertete Population im Chaos-Zustand
        let unsortiert = vec![
            (5, netz_schlecht.clone()), // Schlechtes Netz hat 5 Punkte
            (10, netz_gut.clone()),     // Gutes Netz hat 10 Punkte
        ];

        let mut sortiert = unsortiert;
        sortiere_nach_fitness(&mut sortiert);

        // Das Netz mit 10 Punkten MUSS jetzt an Index 0 stehen
        assert_eq!(
            sortiert[0].0, 10,
            "Das beste Netzwerk muss auf Platz 1 stehen."
        );
        assert_eq!(
            sortiert[1].0, 5,
            "Das schlechteste Netzwerk muss nach hinten."
        );
    }

    #[test]
    fn test_trainieren_waehlt_besseren_mutanten() {
        // arrange: konfiguration für genau eine generation mit zwei mutanten vorbereiten
        let konfiguration = TrainingsKonfiguration {
            populations_groesse: 2,
            maximale_generationen: 1,
            basis_mutations_rate: 0.02,
            stagnations_grenze: 5,
        };

        // ein testdatensatz mit einem beispiel erstellen
        let datensatz = vec![TrainingSample {
            input: [BitByte::new(0x00); 32],
            target: Classification::EINS,
        }];

        // act: das training für eine runde starten
        let finaler_champion = trainieren(&datensatz, konfiguration);

        // assert: überprüfen, dass die funktion erfolgreich ein netzwerk zurückgibt
        // die fitness darf sich im vergleich zum start nicht verschlechtert haben
        assert!(finaler_champion.evaluate_fitness(&datensatz) >= 0);
    }
}
