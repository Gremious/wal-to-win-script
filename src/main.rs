//! ```cargo
//! [dependencies]
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! structopt = "0.3"
//! log = "0.4"
//! simple_logger = "1.11"
//! ```

#![warn(
clippy::all,
clippy::pedantic,
)]

use serde::{Serialize, Deserialize};

use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use simple_logger::SimpleLogger;

//{
//     "wallpaper": "D:\\Libraries\\Projects\\Programming\\Rust\\ColorScript\\src\\illusion_by_gydw1n_de3eb5i.png",
//     "alpha": "100",
//     "special": {
//         "background": "#252439",
//         "foreground": "#c8c1e3",
//         "cursor": "#c8c1e3"
//     },
//     "colors": {
//         "color0": "#252439",
//         "color1": "#8B77C5",
//         "color2": "#6D93DE",
//         "color3": "#FC9493",
//         "color4": "#F19FA6",
//         "color5": "#F594AF",
//         "color6": "#DEB4BA",
//         "color7": "#c8c1e3",
//         "color8": "#8c879e",
//         "color9": "#8B77C5",
//         "color10": "#6D93DE",
//         "color11": "#FC9493",
//         "color12": "#F19FA6",
//         "color13": "#F594AF",
//         "color14": "#DEB4BA",
//         "color15": "#c8c1e3"
//     }
// }
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

//{
//   "name": "illusion_theme",
//   "cursorColor": "#c8c1e3",
//   "selectionBackground": "#c8c1e3",
//   "background": "#252439",
//   "foreground": "#c8c1e3",
//   "black": "#252439",
//   "blue": "#8B77C5",
//   "cyan": "#6D93DE",
//   "green": "#FC9493",
//   "purple": "#F19FA6",
//   "red": "#F594AF",
//   "white": "#DEB4BA",
//   "yellow": "#c8c1e3",
//   "brightBlack": "#8c879e",
//   "brightBlue": "#8B77C5",
//   "brightCyan": "#6D93DE",
//   "brightGreen": "#FC9493",
//   "brightPurple": "#F19FA6",
//   "brightRed": "#F594AF",
//   "brightWhite": "#DEB4BA",
//   "brightYellow": "#c8c1e3"
// }
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
	/// Verbose mode
	#[structopt(short, long)]
	verbose: bool,

	/// File to process
	#[structopt(name = "INPUT", parse(from_os_str))]
	input: PathBuf,

	/// Output file. If left out, will create a *_wt.json in the same directory as input.
	#[structopt(name = "OUTPUT", parse(from_os_str))]
	output: Option<PathBuf>,
}

fn main() {
	SimpleLogger::new().init().unwrap();
	let opt = Opt::from_args();
	let json_string = fs::read_to_string(&opt.input).expect("Something went wrong reading the file");
	let pywal: Pywal = serde_json::from_str(&json_string).expect("Not a valid json");
	if opt.verbose { log::info!("{:#?}", &pywal); }

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
		bright_yellow: pywal.colors.color15,
	};

	let win_js = serde_json::to_string_pretty(&win).unwrap();

	let output_dir = if let Some(output_dir) = &opt.output { output_dir.clone() } else {
		let mut output_dir = opt.input.clone();
		if let Ok(mut file_name) = opt.input.file_stem().expect("Invalid file name/no file selected.").to_os_string().into_string() {
			file_name = format!("{}{}", file_name, "_wt.json");
			output_dir.set_file_name(&file_name);
			output_dir
		} else {
			panic!("Invalid/non-unicode input path.");
		}
	};

	if opt.verbose { log::info!("output_dir {:#?}", output_dir); }

	fs::write(output_dir, win_js).expect("Unable to write file");
	if opt.verbose { log::info!("{:#?}", &win); }
}
