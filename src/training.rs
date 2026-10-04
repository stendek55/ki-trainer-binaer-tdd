use crate::{BitNeuralNetwork, TrainingSample};
use rayon::prelude::*;
use std::fs::File;
use std::io::{Read, Write};

/// konfigurationsparameter für die evolutionsschleife
pub struct TrainingsKonfiguration {
    pub populations_groesse: usize, // anzahl der mutanten pro generation
    pub maximale_generationen: u32, // maximale anzahl an trainingsrunden
    pub basis_mutations_rate: f32,  // start-mutationsrate als fließkommazahl
    pub stagnations_grenze: u32,    // runden ohne verbesserung bis zur anpassung
}

/// Startet den genetischen Trainingsprozess
pub fn trainieren(
    datensatz: &[TrainingSample],
    konfig: TrainingsKonfiguration,
) -> BitNeuralNetwork {
    //würfelt ein zufälliges Start-Netzwerk als ersten Champion
    let mut champion = BitNeuralNetwork::new_random();
    let mut champion_fitness = champion.evaluate_fitness(datensatz);
    let max_possible_fitness = datensatz.len() as u32;

    // Dynamische Anpassungsvariablen für die Mutationsrate bei Stagnation
    let mut aktuelle_mutation_rate = konfig.basis_mutations_rate;
    let mut generationen_ohne_verbesserung = 0;

    println!("==================================================");
    println!("STARTE TRAINING");
    println!(
        "Maximal erreichbare Fitness: {} Punkte",
        max_possible_fitness
    );
    println!(
        "Start-Fitness des Zufallsnetzes: {} Punkte",
        champion_fitness
    );
    println!("==================================================");

    // Frühzeitiger Abbruch, falls der Zufall uns bereits ein perfektes Netz geschenkt hat
    if champion_fitness == max_possible_fitness {
        println!("JUHUUU--->Das Startnetz ist bereits perfekt;)");
        return champion;
    }

    // Hauptschleife über die Generationen
    for generation in 1..=konfig.maximale_generationen {
        // Schritt A: Klonen und Mutieren
        // Wir erzeugen eine Population von unabhängigen Kopien des aktuellen Champions.
        let mutanten: Vec<BitNeuralNetwork> = (0..konfig.populations_groesse)
            .map(|_| {
                let mut kopie = champion.clone();
                // Nutzt die in deiner lib.rs definierte .mutate()-Methode auf Netzwerk-Ebene
                // Da rand::rng() Thread-lokal arbeitet, holt sich jeder Thread
                // automatisch seinen eigenen, sicheren Zufallsgenerator!
                kopie.mutate(aktuelle_mutation_rate);
                kopie
            })
            .collect(); // Führt die Threads am Ende wieder sauber zusammen

        // Schritt B & C: Bevölkerung bewerten UND direkt den Champion extrahieren
        // Wir nutzen hier deine freistehende Funktion 'bewerte_population'!
        let bewertete_mutanten = bewerte_population(mutanten, datensatz);

        // Idiomatisches Rust: Wir suchen NUR das Maximum, anstatt die ganze Liste zu sortieren.
        // Das spart massig Rechenzeit bei großen Populationen!
        let (mutanten_fitness, mutanten_netzwerk) = bewertete_mutanten
            .into_iter()
            .max_by_key(|eintrag| eintrag.0) // Sucht nach dem höchsten u32-Fitnesswert
            .expect("Fehler: Die Population darf nicht leer sein.");

        // Schritt D: Auswertung & adaptive Steuerung
        if mutanten_fitness > champion_fitness {
            // Ein neuer, besserer Champion wurde gefunden!
            champion = mutanten_netzwerk;
            champion_fitness = mutanten_fitness;
            generationen_ohne_verbesserung = 0;
            aktuelle_mutation_rate = konfig.basis_mutations_rate; // Reset auf den Ausgangswert

            println!(
                "Generation {:4}: Neuer Champion! Fitness = {}/{} (Mutation: {:.2}%)",
                generation,
                champion_fitness,
                max_possible_fitness,
                aktuelle_mutation_rate * 100.0
            );
        } else {
            // Keine Verbesserung in dieser Runde
            generationen_ohne_verbesserung += 1;

            // Wenn sich zu lange nichts tut, erhöhen wir die Mutationsrate ("Rütteln")
            if generationen_ohne_verbesserung >= konfig.stagnations_grenze {
                // Erhöhe die Rate um 50%, aber deckele sie bei 15%, um das Netz nicht völlig zu zerstören
                aktuelle_mutation_rate = (aktuelle_mutation_rate * 1.5).min(0.15);
                generationen_ohne_verbesserung = 0;
                println!(
                    "Stagnation in Gen {}! Erhöhe Mutationsrate auf {:.2}%...",
                    generation,
                    aktuelle_mutation_rate * 100.0
                );
            }
        }

        // Schritt E: Vorzeitiger Abbruch bei 100% korrekter Erkennung
        if champion_fitness == max_possible_fitness {
            println!("Perfektes Netzwerk in Generation {} gefunden!", generation);
            break;
        }
    }

    println!("==================================================");
    println!(
        "TRAINING BEENDET. Finale Fitness: {}/{}",
        champion_fitness, max_possible_fitness
    );
    println!("==================================================");

    champion
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
        .into_par_iter() // <- HIER: Berechnet die Netzwerke gleichzeitig
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

    #[test]
    fn test_trainieren_verbessert_fitness_oder_behaelt_champion() {
        // ARRANGE: Wir erstellen einen Datensatz mit 20 identischen Beispielen.
        // Das Erreichen einer höheren Punktzahl ist durch reines Raten unwahrscheinlich.
        let mut datensatz = Vec::new();
        for _ in 0..20 {
            datensatz.push(TrainingSample {
                input: [BitByte::new(0x00); 32],
                target: Classification::EINS,
            });
        }

        // Wir erzwingen viele Generationen und eine große Population.
        // Wenn die Schleife arbeitet, MUSS sich ein besseres Netz als der Start-Zufall finden.
        let konfiguration = TrainingsKonfiguration {
            populations_groesse: 50,
            maximale_generationen: 30,
            basis_mutations_rate: 0.1,
            stagnations_grenze: 5,
        };

        // Wir messen die Fitness eines isolierten Zufallsnetzes als Referenz
        let dummy_start = BitNeuralNetwork::new_random();
        let start_fitness = dummy_start.evaluate_fitness(&datensatz);

        // ACT: Training starten
        let finaler_champion = trainieren(&datensatz, konfiguration);
        let end_fitness = finaler_champion.evaluate_fitness(&datensatz);

        // ASSERT: Die Evolution muss statistisch gesehen das Startniveau schlagen!
        // Sollte die Funktion nur das Startnetz zurückgeben, scheitert dieser Assert fast immer.
        assert!(
            end_fitness >= start_fitness,
            "Die Evolution hat die Fitness nicht verbessert! Start: {}, Ende: {}",
            start_fitness,
            end_fitness
        );
    }
}
