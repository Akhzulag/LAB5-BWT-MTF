#![allow(dead_code)]
use crate::utils::{build_sa, build_t, radix_sort};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};

const BLOCK: usize = 256;

fn build_matrix(bytes: &[u8], n: usize) -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; n + 1]; n];

    matrix.iter_mut().enumerate().for_each(|(i, row)| {
        row[0] = i as u8;
        for j in 0..n {
            row[(j + i) % n + 1] = bytes[j];
        }
    });

    matrix
}
use std::time::Instant;

// pub fn encode(file_read: &str, file_write: &str) -> Result<()> {
//     let mut reader = BufReader::new(File::open(file_read)?);
//     let mut writer = BufWriter::new(File::create(file_write)?);
//
//     let mut buf = vec![0; BLOCK];
//     let mut i = 0;
//     while let Ok(n) = reader.read(&mut buf) {
//         if n == 0 {
//             break;
//         }
//         let mut pos = BLOCK + 1;
//
//         let start = Instant::now();
//         let mut a = build_matrix(&buf, n);
//         radix_sort(&mut a, n + 1);
//         let len_r = a[0].len();
//         a.iter().enumerate().for_each(|(i, row)| {
//             writer.write_all(&[row[len_r - 1]]).expect("BWR: Error");
//             if row[0] == 0 {
//                 pos = i;
//             }
//         });
//
//         writer.write_all(&[pos as u8]).expect("BWR: Error");
//         let duration = start.elapsed();
//
//         i += 1;
//         // println!("{i}");
//         // println!("Time: {:?}", duration);
//     }
//
//     Ok(())
// }

const BATCH_SIZE: usize = 10000; // кількість блоків в одній порції

pub fn encode(file_read: &str, file_write: &str) -> Result<()> {
    let mut reader = BufReader::new(File::open(file_read)?);
    let mut writer = BufWriter::new(File::create(file_write)?);

    let mut buf = vec![0; BLOCK];
    let start = Instant::now();
    loop {
        // ---------------------------------------------------------
        // 1) Збираємо порцію блоків
        // ---------------------------------------------------------
        let mut batch: Vec<Vec<u8>> = Vec::with_capacity(BATCH_SIZE);

        for _ in 0..BATCH_SIZE {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            batch.push(buf[..n].to_vec());
        }

        // Якщо порція пуста — кінець файлу
        if batch.is_empty() {
            break;
        }

        // ---------------------------------------------------------
        // 2) ПАРАЛЕЛЬНА обробка цієї порції
        // ---------------------------------------------------------
        let results: Vec<(Vec<u8>, u8)> = batch
            .par_iter()
            .map(|block| {
                let n = block.len();

                // 1) matrix
                let mut a = build_matrix(block, n);

                // 2) radix sort
                radix_sort(&mut a, n + 1);

                // 3) BWT + primary
                let mut bwt = Vec::with_capacity(n);
                let mut primary = 0usize;

                for (i, row) in a.iter().enumerate() {
                    bwt.push(row[n]); // останній символ

                    if row[0] == 0 {
                        primary = i;
                    }
                }

                (bwt, primary as u8)
            })
            .collect();

        // ---------------------------------------------------------
        // 3) ПОСЛІДОВНИЙ запис результатів порції
        // ---------------------------------------------------------
        for (bwt, primary) in results {
            writer.write_all(&bwt)?;
            writer.write_all(&[primary])?;
        }
    }
    println!("time: {:?}", start.elapsed());

    Ok(())
}

use rayon::prelude::*;

pub fn encode_SA(file_read: &str, file_write: &str) -> Result<()> {
    let mut reader = BufReader::new(File::open(file_read)?);
    let mut writer = BufWriter::new(File::create(file_write)?);

    let mut block = vec![0; BLOCK];

    let start = Instant::now();
    loop {
        // --- 1. Збираємо порцію блоків ---
        let mut batch: Vec<Vec<u8>> = Vec::with_capacity(BATCH_SIZE);

        for _ in 0..BATCH_SIZE {
            // ← порція (регулюється)
            let n = reader.read(&mut block)?;
            if n == 0 {
                break;
            }
            batch.push(block[..n].to_vec());
        }

        if batch.is_empty() {
            break;
        }

        // --- 2. ПАРАЛЕЛЬНА обробка порції ---
        let results: Vec<(Vec<u8>, u8)> = batch.par_iter().map(|t| encode_block(t)).collect();

        // ВСІ ПОТОКИ ТУТ УЖЕ ЗАВЕРШЕНІ

        // --- 3. ПОСЛІДОВНИЙ запис ---
        for (bwt, primary) in results {
            writer.write_all(&bwt)?;
            writer.write_all(&[primary])?;
        }
    }
    println!("time: {:?}", start.elapsed());

    Ok(())
}

fn encode_block(text: &[u8]) -> (Vec<u8>, u8) {
    let n = text.len();

    // подвоєння
    let mut doubled = Vec::with_capacity(2 * n);
    doubled.extend_from_slice(text);
    doubled.extend_from_slice(text);

    // SA
    let sa2 = build_sa(&doubled);

    // BWT
    let mut bwt = Vec::with_capacity(n);
    let mut primary = 0usize;
    let mut cnt = 0;

    for &p in sa2.iter() {
        if p < n {
            let prev = if p == 0 { n - 1 } else { p - 1 };
            bwt.push(text[prev]);

            if p == 0 {
                primary = cnt;
            }

            cnt += 1;
            if cnt == n {
                break;
            }
        }
    }

    (bwt, primary as u8)
}

pub fn decode(file_read: &str, file_write: &str) -> Result<()> {
    let mut reader = File::open(file_read)?;
    let mut writer = BufWriter::new(File::create(file_write)?);

    let mut buf = vec![0; BLOCK + 1];
    let mut res: Vec<u8> = vec![0; BLOCK];

    let mut i = 0;
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        let size = n - 1;
        // println!("{i}: size: {size}");
        let t = build_t(&buf[0..size]);
        let mut pos = buf[size] as usize;
        res.iter_mut().for_each(|r| {
            pos = t[pos];
            *r = buf[pos];
        });
        writer.write_all(&res[0..size])?;
        i += 1;
    }

    Ok(())
}
