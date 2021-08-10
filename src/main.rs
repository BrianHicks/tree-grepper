use anyhow::{Context, Result};
// use rayon::iter::{ParallelBridge, ParallelIterator};
// use std::path::PathBuf;

mod cli;
mod language;

use cli::Opts;

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let opts = Opts::from_args()
        .context("couldn't get a valid configuration from the command-line options")?;

    println!("{:?}", opts);

    Ok(())
}

// fn walker(opts: &Opts) -> Result<ignore::Walk> {
//     let builder = match opts.paths.split_first() {
//         Some((first, rest)) => {
//             let mut builder = ignore::WalkBuilder::new(first);
//             for path in rest {
//                 builder.add(path);
//             }

//             builder
//         }
//         None => bail!("I need at least one file or directory to walk!"),
//     };

//     // TODO: git ignore, file matching, et cetera

//     Ok(builder.build())
// }
