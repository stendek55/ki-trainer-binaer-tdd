use rand::Rng;
pub mod training;
use serde::{Deserialize, Serialize};
//###############################################################################################
//##########################-----BITOPERATIONEN-----#############################################
//###############################################################################################
/// Ein Wrapper für u8, der komfortable Bitoperationen per Punktoperator erlaubt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitByte(pub u8);

impl BitByte {
    // --- Konstruktor ---
    pub fn new(val: u8) -> Self {
        BitByte(val)
    }

    // Gibt den inneren u8-Wert zurück
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Führt ein bitweises AND (Und) mit einem anderen BitByte durch.
    /// Ein Bit im Ergebnis ist nur dann 1, wenn es in BEIDEN Ausgangszahlen 1 war.
    pub fn bitwise_and(&self, other: BitByte) -> Self {
        BitByte(self.0 & other.0)
    }

    /// Führt ein bitweises OR (Oder) mit einem anderen BitByte durch.
    /// Ein Bit im Ergebnis ist 1, wenn es in MINDESTENS EINER der beiden Zahlen 1 war.
    pub fn bitwise_or(&self, other: BitByte) -> Self {
        BitByte(self.0 | other.0)
    }

    /// Führt ein bitweises XOR (Exklusiv-Oder) mit einem anderen BitByte durch.
    /// Ein Bit im Ergebnis ist 1, wenn die Bits UNTERSCHIEDLICH sind (eins ist 1, das andere 0).
    pub fn bitwise_xor(&self, other: BitByte) -> Self {
        BitByte(self.0 ^ other.0)
    }

    /// Invertiert alle Bits des aktuellen Byte (bitweises NOT).
    /// Aus jeder 1 wird eine 0, aus jeder 0 eine 1.
    pub fn bitwise_not(&self) -> Self {
        BitByte(!self.0)
    }

    /// Schiebt alle Bits um X Positionen nach links.
    /// Rechts wird mit Nullen aufgefüllt. Bits, die links herausfallen, gehen verloren.
    pub fn shift_left(&self, positions: u8) -> Self {
        BitByte(self.0 << positions)
    }

    /// Schiebt alle Bits um X Positionen nach rechts.
    /// Links wird mit Nullen aufgefüllt. Bits, die rechts herausfallen, gehen verloren.
    pub fn shift_right(&self, positions: u8) -> Self {
        BitByte(self.0 >> positions)
    }

    /// Setzt das Bit am angegebenen Index (0-7) garantiert auf 1.
    pub fn set_bit(mut self, index: u8) -> Self {
        // 1. '1 << index' erstellt eine Schablone, bei der NUR das Bit am Index eine 1 ist.
        // 2. '|=' (OR) erzwingt an dieser Stelle eine 1, lässt alle anderen Bits unverändert.
        self.0 |= 1 << index;
        // Wir geben das modifizierte Objekt für das Method Chaining zurück.
        self
    }

    /// Löscht das Bit am angegebenen Index (0-7) garantiert (setzt es auf 0).
    pub fn clear_bit(mut self, index: u8) -> Self {
        // 1. '1 << index' erstellt die Maske (z.B. 0b0000_0100 bei Index 2).
        // 2. '!' invertiert die Maske (wird zu 0b1111_1011). Überall 1, außer am Ziel-Index.
        // 3. '&=' (AND) behält alle alten Bits bei (da & 1 nix ändert), löscht aber das Ziel-Bit (da & 0 = 0).
        self.0 &= !(1 << index);
        self
    }

    /// Invertiert (toggelt) das Bit am angegebenen Index (0-7).
    /// Aus 1 wird 0, aus 0 wird 1.
    pub fn toggle_bit(mut self, index: u8) -> Self {
        // 1. '1 << index' erstellt die Maske mit einer einzelnen 1 am Index.
        // 2. '^=' (XOR) dreht den Wert um: 1 ^ 1 wird zu 0, und 0 ^ 1 wird zu 1.
        self.0 ^= 1 << index;
        self
    }

    /// Prüft, ob das Bit am angegebenen Index (0-7) den Zustand 1 hat.
    pub fn check_bit(&self, index: u8) -> bool {
        // 1. 'self.0 & (1 << index)' isoliert das Bit. Alle anderen Stellen werden zu 0.
        // 2. Wenn das Ergebnis NICHT 0 ist, bedeutet das, dass das Bit am Index eine 1 war.
        // 3. Gibt true zurück wenn gesetzt, andernfalls false.
        (self.0 & (1 << index)) != 0
    }
}

