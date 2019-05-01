extern crate amethyst;

mod game;

fn main() -> amethyst::Result<()> {
    game::start()?;
    Ok(())
}
