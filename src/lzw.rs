#![allow(dead_code, unused, non_snake_case)]

use bs::{BitStream, Mode};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};

const CLEAR_CODE: usize = 256;
const END_CODE: usize = 257;

pub fn encode(file_read: &str, file_write: &str) -> Result<()> {
    let mut reader = BufReader::new(File::open(file_read)?);

    let mut dict: HashMap<(u16, Option<u32>), u32> = HashMap::new();
    let mut ds = BitStream::new(file_write, Mode::Write);

    for i in 0..=255 {
        dict.insert((i, None), i as u32);
    }

    dict.insert((256, Some(256)), CLEAR_CODE as u32);
    dict.insert((257, Some(257)), END_CODE as u32);

    let s: u8 = 0;
    let mut c = [0];
    let mut I: Option<u32> = None;
    let mut size = 258;
    let mut write_bit = 9;
    while reader.read_exact(&mut c).is_ok() {
        let c = c[0] as u16;

        if let Some(&index) = dict.get(&(c, I)) {
            I = Some(index);
        } else {
            // print!("({:?},{write_bit},{}) ", I.unwrap(), size);

            ds.write_bit_sequence(&I.unwrap().to_le_bytes(), write_bit);

            dict.insert((c, I), size);
            size += 1;

            if size == (1 << write_bit) {
                write_bit += 1;
            }

            if write_bit == 22 {
                ds.write_bit_sequence(&CLEAR_CODE.to_le_bytes(), write_bit);
                dict.clear();
                for i in 0..=255 {
                    dict.insert((i, None), i as u32);
                }
                dict.insert((256, None), CLEAR_CODE as u32);
                dict.insert((257, None), END_CODE as u32);
                write_bit = 9;
                size = 258;
            }

            I = Some(c as u32);
        }
    }
    // print!("({:?},{write_bit},{}) ", I.unwrap(), dict.len());
    ds.write_bit_sequence(&I.unwrap().to_le_bytes(), write_bit);
    ds.write_bit_sequence(&END_CODE.to_le_bytes(), write_bit);

    ds.close();
    Ok(())
}

fn from_le_bytes_to_u32(seq: &[u8]) -> u32 {
    seq.iter()
        .enumerate()
        .fold(0u32, |acc, (i, &x)| acc | ((x as u32) << (8 * i)))
}

fn get_word(S: &(u8, Option<usize>), dict: &[(u8, Option<usize>)]) -> Vec<u8> {
    let mut out_S: Vec<u8> = Vec::new();
    let mut S = S;
    while let Some(i) = S.1 {
        out_S.push(S.0);
        S = &dict[i];
    }
    out_S.push(S.0);
    out_S.reverse();

    out_S
}

pub fn decode(file_read: &str, file_write: &str) -> Result<()> {
    let mut writer = BufWriter::new(File::create(file_write)?);

    let mut dict: Vec<(u8, Option<usize>)> = Vec::with_capacity(256);

    for i in 0..=255 {
        dict.push((i, None));
    }

    dict.push((0, Some(CLEAR_CODE)));
    dict.push((0, Some(END_CODE)));

    let mut read_bits = 9;
    let mut ds = BitStream::new(file_read, Mode::Read);
    let mut I = from_le_bytes_to_u32(&ds.read_bit_sequence(read_bits)?) as usize;

    let mut S = dict[I];
    writer.write_all(&[S.0]); // write S into

    let (mut old_I, mut old_S) = (I, vec![S.0]);
    let mut size = dict.len();
    let mut mm = 0;
    while let Ok(seq) = ds.read_bit_sequence(read_bits) {
        I = from_le_bytes_to_u32(&seq) as usize;
        if I == CLEAR_CODE {
            dict = Vec::new();
            for i in 0..=255 {
                dict.push((i, None));
            }
            dict.push((0, None));
            dict.push((0, None));
            size = dict.len();
            read_bits = 9;

            I = from_le_bytes_to_u32(&ds.read_bit_sequence(read_bits)?) as usize;

            S = dict[I];
            writer.write_all(&[S.0]);

            old_I = I;
            old_S = vec![S.0];

            continue;
        }
        if I == END_CODE {
            // println!("END_CODE");
            break;
        }

        if I < size {
            S = dict[I];

            old_S = get_word(&S, &dict);

            writer.write_all(&old_S);

            dict.push((old_S[0], Some(old_I)));

            old_I = I;
        } else {
            old_S.push(old_S[0]);
            writer.write_all(&old_S);
            dict.push((old_S[0], Some(old_I)));
            old_I = I;
        }

        size += 1;
        if size + 1 == (1 << read_bits) {
            read_bits += 1;
        }
    }

    writer.flush()?;

    Ok(())
}