//#################################################################################################
//##########################-----NETZSTRUKTUREN-----###############################################
//#################################################################################################
//##-----------netzwerk besteht aus folgenden drei hauptschichten (eigene festlegung)------------##
//##----->EINGABEschicht -> die 256 pixel (16x16) der zeichenmaske                               ##
//##----->HIDDENschicht  -> layer_1(64KNOTEN) | layer_2(32KNOTEN) | layer_3(16KNOTEN)            ##
//##        |---> das ist das eigentliche gehirn wo die berechnungen (entscheidungen) stattfinden##
//##----->AUSGABEschicht -> liefert das ergebnis -> ist es eine -> eins | null | wasanderes      ##
//#################################################################################################
/// Ein einzelner binärer Knoten im Netzwerk.
/// u16 für den threshold, damit die 256 Bits der ersten Schicht sicher abbilden können.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryNode<const WEIGHT_BYTES: usize> {
    /// Die gelernten Bit-Muster (Schablonen-Maske) für diesen Knoten.
    #[serde(with = "serde_arrays")]
    pub weights: [u8; WEIGHT_BYTES],

    /// Der Schwellenwert: Wie viele Bits müssen mindestens übereinstimmen?
    pub threshold: u16,
}

/// Die drei Zustände für Ausgabeschicht.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Classification {
    NULL,
    EINS,
    ANDERE,
}

/// Ein einzelnes Trainingsbeispiel, das eine gezeichnete Zahl und die korrekte Antwort enthält.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    /// Die 16x16 Matrix als 32 Bytes verpackt
    pub input: [BitByte; 32],
    /// Das Label, was es in Wirklichkeit ist (NULL, EINS oder ANDERE)
    pub target: Classification,
}

pub fn lade_test_datensatz(index: usize) -> TrainingSample {
    //let inhalt = include_str!("../tests/fixtures/test_daten_01.csv");
    let inhalt = include_str!("../datas/mehr_zahlen.csv");

    // alle zeilen trennen und die zeile am gewünschten index herausholen
    let ds = inhalt.lines().nth(index).expect("index nicht gefunden");

    // die zeile am ersten komma in den schlüssel und die restlichen bits trennen
    let (dskey, rest_bits) = ds.split_once(',').expect("ungültiges format");

    // den schlüssel bestimmen
    let key = match dskey.trim().parse().unwrap_or(0) {
        1 => Classification::EINS,
        0 => Classification::NULL,
        _ => Classification::ANDERE,
    };

    // die verbleibenden 256 bits parsen und in einen flachen vektor laden
    let bits: Vec<u8> = rest_bits
        .split(',')
        .map(|s| s.trim().parse().unwrap_or(0))
        .collect();

    // das neue array für die 32 BitBytes vorbereiten
    let mut input = [BitByte::new(0); 32];

    // die 256 einzelnen bits in die 32 BitBytes komprimieren
    for i in 0..256 {
        if bits.get(i) == Some(&1) {
            let byte_index = i / 8;
            //###############################################################
            //###############################################################
            //korrigiert das vertauchte einlesen
            //da die csv ja von links das erste zeichen liest
            // |---> wurde das als erstes bit rechts gesetzt
            // und wegen 2 byte breite
            // |---> führte zum effekt eines aufgerissenen vertikalen spiegel
            // AHAEFFEKT
            // ||---> wenn die ki trotzdem konsequent mit diesen
            // ||---> optisch falschen positionen trainiert würde
            // ||---> sollte das ergebnis dennoch richtig sein
            //###############################################################
            //###############################################################
            let bit_position = (7 - (i % 8)) as u8;

            // wir holen das aktuelle BitByte, setzen das bit über deine punktnotation und schreiben es zurück
            input[byte_index] = input[byte_index].set_bit(bit_position);
        }
    }

    TrainingSample { input, target: key }
}

/// Das vollständige neuronale Netzwerk mit deinen 3 Hidden Layers (64 -> 32 -> 16 -> 3).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitNeuralNetwork {
    pub hidden_1: Vec<BinaryNode<32>>,
    pub hidden_2: Vec<BinaryNode<8>>,
    pub hidden_3: Vec<BinaryNode<4>>,
    pub output_nodes: Vec<BinaryNode<2>>,
}
// =========================================================================
// IMPLEMENTIERUNG DES NETZWERKS: INITIALISIERUNG & ZUSAMMENBAU
// =========================================================================

