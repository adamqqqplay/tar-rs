//! An example of test uid and gid in an PAX archive.
//!
//! Please run examples/pax_header/pax_build_tar.sh first to generate PAX archive.
//!
extern crate tar;

use std::fs::{self, File};
use std::os::unix::prelude::MetadataExt;
use std::path::Path;
use tar::Archive;
use tempfile::Builder as TempBuilder;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => panic!("{} returned {}", stringify!($e), e),
        }
    };
}

fn main() {
    let tarlist = ["biguid_gnu.tar", "biguid_pax.tar"];
    let dir = "./examples/pax/";

    for path in &tarlist {
        let file = format!("{}{}", dir, path);
        println!("[INFO] test tar file: {}", file);

        let target_path = Path::new(&file);
        if !target_path.exists() {
            println!("[ERROR] Please run examples/pax/pax_build_tar.sh first");
            panic!();
        }

        let td = t!(TempBuilder::new().prefix("tar-rs").tempdir());
        let rdr = t!(File::open(&file));
        let mut ar = Archive::new(rdr);
        ar.set_preserve_ownerships(true);

        if unsafe { libc::getuid() } == 0 {
            let _ = ar.unpack(td.path());
            let meta = t!(fs::metadata(td.path().join("test.txt")));
            let uid = meta.uid();
            let gid = meta.gid();
            // 4294967294 = u32::MAX - 1
            assert_eq!(uid, 4294967294);
            assert_eq!(gid, 4294967294);
            println!("[INFO] uid and gid test success");
        } else {
            // it's not possible to unpack tar while preserving ownership
            // without root permissions
            println!("[WARN] Please run this test as root user");
            assert!(ar.unpack(td.path()).is_err());
        }

        println!()
    }
}
