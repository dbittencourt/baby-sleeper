mod app;
mod audio;
mod clock;
mod formatting;
mod input;
mod session;
mod terminal;

fn main() -> anyhow::Result<()> {
    app::run()
}