// =========================================================================
// FABRIK-FUNKTION FÜR EINEN EINZELNEN KNOTEN
// =========================================================================

impl<const WEIGHT_BYTES: usize> BinaryNode<WEIGHT_BYTES> {
    /// Erstellt einen einzelnen Knoten mit zufälligen Gewichten und einem maßgeschneiderten Schwellenwert.
    ///
    /// * `rng`: Eine veränderliche Referenz auf den Zufallsgenerator des Systems.
    /// * `min_thresh` / `max_thresh`: Der Wertebereich für den gewürfelten Schwellenwert.
    pub fn new_random(rng: &mut rand::rngs::ThreadRng, min_thresh: u16, max_thresh: u16) -> Self {
        // 1. Speicherplatz exakt in der Größe reservieren, die die Schicht benötigt
        let mut weights = [0u8; WEIGHT_BYTES];

        // 2. Den Speicherbereich (alle Bits) in einem Rutsch mit Zufall füllen
        rng.fill(&mut weights);

        // 3. Den Schwellenwert im übergebenen Bereich würfeln
        let threshold = rng.random_range(min_thresh..=max_thresh);

        // 4. Den fertigen Knoten zurückgeben
        BinaryNode { weights, threshold }
    }

    /// MUTATION AUF KNOTEN-EBENE
    /// Geht durch alle Gewichts-Bytes des Knotens und flippt zufällig Bits
    /// basierend auf einer Mutationsrate (z. B. 0.05 für 5% Chance).
    pub fn mutate(&mut self, rng: &mut rand::rngs::ThreadRng, mutations_rate: f32) {
        use rand::Rng;

        // 1. Schleife über alle Bytes, die dieser Knoten besitzt
        for byte in self.weights.iter_mut() {
            // Wir würfeln eine Zahl zwischen 0.0 und 1.0. Ist sie kleiner als die Rate, mutieren wir!
            if rng.random::<f32>() < mutations_rate {
                // Wir würfeln einen zufälligen Bit-Index von 0 bis 7
                let zufall_bit_index = rng.random_range(0..8);

                // HIER NUTZEN WIR DEINE EIGENE TOGGLE-LOGIK!
                // Wir packen das aktuelle Byte in dein BitByte, flippen das Bit und schreiben es zurück.
                let mut bb = BitByte::new(*byte);
                bb = bb.toggle_bit(zufall_bit_index);
                *byte = bb.value();
            }
        }

        // 2. Auch der Schwellenwert (threshold) muss mutieren dürfen!
        // Mit einer 20%-Chance passen wir den Schwellenwert leicht an (+1 oder -1)
        if rng.random::<f32>() < 0.20 {
            if rng.random::<bool>() {
                self.threshold = self.threshold.saturating_add(1);
            } else {
                self.threshold = self.threshold.saturating_sub(1);
            }
        }
    }
}
//--------------------------------------------------------------------------
// die funktion für einen knoten aus schicht 2 aufrufen
// BinaryNode<8>, der 8 Bytes = 64 Bits verarbeitet) mit einem Schwellenwert-Bereich von 25..=45
// Wenn dieser Funktionsaufruf durchgelaufen ist, liegt im Arbeitsspeicher ein fertiges BinaryNode-Objekt.
// Wenn man es ausdrucken würde, sähe es (binär dargestellt) genau so aus:
/* das ist ein knoten aus schicht zwei -> davon liegen 32 stück im ram
BinaryNode {
    // 1. Die gewürfelten Gewichte (8 Bytes im RAM)
    weights: [
        0b1011_0010, // Byte 0
        0b0100_1111, // Byte 1
        0b1110_0001, // Byte 2
        0b0000_1101, // Byte 3
        0b1010_1010, // Byte 4
        0b1111_0000, // Byte 5
        0b0011_1100, // Byte 6
        0b0110_1001, // Byte 7
    ],

    // 2. Der gewürfelte Schwellenwert (als u16 im RAM)
    threshold: 37,
}
*/

