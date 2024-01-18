mod gen_pass_w_sym;
mod gen_pass_wo_sym;

use clap::{Arg, ArgAction, Command};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use gen_pass_w_sym::gen_password_with_symbols;
use gen_pass_wo_sym::gen_password_without_symbols;
use std::u8;

fn main() {
    let app = Command::new("pgen")
        .version("1.0")
        .about("Generating passwords")
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(u8))
                .default_value("12"),
        )
        .arg(
            Arg::new("symbols")
                .short('s')
                .long("symbols")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    let password_length = *app.get_one::<u8>("length").unwrap();
    let symbols: bool = *app.get_one::<bool>("symbols").unwrap();

    let password = if symbols {
        gen_password_with_symbols(password_length, &mut rand::thread_rng())
    } else {
        gen_password_without_symbols(password_length, &mut rand::thread_rng())
    };

    clipboard.set_contents(password.to_owned()).unwrap();
    println!("📋Password copied to clipboard!📋");
}
