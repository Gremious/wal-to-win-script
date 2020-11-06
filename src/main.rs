//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! structopt = "0.3"
//! ```
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
struct Pywal {
	special: Special,
	colors: Colors,
}

#[derive(Serialize, Deserialize, Debug)]
struct Special {
	background: String,
	foreground: String,
	cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Colors {
	color0: String,
	color1: String,
	color2: String,
	color3: String,
	color4: String,
	color5: String,
	color6: String,
	color7: String,
	color8: String,
	color9: String,
	color10: String,
	color11: String,
	color12: String,
	color13: String,
	color14: String,
	color15: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WinTerm {
	name: String,
	cursor_color: String,
	selection_background: String,
	background: String,
	foreground: String,
	black: String,
	blue: String,
	cyan: String,
	green: String,
	purple: String,
	red: String,
	white: String,
	yellow: String,
	bright_black: String,
	bright_blue: String,
	bright_cyan: String,
	bright_green: String,
	bright_purple: String,
	bright_red: String,
	bright_white: String,
	bright_yellow: String,
}

fn main() {
	let path = Path::new("D:/Libraries/Projects/Programming/Rust/ColorScript/src/illusion_theme.json").to_path_buf();
	let json_string = fs::read_to_string(path).expect("Something went wrong reading the file");
	let deserialized: Pywal = serde_json::from_str(&json_string).unwrap();
	println!("{:#?}", deserialized);
}
