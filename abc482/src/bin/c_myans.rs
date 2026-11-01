use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        q: usize,
    }

    let mut r = HashMap::new();
    let mut c = HashMap::new();

    let mut direction_r = Vec::with_capacity(n);
    let mut direction_c = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            r_in: usize,
            c_in: usize,
        }
        *r.entry(r_in).or_insert(0) += 1;
        *c.entry(c_in).or_insert(0) += 1;
    }

    for (key, value) in &r {
        direction_r.push((*key, *value));
    }
    for (key, value) in &c {
        direction_c.push((*key, *value));
    }

    direction_r.sort_unstable();
    direction_c.sort_unstable();

    let mut ruiseki_r = vec![0; direction_r.len() + 1];
    let mut ruiseki_c = vec![0; direction_c.len() + 1];
    for i in 1..=direction_r.len() {
        ruiseki_r[i] = ruiseki_r[i - 1] + direction_r[i - 1].1;
    }
    for i in 1..=direction_c.len() {
        ruiseki_c[i] = ruiseki_c[i - 1] + direction_c[i - 1].1;
    }

    for _ in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                u: usize,
                d: usize,
            }
            let start = direction_r.partition_point(|&(x, _)| x < u);
            let end = direction_r.partition_point(|&(x, _)| x <= d);
            println!("{}", ruiseki_r[end] - ruiseki_r[start]);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let start = direction_c.partition_point(|&(x, _)| x < l);
            let end = direction_c.partition_point(|&(x, _)| x <= r);

            println!("{}", ruiseki_c[end] - ruiseki_c[start]);
        }
    }
}
