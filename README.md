# Markown Editor w/ TUI

This project is going through it's third rewrite. It aims to be a tool that allows me to quickly create and write makrdown files, wherever, whenever.
I dislike having to navigate to where the file is, i want to press some shortcuts, type the software, folder and file's name in the termninal and be read to edit it.\
Beyond this, it's also a test of my current capabilities as a developer. My goal is to build all the basic components that would compose a text editor, plus some QOL tools and/or systems.

## Current progress:

| Feature             | State  |
| ------------------- | :----: |
| Create Files        |  TBD   |
| Open/Write Files    | DONE*  |
| Syntax Highlight    |  TBD   |
| Soft/Hard Wrap      |  TBD   |
| Dprint Integration  | TBD**  |
| Tag/Notebook System | TBD*** |

\* : Will be rewritten + Needs a visual indication that the file has been saved.\
\*\* : I'm debating if i should integrate, i do like the ability to quickly fix my tables upon saving.\
\*\*\* : Still debating wheter i should support it, i do want to have some way to setup a main folder for notes and quickly openning the wanted file without having to navigate all the way to it's place.

## Tools:

- Crossterm - Backend
- Ratatui + Tui_textarea - Frontend
- Comrak - Markdown parser
