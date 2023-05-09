# Übungsblatt 2
## Generelles
Diese beiden Aufgaben dienen dem Einstieg in die Zerlegung von Text mit der [logos](https://crates.io/crates/logos) crate. 
Sie implementieren die lexikalische Analyse für unseren Praktikumsinterpreter der Sprache C1. 

## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, Punktabzug erhalten! 
Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen. 
Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.


## Abgabemodus
Die Lösung ist in einem eigenen Git-Repository abzugeben.
Sie können in ihrer Lösung jedoch beliebige Hilfstypen und Module selbst definieren.
Die grundlegende Struktur des hier mitgegebenen Templates sollte jedoch nicht verändert werden.

Zur Lösung der Aufgaben steht für Sie dieses Repository mit
- vorgegebenen Modulen [c1](src/lexer/c1.rs) und [url](src/lexer/url.rs)
- Testfälle (Integrationstests) innerhalb der Dateien [c1_lexer](tests/c1_lexer.rs) und [url_lexer](tests/url_lexer.rs)

zur Verfügung.
> Sie können die Implementierung mit `cargo test` prüfen. Mit `cargo test -- --nocapture` werden Konsolenausgaben auch bei korrekten Tests angezeigt.

## Aufgabe 1 (50 Punkte)
### Kurzbeschreibung
Implementieren Sie mithilfe von [logos](https://crates.io/crates/logos) einen Scanner, der in einem Eingabestrom bzw. in einer Eingabedatei die Token der Sprache C1 erkennt.

### Aufgabenstellung
Mittels logos sollen sie einen Scanner implementieren, der aus einem Eingabestrom die Token der Sprache C1 extrahiert.

Die Lexik der Sprache C1 befindet sich [hier](https://amor.cms.hu-berlin.de/~kunert/lehre/material/c1-lexic.php). Zusätzlich sind folgende Punkte zu beachten:

- die Implementation befindet sich in der Datei [c1](src/lexer/c1.rs)
- Whitespaces (Leerzeichen, Tabulatoren, Zeilenenden) sollen vom Scanner ignoriert werden
- C- (/* */) und C++- (//) Kommentare sollen überlesen werden
- alle nicht in C1 erlaubten Zeichen (etwa "&") sollen zur Rückgabe der `Error` Variante führen
- wenn Sie den Scanner auf das mitgelieferte C1-Beispielprogramm [demorgan.c1](tests/resources/demorgan.c1) ansetzen, sollte `cargo test -- --nocapture` die in [demorgan_tokens](tests/resources/demorgan_tokens.txt) stehende Ausgabe erzeugen


## Aufgabe 2 (50 Punkte)
### Kurzbeschreibung
Implementieren Sie mit Hilfe von [logos](https://crates.io/crates/logos) einen Scanner, der in einem Eingabestrom bzw. aus einer Eingabedatei URLs und damit verbundene Linkbeschreibungen extrahiert.

### Aufgabenstellung
Der zweite zu implementierende Scanner soll mittels logos aus einem Eingabestrom im xhtml-Format die URLs und die Linktexte extrahieren und ausgeben.

Zur Vereinfachung der Analyse gelten folgende Konventionen:

- der Eingabestrom ist valides xhtml, das keinen CDATA Abschnitt enthält. Die Links haben das Format <a ... href="URL" ... >LINKTEXT</a>, wobei im schließenden Tag nach dem 'a' null oder mehr Leerzeichen und Newlines stehen dürfen. Innerhalb des öffnenden Tags dürfen vor und nach href weitere Attribute auftreten, die überlesen werden
- Newlines innerhalb von LINKTEXT gehören zum Linktext. D.h. alles innerhalb des a-Tags gehört zum Linktext
- zwischen <a ...> und <\/a> treten keine anderen Tags auf
- a-Tags, die kein href-Attribut beinhalten, werden komplett ignoriert
- die Eingabe enthält keinerlei Kommentare

Wenn Sie den Scanner auf [urls.html](tests/resources/urls.html) ansetzen, sollte `cargo test -- --nocapture` die in [url_tokens](tests/resources/url_tokens.txt) stehende Ausgabe erzeugen.
