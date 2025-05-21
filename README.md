# TermPlanner

TermPlanner is a minimal terminal-based task planner written in Rust (TUI).

## Current Features

- **View Tasks**  
- **Add Task** (with date & description)
- **Mark Task as done**
- **Edit tasks**

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed  
- Create the data file before running:

  ```bash
  touch tasks.json

## Installation & Run

1. Clone this repository:

   ```bash
   git clone https://github.com/Mateus-Lacerda/term_planner.git
   cd term_planner
   ```

2. Create the JSON storage file:

   ```bash
   echo "{\"tasks\":[]}" >> tasks.json
   ```

3. Build and run:

   ```bash
   cargo run
   ```

## Usage & Navigation

* **Arrow keys**: move between menu options
* **→ (Right arrow)**: select or confirm an option
* **Enter**: also selects or confirms
* **← (Left arrow)**: go back to the previous menu
* **`x`**: exit the application

## Data Storage

Tasks are stored in `tasks.json` with this structure:

```json
{
  "tasks": [
    {
      "description": "Your task description",
      "due_date": "YYYY-MM-DD HH:MM",
      "index": 0
    }
  ]
}
```

## TODO

* Remove tasks
* Filter and search tasks
* Nerd fonts for nerds
* AI?