fn fun_mtf(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files_mtf/".to_string() + types + "/test" + num + ".mtf";
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".mlzw";
    // let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decmtf";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn fun_bwt(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files_bwt/".to_string() + types + "/test" + num + ".bwt";
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".blzw";
    // let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decbwt";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn fun_bwtmtf(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files_bwtmtf/".to_string() + types + "/test" + num + ".bwtmtf";
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".bmlzw";
    // let test_path_decode = "test_files_csv/".to_string() + types + "/test" + num + ".dbw";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
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

use rayon::prelude::*;
use rayon::scope;

fn run_all_tests() -> Result<()> {
    println!("BWT");
    let files = ["pdf", "mov", "3mf", "exe", "csv"];
    files.par_iter().for_each(|f| {
        test_bwt(f).unwrap(); // всередині відкривається свій файл і пишеться у нього
    });

    println!("MTF");
    files.par_iter().for_each(|f| {
        test_mtf(f).unwrap(); // всередині відкривається свій файл і пишеться у нього
    });
    println!("BWTMTF");
    files.par_iter().for_each(|f| {
        test_bwtmtf(f).unwrap(); // всередині відкривається свій файл і пишеться у нього
    });
    Ok(())
}

fn lens(types: &str, num: &str, algo: &str) -> Result<f32> {
    let test_path = "test_files/".to_string() + types + "/test" + num + "." + types;
    let test_path_out = "test_files/".to_string() + types + "/test" + num + "." + algo;
    let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".dec";
    let metadata = std::fs::metadata(test_path)?;
    let size_test = metadata.len();
    let metadata = std::fs::metadata(test_path_out)?;
    let arch_test = metadata.len();
    // println!(
    //     "[{num}],[{}],[{}],[{}],",
    //     size_test,
    //     arch_test,
    //     size_test as f32 / arch_test as f32
    // );

    Ok(size_test as f32 / arch_test as f32)
}

fn calc(s: &mut f32, max_t: &mut f32, min_t: &mut f32, t: f32) {
    *max_t = max_t.max(t);
    *min_t = min_t.min(t);
    *s += t;
}

fn all_lens(types: &str, algo: &str, cap: &str) {
    // println!("{}.{}", types, algo);
    let mut s = 0.0;
    let mut min_t: f32 = 1000.;
    let mut max_t: f32 = -1000.;
    // println!("#figure(\n table(\n columns: 4,\n table.header[*Файл*][*Початкова \\#байт*][*Cтиснута \\#байт*][*коеф*], ");
    let mut t = lens(types, "1", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "2", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "3", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "4", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "5", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "6", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "7", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "8", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "9", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    t = lens(types, "10", algo).unwrap();
    calc(&mut s, &mut max_t, &mut min_t, t);
    // println!("),\n caption: \"{types} {cap}\",\n)\n");

    println!("[{}],  [{}], [{}],[{}],", types, s / 10., min_t, max_t,);
}

fn fun_lzw(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files/".to_string() + types + "/test" + num + "." + types;
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".lzw";
    // let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decmtf";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}
fn test_lzw(types: &str) -> Result<()> {
    println!("type of file: {}", types);
    fun_lzw(types, "1")?;
    fun_lzw(types, "2")?;
    fun_lzw(types, "3")?;
    fun_lzw(types, "4")?;
    fun_lzw(types, "5")?;
    fun_lzw(types, "6")?;
    fun_lzw(types, "7")?;
    fun_lzw(types, "8")?;
    fun_lzw(types, "9")?;
    fun_lzw(types, "10")?;
    Ok(())
}

fn main() -> Result<()> {
    let files = ["pdf", "mov", "3mf", "exe", "csv"];
    // files.par_iter().for_each(|f| {
    //     test_lzw(f).unwrap(); // всередині відкривається свій файл і пишеться у нього
    // });

    println!("== LZW");
    // println!("---- START MOV ----");
    println!("=== MOV");
    all_lens("mov", "lzw", "");
    all_lens("mov", "blzw", "bwt");
    all_lens("mov", "mlzw", "mtf");
    all_lens("mov", "bmlzw", "bwt+mtf");

    // println!("---- END MOV ----\n");
    // println!("---- START PDF ----");
    println!("=== PDF");
    all_lens("pdf", "lzw", "");
    all_lens("pdf", "blzw", "bwt");
    all_lens("pdf", "mlzw", "mtf");
    all_lens("pdf", "bmlzw", "bwt+mtf");
    // println!("---- END PDF ----\n");
    // println!("---- START CSV ----");
    println!("=== CSV");
    all_lens("csv", "lzw", "");
    all_lens("csv", "blzw", "bwt");
    all_lens("csv", "mlzw", "mtf");
    all_lens("csv", "bmlzw", "bwt+mtf");

    // println!("---- END CSV ----\n");
    // println!("---- START EXE ----");

    println!("=== EXE");
    all_lens("exe", "lzw", "");
    all_lens("exe", "blzw", "bwt");
    all_lens("exe", "mlzw", "mtf");
    all_lens("exe", "bmlzw", "bwt+mtf");
    // println!("---- END EXE ----\n");
    // println!("---- START 3mf ----");

    println!("=== 3MF");
    all_lens("3mf", "lzw", "");
    all_lens("3mf", "blzw", "bwt");
    all_lens("3mf", "mlzw", "mtf");
    all_lens("3mf", "bmlzw", "bwt+mtf");

    println!("#figure(\n table(\n columns: 4,\n table.header[*Тип файлу*][*Середній коеф*][*Мінімальний*][*Максимальний*], ");
    all_lens("mov", "lzw", "");
    all_lens("pdf", "lzw", "");
    all_lens("csv", "lzw", "");
    all_lens("exe", "lzw", "");
    all_lens("3mf", "lzw", "");
    println!("),\n caption: \"lzw\",\n)\n");

    println!("#figure(\n table(\n columns: 4,\n table.header[*Тип файлу*][*Середній коеф*][*Мінімальний*][*Максимальний*], ");
    all_lens("mov", "blzw", "bwt");
    all_lens("pdf", "blzw", "bwt");
    all_lens("csv", "blzw", "bwt");
    all_lens("exe", "blzw", "bwt");
    all_lens("3mf", "blzw", "bwt");
    println!("),\n caption: \"bwt+lzw\",\n)\n");
    println!("#figure(\n table(\n columns: 4,\n table.header[*Тип файлу*][*Середній коеф*][*Мінімальний*][*Максимальний*], ");
    all_lens("mov", "mlzw", "mtf");
    all_lens("pdf", "mlzw", "mtf");
    all_lens("csv", "mlzw", "mtf");
    all_lens("exe", "mlzw", "mtf");
    all_lens("3mf", "mlzw", "mtf");
    println!("),\n caption: \"mtf+lzw\",\n)\n");
    println!("#figure(\n table(\n columns: 4,\n table.header[*Тип файлу*][*Середній коеф*][*Мінімальний*][*Максимальний*], ");
    all_lens("mov", "bmlzw", "bwt+mtf");
    all_lens("pdf", "bmlzw", "bwt+mtf");
    all_lens("csv", "bmlzw", "bwt+mtf");
    all_lens("exe", "bmlzw", "bwt+mtf");
    all_lens("3mf", "bmlzw", "bwt+mtf");
    println!("),\n caption: \" bwt+mtf+lzw\",\n)\n");

    // all_lens("exe", "blzw");
    // all_lens("exe", "mlzw");
    // all_lens("exe", "bmlzw");
    // run_all_tests();
    // println!("BWT");
    // test_bwt("pdf")?;
    // test_bwt("mov")?;
    // test_bwt("3mf")?;
    // test_bwt("exe")?;
    // test_bwt("csv")?;
    // println!("MTF");
    // test_mtf("pdf")?;
    // test_mtf("mov")?;
    // test_mtf("3mf")?;
    // test_mtf("exe")?;
    // test_mtf("csv")?;
    // println!("BWTMTF");
    // test_bwtmtf("pdf")?;
    // test_bwtmtf("mov")?;
    // test_bwtmtf("3mf")?;
    // test_bwtmtf("exe")?;
    // test_bwtmtf("csv")?;
    Ok(())
}
