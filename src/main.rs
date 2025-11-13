mod bwt;
mod mtf;
mod utils;

use mtf::LinkedList;

use crate::utils::radix_sort;

fn main() {
     // bwt::encode("test_bwt.txt", "test.bwt");
    // println!("decoded");
    // bwt::decode("test.bwt", "test22.dec");
    // println!("decoded");
    // bwt::encode("test8.csv", "test_t.bwt");
    // println!("decoded");
    // bwt::decode("test_t.bwt", "test8.decbwt");
    // println!("decoded");

    mtf::encode("test2.csv", "test.bwt");
    println!("decoded");
    // mtf::decode("test.mtf", "test8.decmtf");
    // println!("decoded");
}