// =========================================================================
// AUFGERÄUMTER ZUSAMMENBAU DES GESAMT-NETZWERKS
// =========================================================================
// =====================================================================================
// INITIALISIERUNG DES NETZWERKS (FABRIK-FUNKTION):
// =====================================================================================
// Erstellt ein komplett neues, untrainiertes Netzwerk im RAM.
// 1. `.map(...)`: Iteriert sequentiell durch die festgelegte Knotenanzahl pro Schicht.
// 2. Gewichte: Füllt die Byte-Arrays der Knoten via `rng.fill` mit zufälligen Startbits.
// 3. Thresholds: Würfelt für jeden Knoten einen Schwellenwert im sicheren AND-Bereich.
// 4. `.collect()`: Sammelt alle erzeugten Knoten performant in die finalen Layer-Vektoren.
// =====================================================================================
impl BitNeuralNetwork {
    /// Erstellt das gesamte Netzwerk durch das Aufrufen der ausgelagerten Knoten-Fabrik.
    pub fn new_random() -> Self {
        let mut rng = rand::rng();
        //schicht eins mit 64 knoten zu je 32 byte
        let hidden_1: Vec<BinaryNode<32>> = (0..64)
            .map(|_| BinaryNode::new_random(&mut rng, 10, 18))
            .collect();

        //schicht zwei mit 32 knoten zu je 8 byte
        let hidden_2: Vec<BinaryNode<8>> = (0..32)
            .map(|_| BinaryNode::new_random(&mut rng, 8, 14))
            .collect();

        //schicht drei mit 16 knoten zu je 4 byte
        let hidden_3: Vec<BinaryNode<4>> = (0..16)
            .map(|_| BinaryNode::new_random(&mut rng, 4, 8))
            .collect();

        //ausgabeschicht mit 3 knoten zu je 2 byte
        let output_nodes: Vec<BinaryNode<2>> = (0..3)
            .map(|_| BinaryNode::new_random(&mut rng, 1, 4))
            .collect();
        // Alle sauber erzeugten Schichten zusammenfügen
        BitNeuralNetwork {
            hidden_1,
            hidden_2,
            hidden_3,
            output_nodes,
        }
    }

    /// Hilfsfunktion, die die Übereinstimmungen zwischen einem Eingabe-Slice
    /// und den Gewichten eines Knotens unter Verwendung von BitByte berechnet.
    // =====================================================================================
    // WARUM VON XNOR (!XOR) AUF AND UMSTELLEN (WICHTIGER LERN-EFFEKT):
    // =====================================================================================
    // ALT (XNOR):
    // Bei XNOR ergab (0 XNOR 0) eine 1. Das bedeutete: Das Netzwerk hat Punkte vergeben,
    // wenn SOWOHL das Bild als auch die gelernten Gewichte eine Null (Hintergrund) hatten.
    // Da ein 16x16 Bild zu ~85% aus leerem Hintergrund besteht, konnte ein "fauler" Knoten
    // astronomisch hohe Match-Scores erzielen, bloß weil er den Hintergrund wiedererkannte.
    // Das Modell hat also primär gelernt, die Abwesenheit von Pixeln zu klassifizieren.
    //
    // NEU (AND):
    // Bei einem bitweisen AND gilt: Nur (1 AND 1) ergibt eine 1. Überall dort, wo das Bild
    // eine Null hat (Hintergrund), wird das Ergebnis knallhart auf 0 erzwungen – völlig egal,
    // was der Knoten dort im Speicher hat (0 & 1 = 0; 0 & 0 = 0).
    // Dadurch wird der leere Hintergrund komplett ausgeblendet. Der Knoten wird AUSSCHLIESSLICH
    // dafür belohnt, wenn er echte, aktiv gezeichnete Linien und Formen der Zahl trifft!
    //
    // HINWEIS: Nach dem Wechsel müssen die zufälligen Start-Thresholds in `new_random()`
    // drastisch abgesenkt werden (z.B. von 100-180 runter auf 10-25), da die "geschenkten"
    // Hintergrund-Punkte nun wegfallen und das Netzwerk sonst "verhungert".
    // =====================================================================================
    fn berechne_knoten_matches(input_layer: &[u8], knoten_weights: &[u8]) -> u32 {
        let mut gesamt_matches = 0u32;

        for (layer_byte, weight_byte) in input_layer.iter().zip(knoten_weights.iter()) {
            let bb_layer = BitByte::new(*layer_byte);
            let bb_weight = BitByte::new(*weight_byte);

            //alt -> XOR
            //let xnor_byte = bb_layer.bitwise_xor(bb_weight).bitwise_not();
            //gesamt_matches += xnor_byte.value().count_ones();

            //AND -> dadurch wird der hintergrund aus nullen ignoriert
            let and_byte = bb_layer.bitwise_and(bb_weight);
            gesamt_matches += and_byte.value().count_ones();
        }
        gesamt_matches
    }

