use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

//entfernt aus einer csv die einträge mit wenig zeichen
//falls beim erstellen versehentlich fehleingaben gespeichert wurden
fn main() -> io::Result<()> {
    // Pfade für die Eingabe- und Ausgabe-Datei definieren
    let input_path = "datas/mehr_zahlen.csv";
    let output_path = "datas/fertige_daten.csv";

    // Eingabedatei öffnen und in einen gepufferten Reader laden (schont den RAM)
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Ausgabedatei erstellen
    let mut output_file = File::create(output_path)?;

    // Jede Zeile der CSV-Datei einzeln durchlaufen
    for line_result in reader.lines() {
        let line = line_result?;

        // Die Zeile anhand der Kommas in einzelne Felder zerlegen.
        // Danach zählen, wie viele Felder exakt "1" sind.
        let einsen_anzahl = line.split(',').filter(|&feld| feld.trim() == "1").count();

        // Bedingung: Nur in die neue Datei schreiben, wenn 6 oder mehr Einsen existieren
        if einsen_anzahl >= 16 {
            writeln!(output_file, "{}", line)?;
        }
    }

    println!("Filterung erfolgreich abgeschlossen!");
    Ok(())
}
