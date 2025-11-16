#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::vec;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Result, Seek, SeekFrom, Write},
};

use bs::{BitStream, Mode};

#[derive(Clone, Debug)]

pub struct node {
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
    freq: u32,
}

pub fn build_tree(freq: &[u32]) -> Vec<node> {
    let mut elements = BinaryHeap::new();

    let mut tree = vec![
        node {
            left: None,
            right: None,
            parent: None,
            freq: 0,
        };
        freq.len()
    ];

    freq.iter().enumerate().for_each(|(i, &x)| {
        if x != 0 {
            elements.push(Reverse((x, i)));
        }
        tree[i].freq = x;
    });

    let mut tree_ind = tree.len();
    while elements.len() != 1 {
        let (left, right) = (elements.pop().unwrap().0, elements.pop().unwrap().0);

        let freq_ind = left.0 + right.0;
        tree.push(node {
            left: Some(left.1),
            right: Some(right.1),
            parent: None,
            freq: freq_ind,
        });

        tree[left.1].parent = Some(tree_ind);
        tree[right.1].parent = Some(tree_ind);

        elements.push(Reverse((freq_ind, tree_ind)));
        tree_ind += 1;
    }

    tree
}

pub fn build_freq_table(file_name: &str) -> Result<(Vec<u32>, u32)> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut freq = vec![0; 256];
    let mut byte = [0];
    let mut count_bytes = 0;
    while reader.read_exact(&mut byte).is_ok() {
        freq[byte[0] as usize] += 1;
        count_bytes += 1;
    }

    Ok((freq, count_bytes))
}

fn build_table_code(tree: &[node]) -> Vec<(Vec<u8>, usize)> {
    let mut table_code: Vec<(Vec<u8>, usize)> = vec![(Vec::new(), 0); 256];

    table_code.iter_mut().enumerate().for_each(|(i, x)| {
        let mut child = i;
        let mut code: Vec<u8> = Vec::new();
        while let Some(parent) = tree[child].parent {
            if tree[parent].right.unwrap() == child {
                code.push(1);
            } else {
                code.push(0);
            }
            child = parent;
        }
        code.reverse();

        let seq_code = code
            .chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .fold(0u8, |acc, (i, &b)| acc | (b << i))
            })
            .collect();
        *x = (seq_code, code.len());
    });

    table_code
}

fn write_freq_table(freq: &[u32], count_bytes: u32, file_write: &str) -> Result<File> {
    let file_write = File::create(file_write)?;
    let mut writer = BufWriter::new(&file_write);
    freq.iter()
        .try_for_each(|x| writer.write_all(&x.to_le_bytes()))?;

    writer.write_all(&count_bytes.to_le_bytes())?;
    writer.flush()?;
    drop(writer);

    Ok(file_write)
}

pub fn encode(file_read: &str, file_write: &str) -> Result<()> {
    let (freq, count_bytes) = build_freq_table(file_read)?;
    let file_write = write_freq_table(&freq, count_bytes, file_write)?;
    let mut bs = BitStream::new_file(file_write, Mode::Write);

    let tree = build_tree(&freq);
    let table_code = build_table_code(&tree);
    let mut reader = BufReader::new(File::open(file_read)?);
    let mut byte = [0];
    while reader.read_exact(&mut byte).is_ok() {
        bs.write_bit_sequence(
            &table_code[byte[0] as usize].0,
            table_code[byte[0] as usize].1,
        )?;
    }

    bs.close()?;
    Ok(())
}

fn read_freq_table(file_read: &str) -> Result<(Vec<u32>, u32, File)> {
    let file = File::open(file_read)?;
    let mut reader = BufReader::new(&file);
    let mut freq: Vec<u32> = Vec::with_capacity(256);
    let mut buf = [0u8; 4];
    for _ in 0..256 {
        if reader.read_exact(&mut buf).is_ok() {
            freq.push(u32::from_le_bytes(buf));
        }
    }

    let count_bytes = if reader.read_exact(&mut buf).is_ok() {
        u32::from_le_bytes(buf)
    } else {
        0
    };

    Ok((freq, count_bytes, file))
}

pub fn decode(file_read: &str, file_write: &str) -> Result<()> {
    let (freq, mut count_bytes, _) = read_freq_table(file_read)?;
    let tree = build_tree(&freq);
    let mut file_read = File::open(file_read)?;
    file_read.seek(SeekFrom::Start(1028))?;

    let mut bs = BitStream::new_file(file_read, Mode::Read);
    let mut writer = BufWriter::new(File::create(file_write)?);
    let mut cur_node = tree.len() - 1;

    while let Ok(seq) = bs.read_bit_sequence(1) {
        if let Some(left) = tree[cur_node].left {
            let right = tree[cur_node].right.unwrap();
            cur_node = match seq[0] {
                0 => left,
                1 => right,
                _ => {
                    panic!("Should be only 0 or 1")
                }
            };
            if cur_node < 256 {
                writer.write_all(&[cur_node as u8])?;
                cur_node = tree.len() - 1;
                count_bytes -= 1;
                if count_bytes == 0 {
                    break;
                }
            }
        }
    }
    writer.flush()?;

    Ok(())
}

