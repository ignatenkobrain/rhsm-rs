extern crate base64;
extern crate flate2;
extern crate iter_read;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use iter_read::IterRead;

const ENTITLEMENT_DATA_HEADER: &'static str = "-----BEGIN ENTITLEMENT DATA-----";
const ENTITLEMENT_DATA_FOOTER: &'static str = "-----END ENTITLEMENT DATA-----";

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitlementCertificate {
    consumer: String,
    quantity: u64,
}

impl EntitlementCertificate {
    pub fn from_file(file: &str) -> Self {
        let path = Path::new(file);

        let rdr = BufReader::new(File::open(path).unwrap());
        let b64iter = rdr.lines()
            .skip_while(|l| l.as_ref().unwrap() != ENTITLEMENT_DATA_HEADER)
            .skip(1)
            .take_while(|l| l.as_ref().unwrap() != ENTITLEMENT_DATA_FOOTER);
        let mut b64rdr = IterRead::new(b64iter);

        let mut data = String::new();
        b64rdr.read_to_string(&mut data).unwrap();
        let zlib_encoded = base64::decode(&data).unwrap();
        let foo = &*zlib_encoded;

        use flate2::FlateReadExt;
        foo.zlib_decode();
        let decompressor = flate2::read::ZlibDecoder::new(&*zlib_encoded);

        serde_json::from_reader(decompressor).unwrap()
    }
}
