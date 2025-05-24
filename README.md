# TermPlanner

TermPlanner is a minimal terminal-based task planner written in Rust (TUI).

## Current Features

- **View Tasks**  
- **Add Task** (with date & description)
- **Mark Task as done**
- **Edit tasks**
- **Delete tasks**
- **Notifications when tasks are due**
- **systemd service for background notifications (annoyingly)**

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed  
- [dunst](https://dunst-project.org/download) installed (for notifications)

## Usage & Navigation

* **Arrow keys**: move between menu options
* **→ (Right arrow)**: select or confirm an option
* **Enter**: also selects or confirms
* **← (Left arrow)**: go back to the previous menu
* **`x`**: exit the application

## Local development

1. Clone this repository:

   ```bash
   git clone https://github.com/Mateus-Lacerda/term_planner.git
   cd term_planner
   ```

2. Build and run:

   ```bash
   cargo run
   ```

3. Add some cool feature!

## Manual Installation (per-user)

1. **Clone the repository**  
   ```bash
   git clone https://github.com/Mateus-Lacerda/term_planner.git
   cd term_planner

2. **Run the installer script**

   ```bash
   ./install.sh
   ```

   This will:

   * Build the project in release mode (`cargo build --release`)
   * Install the binary to `~/.local/bin/term_planner`
   * Copy the systemd user units to `~/.config/systemd/user/`
   * Reload the user systemd daemon and enable the timer to run every minute

3. **Verify the timer**

   ```bash
   systemctl --user status term_planner-notify.timer
   ```

   To view recent notification logs:

   ```bash
   journalctl --user -u term_planner-notify.service -n 20
   ```

4. **Uninstall**

   ```bash
   ./uninstall.sh
   ```

---

## AUR Installation

1. **Install build dependencies**

   ```bash
   sudo pacman -S --needed base-devel rust
   ```

2. **Build and install via `makepkg`**

   ```bash
   makepkg -si
   ```

   This will:

   * Clone the repository
   * Compile and package the binary
   * Install `/usr/bin/term_planner` and the user units in `/usr/lib/systemd/user/`

3. **Enable the user timer**

   ```bash
   systemctl --user daemon-reload
   systemctl --user enable --now term_planner-notify.timer
   ```

   Check its status with:

   ```bash
   systemctl --user status term_planner-notify.timer
   ```



## TODO

* Filter and search tasks
* AI?
