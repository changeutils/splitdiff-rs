//!
//! The `splitdiff` binary.
//!

use std::{fs, io};

#[derive(Debug)]
enum Error {
    Reading(io::Error),
    Processing(splitdiff_rs::Error),
}

fn main() -> Result<(), Error> {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("patch")
                .help("The patch")
                .index(1)
                .value_name("PATCH")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let patch = args.value_of("patch").unwrap();
    let patch = fs::read_to_string(patch).map_err(Error::Reading)?;

    let splitdiff = splitdiff_rs::SplitDiff::new(&patch);
    let patch_data = splitdiff.process().map_err(Error::Processing)?;
    for (i, (path, patches)) in patch_data.0.iter().enumerate() {
        println!("File {}: {}", i, path);
        for (j, patch) in patches.iter().enumerate() {
            println!("Patch {}:", j);
            for line in patch.iter() {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