    /// Hilfsfunktion, die prüft, ob der Schwellenwert erreicht ist,
    /// und das entsprechende Bit im Ausgangs-Array mithilfe von BitByte aktiviert.
    fn aktiviere_ausgangs_bit(
        layer_output: &mut [u8],
        node_idx: usize,
        matches: u32,
        threshold: u16,
    ) {
        if matches >= threshold as u32 {
            let byte_pos = node_idx / 8;
            let bit_pos = node_idx % 8;

            let mut bb_layer = BitByte::new(layer_output[byte_pos]);
            bb_layer = bb_layer.set_bit(bit_pos as u8);
            layer_output[byte_pos] = bb_layer.value();
        }
    }

    /// Schleust eine 16x16 Matrix (gespeichert als 32 Bytes = 256 Bits) durch alle
    /// drei Schichten des Netzwerks und gibt das Ergebnis der Erkennung zurück.
    pub fn forward_pass(&self, input: &[BitByte; 32]) -> Classification {
        // =========================================================================
        // SCHICHT 1: 256 Eingangs-Bits -> 64 Ausgangs-Bits (8 Bytes)
        // =========================================================================
        let mut layer_1_output = [0u8; 8];

        for node_idx in 0..64 {
            let knoten = &self.hidden_1[node_idx];

            // anpassung hier: wir berechnen die matches direkt, indem wir die inneren u8-werte übergeben
            let mut gesamt_matches = 0u32;
            for (bb_layer, weight_byte) in input.iter().zip(knoten.weights.iter()) {
                let bb_weight = BitByte::new(*weight_byte);
                // bb_layer ist bereits ein BitByte, wir rufen direkt deine operationen auf
                let xnor_byte = bb_layer.bitwise_xor(bb_weight).bitwise_not();
                gesamt_matches += xnor_byte.value().count_ones();
            }
            Self::aktiviere_ausgangs_bit(
                &mut layer_1_output,
                node_idx,
                gesamt_matches,
                knoten.threshold,
            );
        }

        // =========================================================================
        // SCHICHT 2: 64 Bits (8 Bytes) -> 32 Ausgangs-Bits (4 Bytes)
        // =========================================================================
        let mut layer_2_output = [0u8; 4];

        for node_idx in 0..32 {
            let knoten = &self.hidden_2[node_idx];
            let gesamt_matches = Self::berechne_knoten_matches(&layer_1_output, &knoten.weights);

            Self::aktiviere_ausgangs_bit(
                &mut layer_2_output,
                node_idx,
                gesamt_matches,
                knoten.threshold,
            );
        }

        // =========================================================================
        // SCHICHT 3: 32 Bits (4 Bytes) -> 16 Ausgangs-Bits (2 Bytes)
        // =========================================================================
        let mut layer_3_output = [0u8; 2];

        for node_idx in 0..16 {
            let knoten = &self.hidden_3[node_idx];
            let gesamt_matches = Self::berechne_knoten_matches(&layer_2_output, &knoten.weights);

            Self::aktiviere_ausgangs_bit(
                &mut layer_3_output,
                node_idx,
                gesamt_matches,
                knoten.threshold,
            );
        }

        // =========================================================================
        // AUSGABESCHICHT: Evaluierung der 3 Zustandsknoten (NULL, EINS, ANDERE)
        // =========================================================================
        // Wir sammeln die Trefferpunkte für jeden der 3 Ausgangsknoten
        let mut scores = [0u32; 3];

        for (score, knoten) in scores.iter_mut().zip(self.output_nodes.iter()) {
            for (l3_byte, weight_byte) in layer_3_output.iter().zip(knoten.weights.iter()) {
                let bb_l3 = BitByte::new(*l3_byte);
                let bb_weight = BitByte::new(*weight_byte);

                let xnor_byte = bb_l3.bitwise_xor(bb_weight).bitwise_not();
                // Wir addieren die Hardware-Popcounts direkt auf die veränderbare Referenz (*score)
                *score += xnor_byte.value().count_ones();
            }
        }

        // Winner-Takes-All Auswertung (Wer hat die meisten Bit-Übereinstimmungen?)
        let punkte_null = scores[0];
        let punkte_eins = scores[1];
        let punkte_andere = scores[2];

        // Wenn der "ANDERE"-Knoten gewinnt oder Gleichstand herrscht, brechen wir ab
        if punkte_andere >= punkte_null && punkte_andere >= punkte_eins {
            return Classification::ANDERE;
        }

        // Wir fordern eine klare Konfidenz (Vorsprung von mindestens 2 Punkten),
        // um Rauschen oder uneindeutige Zeichnungen als ANDERE abzufangen.
        if punkte_eins > punkte_null && (punkte_eins - punkte_null) >= 2 {
            Classification::EINS
        } else if punkte_null > punkte_eins && (punkte_null - punkte_eins) >= 2 {
            Classification::NULL
        } else {
            Classification::ANDERE // Bei zu knappen Unterschieden
        }
    }
    /// MUTATION AUF NETZWERK-EBENE
    /// Wandert durch jede einzelne Schicht des Netzwerks und ruft für jeden
    /// Knoten die mutations_rate auf, um das "Gehirn" minimal per Zufall zu verändern.
    pub fn mutate(&mut self, mutations_rate: f32) {
        // Wir holen uns den Zufallsgenerator des Systems
        let mut rng = rand::rng();

        // 1. Mutiere Schicht 1 (Alle 64 Knoten)
        for knoten in self.hidden_1.iter_mut() {
            knoten.mutate(&mut rng, mutations_rate);
        }

        // 2. Mutiere Schicht 2 (Alle 32 Knoten)
        for knoten in self.hidden_2.iter_mut() {
            knoten.mutate(&mut rng, mutations_rate);
        }

        // 3. Mutiere Schicht 3 (Alle 16 Knoten)
        for knoten in self.hidden_3.iter_mut() {
            knoten.mutate(&mut rng, mutations_rate);
        }

        // 4. Mutiere die Ausgabeschicht (Alle 3 Ausgangsknoten)
        for knoten in self.output_nodes.iter_mut() {
            knoten.mutate(&mut rng, mutations_rate);
        }
    }

