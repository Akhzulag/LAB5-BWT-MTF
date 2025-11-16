#![allow(dead_code)]



use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub enum Mode {
    Read,
    Write,
}

// const BUFFER_SIZE: usize = 1;
const BUFFER_SIZE: usize = 64 * 1024;

pub struct BitStream {
    file: File,
    pub buffer: Vec<u8>,
    index_buf: usize,
    point_bit: usize,
    mode: Mode,
}

impl BitStream {
    pub fn new_file(file: File, mode: Mode) -> Self {
        match mode {
            Mode::Read => {
                let mut a = Self {
                    file,
                    buffer: vec![0; BUFFER_SIZE],
                    index_buf: 0,
                    point_bit: 0,
                    mode: Mode::Read,
                };
                a.read_buf(1).unwrap();
                a
            }
            Mode::Write => Self {
                file,
                buffer: vec![0; BUFFER_SIZE],
                index_buf: 0,
                point_bit: 0,
                mode: Mode::Write,
            },
        }
    }

    pub fn new(file_name: &str, mode: Mode) -> Self {
        match mode {
            Mode::Read => {
                let file = OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(file_name);

                match file {
                    Ok(file) => {
                        let mut a = Self {
                            file,
                            buffer: vec![0; BUFFER_SIZE],
                            index_buf: 0,
                            point_bit: 0,
                            mode: Mode::Read,
                        };
                        a.read_buf(1).unwrap();
                        a
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
            Mode::Write => {
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(file_name)
                    .unwrap();
                Self {
                    file,
                    buffer: vec![0; BUFFER_SIZE],
                    index_buf: 0,
                    point_bit: 0,
                    mode: Mode::Write,
                }
            }
        }
    }

    fn change_file(&mut self, file_name: &str, mode: Option<Mode>) {
        match File::open(file_name) {
            Ok(file) => {
                self.file = file;
                self.point_bit = 0;
                if let Some(mode) = mode {
                    self.mode = mode;
                }
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    fn read_buf(&mut self, bit_len: usize) -> std::io::Result<()> {
        let readed_size = self.file.read(&mut self.buffer)?;
        self.buffer.truncate(readed_size);
        self.index_buf = 0;
        let bytes = (bit_len as f32 / 8.0).ceil() as usize;
        if bytes > self.buffer.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Задана довжина послідовності не відповідає обсягу даних, що залишилися у файлі.",
            ));
        }

        Ok(())
    }

    pub fn read_bit_sequence(&mut self, mut bit_len: usize) -> std::io::Result<Vec<u8>> {
        let bytes = (bit_len as f32 / 8.0).ceil() as usize;
        let mut seq: Vec<u8> = vec![0; bytes];
        if self.index_buf == BUFFER_SIZE {
            self.read_buf(bit_len)?;
        }

        if self.buffer.len() != BUFFER_SIZE && self.index_buf == self.buffer.len() - 1 {
            let remain_bit = (8 - self.point_bit) + (self.buffer.len() - 1 - self.index_buf) * 8;

            if remain_bit < bit_len {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Задана довжина послідовності не відповідає обсягу даних, що залишилися у файлі.",
                ));
            }
        }
        if self.index_buf == self.buffer.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Задана довжина послідовності не відповідає обсягу даних, що залишилися у файлі.",
            ));
        }
        match self.mode {
            Mode::Write => {
                println!("Даний бітовий потік може лише зчитувати дані з файла");
            }
            Mode::Read => {
                let mut i = 0;
                let copy = self.point_bit;
                self.point_bit = (self.point_bit + bit_len) % 8;
                while bit_len >= 8 {
                    seq[i] = self.buffer[self.index_buf] >> copy;
                    self.index_buf += 1;

                    if self.index_buf == BUFFER_SIZE {
                        self.read_buf(bit_len)?;
                    }

                    if copy != 0 {
                        seq[i] |= self.buffer[self.index_buf] << (8 - copy);
                    }

                    bit_len -= 8;
                    i += 1;
                }

                if bit_len != 0 {
                    let shift = bit_len + copy;
                    if shift > 8 {
                        seq[i] = self.buffer[self.index_buf] >> copy;
                        self.index_buf += 1;

                        if self.index_buf == BUFFER_SIZE {
                            self.read_buf(bit_len)?;
                        }

                        if copy != 0 {
                            seq[i] |= (self.buffer[self.index_buf]
                                & (0xff >> (8 - self.point_bit)))
                                << (8 - copy);
                        }
                    } else {
                        seq[i] = (self.buffer[self.index_buf] >> copy) & (0xff >> (8 - bit_len));
                        if shift == 8 {
                            self.index_buf += 1;
                        }
                    }
                }
            }
        }

        Ok(seq)
    }

    pub fn write_bit_sequence(&mut self, seq: &[u8], mut bit_len: usize) -> std::io::Result<()> {
        if self.index_buf == BUFFER_SIZE {
            self.file.write_all(&self.buffer)?;
            self.buffer = vec![0; BUFFER_SIZE];
            self.index_buf = 0;
        }

        match self.mode {
            Mode::Read => {
                println!("Даний бітовий потік може лише записувати дані в файл");
            }
            Mode::Write => {
                let mut i = 0;
                let copy = self.point_bit;

                while bit_len >= 8 {
                    self.buffer[self.index_buf] |= seq[i] << copy;
                    self.index_buf += 1;

                    if self.index_buf == BUFFER_SIZE {
                        self.file.write_all(&self.buffer)?;
                        self.buffer = vec![0; BUFFER_SIZE];
                        self.index_buf = 0;
                    }

                    if copy != 0 {
                        self.buffer[self.index_buf] |= seq[i] >> (8 - copy);
                    }
                    bit_len -= 8;
                    i += 1;
                }

                if bit_len != 0 {
                    let shift = bit_len + copy;
                    self.point_bit = (self.point_bit + bit_len) % 8;
                    if shift > 8 {
                        self.buffer[self.index_buf] |= seq[i] << copy;
                        self.index_buf += 1;

                        if self.index_buf == BUFFER_SIZE {
                            self.file.write_all(&self.buffer)?;
                            self.buffer = vec![0; BUFFER_SIZE];
                            self.index_buf = 0;
                        }

                        self.buffer[self.index_buf] |=
                            (seq[i] >> (8 - copy)) & (0xff >> (8 - self.point_bit));
                    } else {
                        self.buffer[self.index_buf] |= (seq[i] & (0xff >> (8 - bit_len))) << copy;
                        if shift == 8 {
                            self.index_buf += 1;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        if self.point_bit == 0 {
            self.buffer.truncate(self.index_buf);
        } else {
            self.buffer.truncate(self.index_buf + 1)
        }

        // println!("buffer {:?}", self.buffer);
        self.file.write_all(&self.buffer)?;
        self.buffer = vec![0; BUFFER_SIZE];
        self.point_bit = 0;
        self.index_buf = 0;
        Ok(())
    }
}
