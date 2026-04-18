**Castli**
Ongoing project of a multiplayer terminal based strategy game.

**How to build and run**
(If you have windows I don't think it will work)
You will need to have the Rust toolchain installed.

git clone <repo-url>
cd <repo-url>

Open a terminal and run the following command from the project root:
cargo run --bin server

Open a second terminal and run the client:
cargo run --bin client

If the client looks messy you can probably fix by lowering the terminal font size.
This is because the game is rendered on a canvas of fixed terminal rows/cols.

**Controls**
l: look
n: create your castle
a: send units
z: toggle zoom
q: quit
enter/esc: toggle on/off selection in inspect panel
arrowkeys: move camera / cursor
CTRL+arrowkeys: (move camera / cursor) x8
1,2,3: right panel tabs
