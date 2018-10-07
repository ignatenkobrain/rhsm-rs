extern crate rhsm;

use rhsm::*;

fn main() {
    let cert = EntitlementCertificate::from_file("/home/brain/Projects/upstream/rhsm-rs/ent_cert_to_import.pem");
    println!("{:?}", cert);
}
