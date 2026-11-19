# ZuZa

### Beschreibung
ZuZa ist ein Tool, um die Zusatztexte einer Meldergruppe an die Melder einer Esser-Brandmeldeanlage weiterzugeben. Somit wird das manuelle Kopieren in Tools8000 obsolet.

### Anleitung
Zuerst müssen in Tools8000 die Zusatztexte exportiert werden. Hierbei wird eine .csv erstellt, welche dann mit der ZuZa-Anwendung geöffnet werden kann.
Nach einem Klick auf **"Texte ergänzen"** wird die geöffnete Datei auf fehlende Zusatztexte geprüft. Nur dann werden die Texte ergänzt.

**Wichtig:** Hierbei ist zu beachten, dass die .csv im Vorhinein nicht manuell bearbeitet und die Reihenfolge der Zeilen nicht geändert wird.
Der Standard beim Export zeigt zuerst die Meldergruppe und darunter alle dazugehörigen Melder dieser Meldergruppe an. Auf dieser Reihenfolge basiert die Anwendung.

--- 

### Build
für Linux<br>
`cargo build --release`

für Windows <br>
`rustup target add x86_64-pc-windows-gnu`<br>
`cargo build --release --target x86_64-pc-windows-gnu`<br>