    /// EVALUIERUNG (Fitness-Funktion)
    /// Jagt einen ganzen Stapel an Testbildern durch das Netzwerk und zählt,
    /// wie viele das Netzwerk davon bereits fehlerfrei erraten hat.
    pub fn evaluate_fitness(&self, dataset: &[TrainingSample]) -> u32 {
        let mut korrekte_treffer = 0u32;

        for sample in dataset {
            // Wir jagen das Bild durch den Vorwärtspass
            let vorhersage = self.forward_pass(&sample.input);

            // Wenn die Vorhersage der Wahrheit entspricht, gibt es einen Punkt!
            if vorhersage == sample.target {
                korrekte_treffer += 1;
            }
        }

        korrekte_treffer
    }
}

// --- TDD Testumgebung mit Punktnotation (Vollständige Version) ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        // 0b1100 & 0b1010 = 0b1000
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_and(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_1000)
        );
        assert_eq!(
            BitByte::new(0xFF).bitwise_and(BitByte::new(0x00)),
            BitByte::new(0x00)
        );
    }

    #[test]
    fn test_or() {
        // 0b1100 | 0b1010 = 0b1110
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_or(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_1110)
        );
        assert_eq!(
            BitByte::new(0x00).bitwise_or(BitByte::new(0x55)),
            BitByte::new(0x55)
        );
    }

    #[test]
    fn test_xor() {
        // 0b1100 ^ 0b1010 = 0b0110
        assert_eq!(
            BitByte::new(0b0000_1100).bitwise_xor(BitByte::new(0b0000_1010)),
            BitByte::new(0b0000_0110)
        );
        assert_eq!(
            BitByte::new(0xFF).bitwise_xor(BitByte::new(0xFF)),
            BitByte::new(0x00)
        );
    }

    #[test]
    fn test_not() {
        // !0b0000_1111 = 0b1111_0000
        assert_eq!(
            BitByte::new(0b0000_1111).bitwise_not(),
            BitByte::new(0b1111_0000)
        );
        assert_eq!(BitByte::new(0x00).bitwise_not(), BitByte::new(0xFF));
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(
            BitByte::new(0b0000_0001).shift_left(3),
            BitByte::new(0b0000_1000)
        );
        // Überlauf-Verhalten bei u8 (Grenzfall)
        assert_eq!(
            BitByte::new(0b1000_0000).shift_left(1),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(
            BitByte::new(0b0000_1000).shift_right(3),
            BitByte::new(0b0000_0001)
        );
        assert_eq!(
            BitByte::new(0b0000_0001).shift_right(1),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_set_bit() {
        // Setze Bit an Index 2 (Wert 4)
        assert_eq!(
            BitByte::new(0b0000_0000).set_bit(2),
            BitByte::new(0b0000_0100)
        );
        // Bleibt 1, wenn es schon 1 war
        assert_eq!(
            BitByte::new(0b0000_0100).set_bit(2),
            BitByte::new(0b0000_0100)
        );
    }

    #[test]
    fn test_clear_bit() {
        // Lösche Bit an Index 3 (Wert 8)
        assert_eq!(
            BitByte::new(0b0000_1111).clear_bit(3),
            BitByte::new(0b0000_0111)
        );
        // Bleibt 0, wenn es schon 0 war
        assert_eq!(
            BitByte::new(0b0000_0111).clear_bit(3),
            BitByte::new(0b0000_0111)
        );
    }

    #[test]
    fn test_toggle_bit() {
        // Aus 0 wird 1
        assert_eq!(
            BitByte::new(0b0000_0000).toggle_bit(4),
            BitByte::new(0b0001_0000)
        );
        // Aus 1 wird 0
        assert_eq!(
            BitByte::new(0b0001_0000).toggle_bit(4),
            BitByte::new(0b0000_0000)
        );
    }

    #[test]
    fn test_check_bit() {
        assert!(BitByte::new(0b0000_1000).check_bit(3));
        assert!(!BitByte::new(0b0000_1000).check_bit(2));
    }

    #[test]
    fn test_method_chaining() {
        // Komplexer Ablauf über mehrere Mutationen hinweg
        let result = BitByte::new(0)
            .set_bit(0) // -> 0b0000_0001
            .set_bit(3) // -> 0b0000_1001
            .toggle_bit(4) // -> 0b0001_1001
            .clear_bit(0); // -> 0b0001_1000

        assert_eq!(result, BitByte::new(0b0001_1000));
        assert!(result.check_bit(3));
        assert!(result.check_bit(4));
        assert!(!result.check_bit(0));
    }

    // =========================================================================
    // ERWEITERTE TESTSUITE FÜR DIE NETZWERK-LOGIK (Vollständige Testabdeckung)
    // =========================================================================

    /// HILFSFUNKTION: Erstellt ein deterministisches (fest verdrahtetes) Testnetzwerk.
    /// Da ein über `new_random` erzeugtes Netzwerk rein zufällige Gewichte hat,
    /// lässt es sich nicht zuverlässig auf mathematische Logik prüfen.
    /// Hier erzwingen wir kontrollierte Zustände, um den `forward_pass` exakt zu testen.
    fn erstelle_vorhersagbares_netzwerk() -> BitNeuralNetwork {
        // Schicht 1, 2 und 3 bekommen Schablonen aus puren Nullen und Threshold 0.
        // Das bedeutet: Jedes Bit passt im XNOR (0^0=!1) perfekt. Die Knoten feuern IMMER.
        let knoten_l1 = BinaryNode {
            weights: [0x00; 32],
            threshold: 0,
        };
        let knoten_l2 = BinaryNode {
            weights: [0x00; 8],
            threshold: 0,
        };
        let knoten_l3 = BinaryNode {
            weights: [0x00; 4],
            threshold: 0,
        };

        // Die Ausgabeschicht steuern wir jetzt präzise über die Gewichts-Masken:
        // Da die Schicht 3 davor immer nur Einsen feuert, erzeugt das Zwischenergebnis
        // in der Ausgabeschicht den Byte-Zustand [0xFF, 0xFF] (pures High-Signal).

        // Knoten NULL (Index 0): Maske verlangt Nullen -> XNOR mit 0xFF liefert 0 Treffer.
        let knoten_null = BinaryNode {
            weights: [0x00, 0x00],
            threshold: 0,
        };
        // Knoten EINS (Index 1): Maske verlangt Einsen -> XNOR mit 0xFF liefert 16 Treffer (Maximum).
        let knoten_eins = BinaryNode {
            weights: [0xFF, 0xFF],
            threshold: 0,
        };
        // Knoten ANDERE (Index 2): Maske verlangt ein mittleres Rauschen.
        let knoten_andere = BinaryNode {
            weights: [0x55, 0xAA],
            threshold: 0,
        };

        BitNeuralNetwork {
            hidden_1: vec![knoten_l1; 64],
            hidden_2: vec![knoten_l2; 32],
            hidden_3: vec![knoten_l3; 16],
            output_nodes: vec![knoten_null, knoten_eins, knoten_andere],
        }
    }

    #[test]
    fn test_netzwerk_initialisierung() {
        // Testet, ob die Fabrik-Funktion 'new_random' fehlerfrei durchläuft
        let netzwerk = BitNeuralNetwork::new_random();

        // Überprüfung der strukturellen Integrität (Array-Längen der Schichten)
        assert_eq!(netzwerk.hidden_1.len(), 64);
        assert_eq!(netzwerk.hidden_2.len(), 32);
        assert_eq!(netzwerk.hidden_3.len(), 16);
        assert_eq!(netzwerk.output_nodes.len(), 3);

        // Validierung, dass die erwürfelten Thresholds innerhalb der definierten Schranken liegen
        assert!(netzwerk.hidden_1[0].threshold >= 100 && netzwerk.hidden_1[0].threshold <= 180);
        assert!(netzwerk.hidden_2[0].threshold >= 25 && netzwerk.hidden_2[0].threshold <= 45);
        assert!(netzwerk.hidden_3[0].threshold >= 12 && netzwerk.hidden_3[0].threshold <= 24);
        assert!(
            netzwerk.output_nodes[0].threshold >= 6 && netzwerk.output_nodes[0].threshold <= 12
        );
    }

    #[test]
    fn test_forward_pass_klassifizierung_eins() {
        let netzwerk = erstelle_vorhersagbares_netzwerk();
        let leerer_input = [BitByte::new(0x00); 32]; // Löst die maximale Übereinstimmungskette aus

        // Der Forward-Pass muss unter diesen kontrollierten Bedingungen zwingend EINS ausgeben,
        // da der Knoten EINS 16 Treffer erzielt und die geforderte Konfidenz von >= 2 Punkten erfüllt.
        let ergebnis = netzwerk.forward_pass(&leerer_input);
        assert_eq!(ergebnis, Classification::EINS);
    }

    #[test]
    fn test_forward_pass_klassifizierung_andere_bei_gleichstand() {
        let mut netzwerk = erstelle_vorhersagbares_netzwerk();

        // Wir manipulieren das Netz künstlich zu einem absoluten Gleichstand (Patt):
        // Knoten NULL und Knoten EINS bekommen exakt dieselbe Gewichtsmaske.
        netzwerk.output_nodes[0].weights = [0xFF, 0xFF]; // Knoten NULL fordert nun auch Max-Treffer

        let leerer_input = [BitByte::new(0x00); 32];
        let ergebnis = netzwerk.forward_pass(&leerer_input);

        // Bei einem Patt oder zu geringem Vorsprung (< 2 Punkte) greift die Konfidenz-Klaue
        // und das System muss "ANDERE" (Unbekannt) zurückgeben.
        assert_eq!(ergebnis, Classification::ANDERE);
    }

    #[test]
    fn test_netzwerk_mutation() {
        let mut netzwerk = BitNeuralNetwork::new_random();
        let netzwerk_kopie = netzwerk.clone();

        // Wir erzwingen eine radikale Mutationsrate von 100% (1.0).
        // Jedes einzelne Gewichts-Byte im gesamten Netz MUSS sich nun verändern!
        netzwerk.mutate(1.0);

        // Überprüfung Schicht 1: Die Gewichte dürfen nach der 100%-Mutation nicht mehr identisch sein
        assert_ne!(
            netzwerk.hidden_1[0].weights,
            netzwerk_kopie.hidden_1[0].weights
        );
        assert_ne!(
            netzwerk.output_nodes[0].weights,
            netzwerk_kopie.output_nodes[0].weights
        );
    }

    #[test]
    fn test_fitness_evaluierung() {
        let netzwerk = erstelle_vorhersagbares_netzwerk();

        // Wir bauen einen künstlichen Mini-Datensatz aus zwei Bildern
        let datensatz = vec![
            TrainingSample {
                input: [BitByte::new(0x00); 32], // Dieses Bild wird vom Testnetz als EINS erkannt
                target: Classification::EINS,    // Korrektes Label -> Gibt 1 Punkt
            },
            TrainingSample {
                input: [BitByte::new(0x00); 32], // Wird ebenfalls als EINS erkannt
                target: Classification::NULL,    // Falsches Label -> Gibt 0 Punkte
            },
        ];

        // Da genau eines von zwei Bildern mathematisch korrekt vorhergesagt wird,
        // muss die gemessene Fitness exakt 1 betragen.
        let fitness = netzwerk.evaluate_fitness(&datensatz);
        assert_eq!(fitness, 1);
    }
}
