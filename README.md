# Judo

[![Crates.io](https://img.shields.io/crates/v/judo.svg)](https://crates.io/crates/judo)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

```
     ██╗██╗   ██╗██████╗  ██████╗ 
     ██║██║   ██║██╔══██╗██╔═══██╗
     ██║██║   ██║██║  ██║██║   ██║
██   ██║██║   ██║██║  ██║██║   ██║
╚█████╔╝╚██████╔╝██████╔╝╚██████╔╝
 ╚════╝  ╚═════╝ ╚═════╝  ╚═════╝ 
```

A terminal-based todo list application.

## Table of Contents

- [What Judo Looks Like](#what-judo-looks-like)
- [What It Does](#what-it-does)
- [Why Another Todo App](#why-another-todo-app)
- [Installation](#installation)
- [Usage](#usage)
- [Key Bindings](#key-bindings)
- [Data Storage](#data-storage)

## What Judo Looks Like
![](https://github.com/giacomopiccinini/judo/blob/main/assets/judo.png)

## What It Does

Judo (*Just Do It*) is a simple TUI for managing todo lists. You can create multiple lists, add items to them, mark items as complete, and delete items or entire lists when you're done.

The interface shows your lists on the left side and the items from the selected list on the right side. All your data is saved locally on your computer, so your todos persist between sessions.

## Why Another Todo App

**Q: Who needs yet another todo app?**  
A: No one, really.

**Q: Then why did you create Judo in the first place?**  
A: I am often having conversations in Slack, taking notes on todo's and sending them to my private channel. Which looks embarassing, actually. So, there you go. Plus, I wanted to understand how to work with TUIs.

**Q: Why Rust?**  
A: No particular reason other than I wanted to familiarise more with it. No one cares about "blazing fast" performance for such a simple app. 


## Installation

Install Judo using Cargo:

```bash
cargo install judo
```

Then run it with:

```bash
judo
```

## Usage

When you start Judo, you'll see the main interface with two panels:

- **Left panel**: Your todo lists
- **Right panel**: Items from the selected list

Navigate between lists and items using the keyboard. All changes are automatically saved to your local database.

## Key Bindings

### Main Screen

#### List Navigation
| Key | Action |
|-----|--------|
| `w` | Move up in lists |
| `s` | Move down in lists |
| `↑` | Move up in items |
| `↓` | Move down in items |
| `←` | Deselect current item |
| `→` | Select first item in list |

#### Actions
| Key | Action |
|-----|--------|
| `A` | Add new list |
| `a` | Add new item to selected list |
| `D` | Delete selected list |
| `d` | Delete selected item |
| `Enter` | Toggle item completion status |
| `q` | Quit application |

### Add List/Item Screens
| Key | Action |
|-----|--------|
| `Enter` | Save and return to main screen |
| `Esc` | Cancel and return to main screen |
| `Backspace` | Delete last character |

## Data Storage

Your todo lists and items are stored in a local SQLite database on your computer. This means:

- Your data persists between application sessions
- No internet connection required
- Your todos remain private on your machine
- You can backup the database file if needed

The database is created automatically when you first run the application.
