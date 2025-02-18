# enigma_cli.rs
CLI Enigma Machine written in Rust


        _____________________________________
      /    _______________________________    \
      |                                       |
      |     .---.¦ .---.¦ .---.¦              |
      |     |¹  |¦ |²  |¦ |³  |¦              |
      |     | A |¦ | A |¦ | A |¦              |
      |     |   |¦ |   |¦ |   |¦              |
      |     '---'  '---'  '---'               |
      |---------------------------------------|
      |  [Q] [W] [E] [R] [T] [Z] [U] [I] [O]  |
      |    [A] [S] [D] [F] [G] [H] [J] [K]    |
      |  [P] [Z] [X] [C] [V] [B] [N] [M] [L]  |
      |---------------------------------------|
      |  (Q) (W) (E) (R) (T) (Y) (U) (I) (O)  |
      |    (A) (S) (D) (F) (G) (H) (J) (K)    |
      |  (P) (Z) (X) (C) (V) (B) (N) (M) (L)  |
      '---------------------------------------'

```
git clone https://github.com/thomasmichaelkane/enigma_cli.git

cd enigma_cli

cargo run

```


### Features
- Three rotor and one reflector enigma machine using command line key entries to simulate key presses.
- Ascii animation for rotor rotation and lamp highlighting.
- Enigma style formatting for encrypted message printing.

### Future improvements
- Add **instructional text** on screen
- Flags for **different modes** (no keyboard/no keyboard highlighting and no message display).
- Addition of **plugboard** encryption.
- Addition of two rotors for the selectable **five rotor design**.


