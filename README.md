<img src="https://github.com/Basicprogrammer10/makey-midi/assets/50306817/2ab4b7b1-139a-4319-b1c1-7918ae0be397" width="30%" align="right"></img>

# makey-midi [![Build](https://github.com/Basicprogrammer10/makey-midi/actions/workflows/rust.yml/badge.svg)](https://github.com/Basicprogrammer10/makey-midi/actions/workflows/rust.yml)

Lets you use a [Makey Makey](https://makeymakey.com) to send midi commands!
When run, this program will load the config file and create a virtual midi out port named 'makey-midi'.
Pressing the keys defined in the config file will then send their respective note values.

## Config

Below is an example `config.toml` file.
The `channel` defined what channel the midi events are sent on.
The `keymap` is the main part where you map keyboard keys to midi notes.
A list of all possible key values is in the dropdown below and a table of midi notes can be found [here](https://www.inspiredacoustics.com/en/MIDI_note_numbers_and_center_frequencies).

```toml
channel = 0
keymap = [
    { key = 'KeyW', note = 59 },       # B
    { key = 'UpArrow', note = 60 },    # C
    { key = 'DownArrow', note = 62 },  # D
    { key = 'LeftArrow', note = 64 },  # E
    { key = 'RightArrow', note = 65 }, # F
    { key = 'Space', note = 67 },      # G
    { key = 'KeyM', note = 69 },       # A
]
```

<details>
<summary>Key List (From <a href="https://docs.rs/rdev/latest/rdev/enum.Key.html">rdev</a>)</summary>

- Alt
- AltGr
- Backspace
- CapsLock
- ControlLeft
- ControlRight
- Delete
- DownArrow
- End
- Escape
- F1
- F10
- F11
- F12
- F2
- F3
- F4
- F5
- F6
- F7
- F8
- F9
- Home
- LeftArrow
- MetaLeft
- MetaRight
- PageDown
- PageUp
- Return
- RightArrow
- ShiftLeft
- ShiftRight
- Space
- Tab
- UpArrow
- PrintScreen
- ScrollLock
- Pause
- NumLock
- BackQuote
- Num1
- Num2
- Num3
- Num4
- Num5
- Num6
- Num7
- Num8
- Num9
- Num0
- Minus
- Equal
- KeyQ
- KeyW
- KeyE
- KeyR
- KeyT
- KeyY
- KeyU
- KeyI
- KeyO
- KeyP
- LeftBracket
- RightBracket
- KeyA
- KeyS
- KeyD
- KeyF
- KeyG
- KeyH
- KeyJ
- KeyK
- KeyL
- SemiColon
- Quote
- BackSlash
- IntlBackslash
- KeyZ
- KeyX
- KeyC
- KeyV
- KeyB
- KeyN
- KeyM
- Comma
- Dot
- Slash
- Insert
- KpReturn
- KpMinus
- KpPlus
- KpMultiply
- KpDivide
- Kp0
- Kp1
- Kp2
- Kp3
- Kp4
- Kp5
- Kp6
- Kp7
- Kp8
- Kp9
- KpDelete
- Function

</details>
