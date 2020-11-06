use std::process;

fn main() {
    if let Err(err) = real_main() {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

fn real_main() -> anyhow::Result<()> {
    Ok(())
}
