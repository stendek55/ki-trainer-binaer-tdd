---
## ............*in bearbeitung*..............
---

# ki-trainer-binaer
**zum erkennen handgeschriebener zahlen soll eigenes ki modell trainiert werden auf grundlage von bitoperationen?!?**
# Challenge: Bit-KI für Handschriften - Built from Scratch in Rust

## Worum geht es überhaupt?
Ich will wissen, ob es möglich ist, ein eigenes KI-Modell zur Erkennung von handschriftlichen Zahlen zu bauen, das ohne den typischen Mathe-Ballast auskommt. Normale KIs fressen Unmengen an Strom und Rechenleistung, weil sie ständig mit fetten Kommazahlen (Float32) jonglieren. Mein Ziel: Eine KI, die ausschließlich mit Low-Level-Bitoperationen (AND, OR, XOR, XNOR, NOT)??? auf der CPU arbeitet. Superschnell, superleicht und perfekt für winzige Hardware.

Da ich mir in vergangenen Projekten die Grundlagen von Rust erarbeitet habe, ziehe ich das Ganze komplett in Rust hoch. Rust ist durch seine Performance und Typsicherheit perfekt für dieses hardwarenahe Bit-Gebastel.

## Meine wichtigste Regel: Keine vorgefertigten Wege!
Ich will nicht einfach bestehende Papers (wie BNNs oder Tsetlin-Maschinen) eins zu eins kopieren oder mich von fertigen Frameworks in eine bestimmte Richtung lenken lassen. Ich möchte das Rad bewusst ein bisschen selbst neu erfinden, eigene Logik-Ansätze testen und schauen, wie weit ich mit meinen eigenen Ideen komme. 

Ich werde KI-Tools als Beschleuniger einsetzen, um Code-Strukturen zu optimieren oder Denkblockaden zu lösen - das eigentliche Architektur-Design und die Algorithmen sollen aber meinen eigenen Ideen entspringen.

## Was ich mir vorgenommen habe (Meine Meilensteine)

### 1. Der eigene Datengenerator
Ich verlasse mich nicht nur auf fertige Datensätze wie MNIST. Ich baue mir in Rust einen eigenen Datengenerator, der:
* Handgeschriebene Bitmaps (Binärbilder) von Ziffern erzeugt.
* Eigene "Bit-Verschiebe-Funktionen" nutzt (Bit-Shifting für Bewegung, Bit-Invertierung für Bildrauschen), um das Modell später richtig auf die Probe zu stellen.

### 2. Das Bit-Modell (Die eigentliche KI)
* Ich designe ein System in Rust, das Bilder einliest und die Klassifizierung rein über logische Verknüpfungen von Bits regelt.
* Ich will herausfinden, wie gut ein rein logisches Regelwerk performen kann.
* Kann ich alle Werte im u8 Bereich halten?

### 3. Der Härtetest
* Am Ende will ich schwarz auf weiß sehen: Funktioniert mein eigener Ansatz? 
* Wie schlägt sich meine Rust-Bit-KI auf meinen eigenen Daten und wie performant ist sie?

## Tech Stack und Skills
* Programmiersprache: Rust (Bit-Manipulation, hardwarenaher Code)
* Konzept: Edge-AI (Ressourceneffiziente Algorithmen ohne Ballast)
* Data Engineering: Eigene Datengenerierung und Bildbearbeitung auf Bitebene
* Arbeitsweise: KI-unterstützte Entwicklung (KI als Programmier-Copilot)

## Die Kernfrage, die ich mir selbst beantworte:
Kann man ein funktionsfähiges KI-Modell nur mit logischem Denken, ein paar Bits und feinstem Rust-Code bauen, ohne den Mainstream-Pfaden zu folgen?

---
---

# .....erstes FAZIT...
# Entwicklungsbericht: Eigenes Binäres Neuronales Netzwerk mit Evolutionärem Ansatz

## 1. Philosophie & Architektur-Ansatz
Die Grundidee war es, eine **eigene Netzwerkarchitektur** komplett nach meinen eigenen Vorstellungen aufzubauen. Ich habe mich bewusst dagegen entschieden, mich zu tief in bestehende Dokumentationen oder fertige Anleitungen zu binären Netzen einzulesen. Ziel war es, maximale kreative Freiheit zu behalten und mich nicht von bereits existierenden Standardlösungen lenken zu lassen.

### Die KI als Pair Programmer
Bei der Entwicklung habe ich eine KI als zusätzlichen Entwickler (Pair Programmer) genutzt. Die Strategie war hierbei, die KI immer nur in **kleinen, isolierten Abschnitten** mit konkreten Bauanweisungen zu füttern. Sie kannte zu keinem Zeitpunkt die gesamte Architektur. Trotz dieses fragmentierten Wissens der KI war die Entwicklungsgeschwindigkeit extrem hoch, und das Konzept ging voll auf.

---

