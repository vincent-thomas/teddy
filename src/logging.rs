use std::fs::File;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use color_eyre::Result;
pub fn init() -> Result<()> {
  let writer = File::create("app.log")?;
  let subscriber = tracing_subscriber::fmt::layer().with_writer(writer).compact();

  tracing_subscriber::registry().with(subscriber).init();

  Ok(())
}
