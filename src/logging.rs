use std::fs::File;
use tracing_subscriber::{fmt, prelude::*};

use color_eyre::Result;
pub fn init() -> Result<()> {
  let writer = File::create("app.log")?;
  let subscriber = fmt::layer().with_writer(writer).compact();

  tracing_subscriber::registry().with(subscriber).init();

  Ok(())
}