## 2. Technischer Aufbau des Netzwerks
Das Netz ist als **Deep Binary Neural Network** (Mehrschichtiges binäres Netzwerk) strukturiert und besteht aus folgenden Schichten:

* **Eingabeschicht (Input Layer):** Nimmt die Bilddaten auf.
* **Verdeckte Schichten (Hidden Layers):** Drei Schichten zur Merkmalsextraktion.
  * *Layer 1:* 64 Knoten (Neuronen)
  * *Layer 2:* 32 Knoten
  * *Layer 3:* 16 Knoten
* **Ausgabeschicht (Output Layer):** 3 Knoten für die Klassifikation (Kategorien: *Eins*, *Null*, *Anderes*).

Jede Schicht arbeitet mit spezifischen **Schwellenwerten (Activation Thresholds)**. Zum Start des Netzwerks wird eine zufällige Initialisierung (Startkonfiguration) der Knoten vorgenommen.

### Performance-Meilenstein
Ein riesiger Sprung in der Performance war die Implementierung von echtem **Multiprocessing**. Durch nur wenige Zeilen Code läuft das Training nun **parallel auf allen CPU-Kernen**, was die Rechengeschwindigkeit massiv nach oben geschraubt hat. Auch die Erstellung der Tools zur Generierung und Erweiterung der Trainingsdaten (Data Augmentation) lief völlig problemlos.

---

## 3. Der Trainingsprozess: Genetischer Algorithmus
Da klassische mathematische Optimierungsmethoden hier nicht greifen, basiert das Training auf einem **Evolutionären Algorithmus (Klonen, Mutation und Selektion)**:

1. **Evaluation:** Das Netz mit der höchsten Trefferquote in einem Durchlauf gewinnt.
2. **Replikation (Klonen):** Das beste Netzwerk wird vervielfältigt.
3. **Mutation:** Auf die Klone wird eine einfache Mutation angewendet, bei der zufällig Bits aktiviert oder deaktiviert werden (Bit-Flipping).
4. **Iteratives Testen:** Die mutierten Netze werden erneut getestet. Setzt sich ein stärkeres Netz durch, wird dieses zum neuen Ausgangspunkt für die nächste Generation.

### Dynamische Mutationsrate (Adaptive Mutation)
Um aus lokalen Minima auszubrechen, habe ich eine dynamische Anpassung eingebaut: Wenn das Netzwerk stagniert – sich die Trefferquote also über mehrere Epochen nicht mehr verbessert –, wird die **Mutationsrate Schritt für Schritt erhöht**, um stärkere strukturelle Sprünge zu erzwingen.

---

## 4. Erkenntnisse & Optimierung der Logik
Im Laufe des Trainings gab es eine wichtige architektonische Anpassung in der ersten Schicht:

* **Problem (XNOR-Vergleich):** Zu Beginn wurden die Eingänge im ersten Layer mittels *XNOR* verglichen. Das führte dazu, dass das Netz primär auf den leeren Hintergrund (Übereinstimmung von ungesetzten Bits) reagierte, anstatt das eigentliche Zeichen zu lernen.
* **Lösung (AND-Vergleich):** Die Logik wurde auf eine strikte *AND*-Verknüpfung umgestellt. Ein Treffer wird jetzt nur gewertet, wenn das Netz dort anschlägt, wo auch im eigentlichen Zeichen aktive Bits gesetzt sind.

---

## 5. Aktueller Stand, Stagnation & KI-Review
Das Netzwerk funktioniert grundsätzlich und lernt. Allerdings zeigt sich nach mittlerweile sehr vielen Trainingsdurchläufen (trotz Variationen bei Klonen, Stagnationsgrenzen und Mutationsraten) ein **Sättigungseffekt (Plateau-Bildung)**. Das Netz bleibt im Training an einem bestimmten Punkt hängen und stagniert dauerhaft.

An diesem Punkt habe ich der KI erstmals die **gesamte Architektur** offengelegt, um Feedback einzuholen. 

### Fazit der KI-Analyse:
* Einige Vorschläge der KI waren für den aktuellen Aufbau unbrauchbar oder hätten das Netz komplett zerstört.
* **Spannender Ansatz (Crossover):** Ein sehr interessanter Tipp ist das Prinzip der **Kreuzung (Crossover)**. Dabei nimmt man die zwei besten Netzwerke einer Generation als "Eltern" und kombiniert deren Gewichte/Bits, um genetische Blockaden zu lösen.
* **Das mathematische Kernproblem (No-Backpropagation):** Die KI hat das mathematische Dilemma bestätigt: Wegen der rein binären Architektur ist **keine Backpropagation** (Fehlerrückführung) möglich. Es gibt keine stetigen Ableitungen (Gradienten), weshalb man Fehler nicht gezielt zurückrechnen kann, um die Stellschrauben exakt zu justieren. Man bleibt auf die evolutionäre Suche angewiesen.
