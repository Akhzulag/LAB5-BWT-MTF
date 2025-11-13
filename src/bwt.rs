#![allow(dead_code)]
use crate::utils::{build_t, radix_sort};
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

pub fn encode(file_read: &str, file_write: &str) -> Result<()> {
    let mut reader = BufReader::new(File::open(file_read)?);
    let mut writer = BufWriter::new(File::create(file_write)?);

    let mut buf = vec![0; BLOCK];
    let mut i = 0;
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        let mut pos = BLOCK + 1;

        let mut a = build_matrix(&buf, n);
        radix_sort(&mut a, n + 1);
        // a.iter().for_each(|row| {
        //     println!("{:?}", row[0]);
        // });
        let len_r = a[0].len();
        a.iter().enumerate().for_each(|(i, row)| {
            writer.write_all(&[row[len_r - 1]]).expect("BWR: Error");
            if row[0] == 0 {
                pos = i;
            }
        });
        // println!("pos:{pos}");
        // println!("");
        writer.write_all(&[pos as u8]).expect("BWR: Error");
        println!("{i}: size: {}", n + 1);
        i += 1;
    }

    Ok(())
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
        println!("{i}: size: {size}");
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
