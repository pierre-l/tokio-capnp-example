#[macro_use] extern crate log;
extern crate env_logger;
extern crate tokio;
extern crate tokio_core;
extern crate mio_uds;
extern crate capnp;
extern crate capnpc;
extern crate capnp_futures;
extern crate futures;

mod capnproto_tests;
mod addressbook_capnp;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("addressbook.capnp")
        .run()
        .unwrap();
}