fn fun_mtf(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files_mtf/".to_string() + types + "/test" + num + ".mtf";
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".mhuf";
    // let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decmtf";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn fun_huf(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files/".to_string() + types + "/test" + num + "." + types;
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".huf";
    // let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decmtf";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn fun_bwt(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files_bwt/".to_string() + types + "/test" + num + ".bwt";
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".bhuf";
    // let test_path_decode = "test_files/".to_string() + types + "/test" + num + ".decbwt";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn fun_bwtmtf(types: &str, num: &str) -> Result<()> {
    let test_path = "test_files_bwtmtf/".to_string() + types + "/test" + num + ".bwtmtf";
    let test_path_out = "test_files/".to_string() + types + "/test" + num + ".bmhuf";
    // let test_path_decode = "test_files_csv/".to_string() + types + "/test" + num + ".dbw";
    encode(&test_path, &test_path_out)?;
    println!("Encoded: {types}.{num}");
    // decode(&test_path_out, &test_path_decode)?;
    // println!("Decoded");
    Ok(())
}

fn test_huf(types: &str) -> Result<()> {
    println!("type of file: {}", types);
    fun_huf(types, "1")?;
    fun_huf(types, "2")?;
    fun_huf(types, "3")?;
    fun_huf(types, "4")?;
    fun_huf(types, "5")?;
    fun_huf(types, "6")?;
    fun_huf(types, "7")?;
    fun_huf(types, "8")?;
    fun_huf(types, "9")?;
    fun_huf(types, "10")?;
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
    let metadata = std::fs::metadata(test_path)?;
    let size_test = metadata.len();
    let metadata = std::fs::metadata(test_path_out)?;
    let arch_test = metadata.len();
    // println!(
    //     "[{num}],[{}],[{}],[{}],[{}],",
    //     size_test,
    //     arch_test,
    //     size_test as f32 / (arch_test ) as f32,
    //     size_test as f32 / (arch_test - 1028) as f32
    // );

    Ok(size_test as f32 / (arch_test - 1028) as f32)
}

fn calc(s: &mut f32, max_t: &mut f32, min_t: &mut f32, t: f32) {
    *max_t = max_t.max(t);
    *min_t = min_t.min(t);
    *s += t;
}

fn all_lens(types: &str, algo: &str,cap: &str) {
    // println!("{}.{}", types, algo);
    let mut s = 0.0;
    let mut min_t: f32 = 1000.;
    let mut max_t: f32 = -1000.;

    // println!("#figure(\n table(\n columns: 5,\n table.header[*Файл*][*Початкова \\#байт*][*Cтиснута \\#байт*][*коеф*][*коеф* з мета], ");
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


    // println!(
    //     "{}.{}: max[{}], min[{}], ave[{}]]",
    //     types,
    //     algo,
    //     max_t,
    //     min_t,
    //     s / 10.0
    // );
}

fn main() -> Result<()> {
    // println!("HUF");

    // run_all_tests();
    // println!("BWT");
    let files = ["pdf", "mov", "3mf", "exe", "csv"];
    // files.par_iter().for_each(|f| {
    //     test_huf(f).unwrap(); // всередині відкривається свій файл і пишеться у нього
    // });


     println!("== HUF");
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
    all_lens("mov", "huf", "");
    all_lens("pdf", "huf", "");
    all_lens("csv", "huf", "");
    all_lens("exe", "huf", "");
    all_lens("3mf", "huf", "");
    println!("),\n caption: \"huf\",\n)\n");

    println!("#figure(\n table(\n columns: 4,\n table.header[*Тип файлу*][*Середній коеф*][*Мінімальний*][*Максимальний*], ");
    all_lens("mov", "bhuf", "bwt");
    all_lens("pdf", "bhuf", "bwt");
    all_lens("csv", "bhuf", "bwt");
    all_lens("exe", "bhuf", "bwt");
    all_lens("3mf", "bhuf", "bwt");
    println!("),\n caption: \"bwt+huf\",\n)\n");
    println!("#figure(\n table(\n columns: 4,\n table.header[*Тип файлу*][*Середній коеф*][*Мінімальний*][*Максимальний*], ");
    all_lens("mov", "mhuf", "mtf");
    all_lens("pdf", "mhuf", "mtf");
    all_lens("csv", "mhuf", "mtf");
    all_lens("exe", "mhuf", "mtf");
    all_lens("3mf", "mhuf", "mtf");
    println!("),\n caption: \"mtf+huf\",\n)\n");
    println!("#figure(\n table(\n columns: 4,\n table.header[*Тип файлу*][*Середній коеф*][*Мінімальний*][*Максимальний*], ");
    all_lens("mov", "bmhuf", "bwt+mtf");
    all_lens("pdf", "bmhuf", "bwt+mtf");
    all_lens("csv", "bmhuf", "bwt+mtf");
    all_lens("exe", "bmhuf", "bwt+mtf");
    all_lens("3mf", "bmhuf", "bwt+mtf");
    println!("),\n caption: \" bwt+mtf+huf\",\n)\n");

    Ok(())
}
