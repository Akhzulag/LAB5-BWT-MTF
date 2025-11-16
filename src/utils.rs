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

pub fn build_sa(text: &[u8]) -> Vec<usize> {
    let n = text.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<i32> = text.iter().map(|&c| c as i32).collect();
    let mut tmp = vec![0i32; n];

    let mut k = 1;
    while k < n {
        sa.sort_by_key(|&i| {
            let r1 = rank[i];
            let r2 = if i + k < n { rank[i + k] } else { -1 };
            (r1, r2)
        });

        tmp[sa[0]] = 0;
        for i in 1..n {
            let a = sa[i - 1];
            let b = sa[i];
            let prev = (rank[a], if a + k < n { rank[a + k] } else { -1 });
            let now = (rank[b], if b + k < n { rank[b + k] } else { -1 });
            tmp[b] = tmp[a] + if prev != now { 1 } else { 0 };
        }

        rank.copy_from_slice(&tmp);
        if rank[sa[n - 1]] == (n as i32 - 1) {
            break;
        }

        k <<= 1;
    }

    sa
}
