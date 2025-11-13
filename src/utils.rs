pub fn build_t(array: &[u8]) -> Vec<usize> {
    let k = 256;
    let mut c = vec![0; k];

    array.iter().for_each(|x| {
        c[*x as usize] += 1;
    });

    for i in 1..k {
        c[i] += c[i - 1];
    }

    let mut t: Vec<usize> = vec![0; array.len()];
    for (i, el) in array.iter().enumerate().rev() {
        let el = *el as usize;
        t[c[el] - 1] = i;
        c[el] -= 1;
    }

    t
}

pub fn radix_sort(array: &mut Vec<Vec<u8>>, block: usize) {
    let k = 256;
    let mut c = vec![0; k];

    array.iter().for_each(|row| {
        c[row[1] as usize] += 1;
    });

    for i in 1..k {
        c[i] += c[i - 1];
    }

    for col_ind in (1..block).rev() {
        let mut b: Vec<Vec<u8>> = vec![Vec::new(); array.len()];
        let mut c = c.clone();
        for row in array.iter_mut().rev() {
            let el = row[col_ind] as usize;
            std::mem::swap(&mut b[c[el] - 1], row);
            c[el] -= 1;
        }
        *array = b;
    }
}
