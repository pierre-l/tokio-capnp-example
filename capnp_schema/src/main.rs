extern crate tokio;
extern crate tokio_core;
extern crate mio_uds;
extern crate capnp;
extern crate capnpc;
extern crate capnp_futures;
extern crate futures;

mod addressbook_capnp;
mod capnproto_tests;

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("addressbook.capnp")
        .run()
        .unwrap();
}