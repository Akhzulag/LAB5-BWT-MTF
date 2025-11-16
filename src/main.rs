// mod bwt;
// mod mtf;
// mod utils;
mod bwt;
mod huffman;
mod lzw;
mod mtf;
mod utils;

use std::io::Result;
// use mtf::LinkedList;

fn fun_mtf(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files/".to_string() + types + "/test" + num + "." + types;
    let test_path_out = "test_files_mtf/".to_string() + types + "/test" + num + ".mtf";
    let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decmtf";
    // mtf::encode(&test_path, &test_path_out)?;
    // println!("Encoded");
    mtf::decode(&test_path_out, &test_path_decode)?;
    println!("Decoded");
    Ok(())
}

fn fun_bwt(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files/".to_string() + types + "/test" + num + "." + types;
    let test_path_out = "test_files_bwt/".to_string() + types + "/test" + num + ".bwt";
    let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decbwt";
    bwt::encode(&test_path, &test_path_out)?;
    println!("Encoded");
    bwt::decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn fun_bwt_sa(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files/".to_string() + types + "/test" + num + "." + types;
    let test_path_out = "test_files_bwtsa/".to_string() + types + "/test" + num + ".bwtsa";
    let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decbwtsa";
    bwt::encode_SA(&test_path, &test_path_out)?;
    println!("Encoded");
    bwt::decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn fun_bwtmtf(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files_bwt/".to_string() + types + "/test" + num + ".bwt";
    let test_path_out = "test_files_bwtmtf/".to_string() + types + "/test" + num + ".bwtmtf";
    let test_path_decode = "test_files_csv/".to_string() + types + "/test" + num + ".dbw";
    mtf::encode(&test_path, &test_path_out)?;
    println!("Encoded");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn test_bwt(types: &str) -> Result<()> {
    println!("type of file: {}", types);
    fun_bwt(types, "1")?;
    fun_bwt(types, "2")?;
    fun_bwt(types, "3")?;
    fun_bwt(types, "4")?;
    fun_bwt(types, "5")?;
    fun_bwt(types, "6")?;
    fun_bwt(types, "7")?;
    fun_bwt(types, "8")?;
    fun_bwt(types, "9")?;
    fun_bwt(types, "10")?;
    Ok(())
}

fn test_bwt_sa(types: &str) -> Result<()> {
    println!("type of file: {}", types);
    // fun_bwt_sa(types, "1")?;
    // fun_bwt_sa(types, "2")?;
    // fun_bwt_sa(types, "3")?;
    // fun_bwt_sa(types, "4")?;
    // fun_bwt_sa(types, "5")?;
    // fun_bwt_sa(types, "6")?;
    // fun_bwt_sa(types, "7")?;
    // fun_bwt_sa(types, "8")?;
    // fun_bwt_sa(types, "9")?;
    fun_bwt_sa(types, "10")?;
    Ok(())
}

fn test_mtf(types: &str) -> Result<()> {
    println!("type of file: {}", types);
    fun_mtf(types, "1")?;
    fun_mtf(types, "2")?;
    fun_mtf(types, "3")?;
    fun_mtf(types, "4")?;
    fun_mtf(types, "5")?;
    fun_mtf(types, "6")?;
    fun_mtf(types, "7")?;
    fun_mtf(types, "8")?;
    fun_mtf(types, "9")?;
    fun_mtf(types, "10")?;
    Ok(())
}

fn test_bwtmtf(types: &str) -> Result<()> {
    println!("type of file: {}", types);
    fun_bwtmtf(types, "1")?;
    fun_bwtmtf(types, "2")?;
    fun_bwtmtf(types, "3")?;
    fun_bwtmtf(types, "4")?;
    fun_bwtmtf(types, "5")?;
    fun_bwtmtf(types, "6")?;
    fun_bwtmtf(types, "7")?;
    fun_bwtmtf(types, "8")?;
    fun_bwtmtf(types, "9")?;
    fun_bwtmtf(types, "10")?;
    Ok(())
}

fn main() -> Result<()> {
    println!("Main");
    // test_bwt_sa("csv")?;

    // println!("BWT:");
    // test_bwt("pdf")?;
    // test_bwt("mov")?;
    // test_bwt("3mf")?;
    // test_bwt("exe")?;
    // test_bwt("csv")?;

    println!("MTF:");
    // test_mtf("pdf")?;
    // test_mtf("mov")?;
    // test_mtf("3mf")?;
    // test_mtf("exe")?;
    test_mtf("csv")?;

    // println!("BWTMTF:");
    // test_bwtmtf("pdf")?;
    // test_bwtmtf("mov")?;
    // test_bwtmtf("3mf")?;
    // test_bwtmtf("exe")?;
    // test_bwtmtf("csv")?;
    Ok(())
}
