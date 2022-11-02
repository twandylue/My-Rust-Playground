use ::std::cmp;

pub fn compute_lcs_len(s1: String, s2: String) -> u32 {
    let mut result: u32 = 0;

    return 0;
}

fn cal(
    r: &mut u32,
    i1: &mut impl Iterator<Item = String>,
    i2: &mut impl Iterator<Item = String>,
) -> u32 {
    let n1 = i1.next();
    let n2 = i2.next();
    if n1.is_none() || n2.is_none() {
        return *r;
    }

    if n1.unwrap() == n2.unwrap() {
        return cal(&mut (*r + 1), i1, i2);
    }

    return cmp::max(cal(r, i1, i2), cal(r, i1, i2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_lcs_len_test() {
        let s1 = String::from("abcd");
        let s2 = String::from("bcd");
        let r = compute_lcs_len(s1, s2);
        assert_eq!(r, 0);
    }
}
