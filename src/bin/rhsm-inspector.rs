extern crate rhsm;

use rhsm::*;

fn main() {
    let cert = EntitlementCertificate::from_file("/home/brain/Projects/upstream/librhsm/426490770692925900.pem",);
    println!("{:?}", cert);
}
