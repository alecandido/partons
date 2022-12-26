use anyhow::Result;
use partons::configs;

fn main() -> Result<()> {
    println!("{:?}", configs::path()?);

    Ok(())
}
