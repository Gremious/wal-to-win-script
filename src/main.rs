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

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
	/// File to process
	#[structopt(name = "INPUT", parse(from_os_str))]
	input: PathBuf,

	/// Output file
	#[structopt(name = "OUTPUT", parse(from_os_str))]
	output: PathBuf,
}

fn main() {
	let opt = Opt::from_args();
	let json_string = fs::read_to_string(&opt.input).expect("Something went wrong reading the file");
	let pywal: Pywal = serde_json::from_str(&json_string).unwrap();

	let win: WinTerm = WinTerm {
		name: opt.input.file_stem().unwrap().to_str().unwrap().to_string(),
		cursor_color: pywal.special.cursor.clone(),
		selection_background: pywal.special.foreground.clone(),
		background: pywal.special.background.clone(),
		foreground: pywal.special.foreground.clone(),
		black: pywal.colors.color0.clone(),
		blue: pywal.colors.color1.clone(),
		cyan: pywal.colors.color2.clone(),
		green: pywal.colors.color3.clone(),
		purple: pywal.colors.color4.clone(),
		red: pywal.colors.color5.clone(),
		white: pywal.colors.color6.clone(),
		yellow: pywal.colors.color7.clone(),
		bright_black: pywal.colors.color8.clone(),
		bright_blue: pywal.colors.color9.clone(),
		bright_cyan: pywal.colors.color10.clone(),
		bright_green: pywal.colors.color11.clone(),
		bright_purple: pywal.colors.color12.clone(),
		bright_red: pywal.colors.color13.clone(),
		bright_white: pywal.colors.color14.clone(),
		bright_yellow: pywal.colors.color15.clone(),
	};
}
