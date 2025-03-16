use std::net::Ipv4Addr;
use std::num::NonZeroU16;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub group: Ipv4Addr,
    #[arg(value_parser = port_in_range)]
    pub port: NonZeroU16,
}

fn port_in_range(raw_port: &str) -> Result<NonZeroU16, String> {
    let port: usize = raw_port
        .parse()
        .map_err(|_| format!("`{}` isn't a port number", raw_port))?;

    let port = u16::try_from(port).map(NonZeroU16::try_from);

    if let Ok(Ok(port)) = port {
        Ok(port)
    } else {
        Err(format!(
            "port not in range {}-{}",
            NonZeroU16::MIN,
            NonZeroU16::MAX,
        ))
    }
}
