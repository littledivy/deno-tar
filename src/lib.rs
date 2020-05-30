extern crate tar;

use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use futures::future::FutureExt;

use serde::Deserialize;
use serde::Serialize;

use std::io::prelude::*;
use std::fs::File;
use tar::{Archive, Builder};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("archive", op_archive);
}

#[derive(Deserialize)]
struct ArchiveOptions {
    tarname: String,
    files: Vec<String>
}

fn op_archive(
    _interface: &mut dyn Interface,
    data: &[u8],
    _zero_copy: Option<ZeroCopyBuf>,
) -> Op {

    let params: ArchiveOptions = serde_json::from_slice(data).unwrap();

    let file = File::create(params.tarname).unwrap();
    let mut a = Builder::new(file);
    for i in params.files {
       println!("{:#?}", i);
       a.append_path(i).unwrap();
    }
    // a.append_file("file2.txt", &mut File::open("file3.txt").unwrap()).unwrap();
    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}
