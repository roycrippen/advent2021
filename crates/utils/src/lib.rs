pub fn flatten_zip3<A, B, C>(v: ((A, B), C)) -> (A, B, C) {
    let ((a, b), c) = v;
    (a, b, c)
}

pub fn ones_bit_count(xs: &Vec<u16>, k: usize) -> usize {
    xs.iter().fold(0 as usize, |acc, x| acc + ((*x as usize & (1 << k)) >> k))
}

pub fn from_digits(xs: &Vec<u16>) -> usize {
    let mut num: usize = 0;
    let mut fac = 10_usize.pow((xs.len() - 1) as u32);
    for i in 0..xs.len() {
        num += fac * *xs.get(i).unwrap() as usize;
        fac /= 10;
    }
    num
}

#[derive(Debug)]
pub enum InputType { Sample, Input }

#[cfg(test)]
mod tests {
    use crate::{flatten_zip3, ones_bit_count, from_digits};

    #[test]
    fn test_flatten_zip3() {
        let v = ((1, 'a'), true);
        assert_eq!((1, 'a', true), flatten_zip3(v))
    }

    #[test]
    fn test_bit_count() {
        let xs = vec![1, 2, 3, 4];
        assert_eq!(2, ones_bit_count(&xs, 0));
        assert_eq!(2, ones_bit_count(&xs, 1));
        assert_eq!(1, ones_bit_count(&xs, 2));
    }

     #[test]
    fn test_from_digits() {
         let aaa = from_digits(&vec![5,3,5,3]);
         assert_eq!(5353, aaa);
     }
}
