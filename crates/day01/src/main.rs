fn main() {
    let xs: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("[imperative ] Day01 part a = {}", part_a_imperative(&xs));
    println!("[functional ] Day01 part a = {}", part_a_functional(&xs));
    println!();
    println!("[imperative ] Day01 part b = {}", part_b_imperative(&xs));
    println!("[functional ] Day01 part b = {}", part_b_functional(&xs));
}

// imperative
fn part_a_imperative(xs: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..xs.len() - 1 {
        if xs[i + 1] > xs[i] {
            count += 1
        };
    }
    count
}

fn part_b_imperative(xs: &[usize]) -> usize {
    assert!(xs.len() > 2);
    let mut ys = Vec::new();
    for i in 0..(xs.len() - 2) {
        ys.push(xs[i] + xs[i + 1] + xs[i + 2]);
    }
    part_a_imperative(&ys)
}

// functional
fn part_a_functional(xs: &Vec<usize>) -> usize {
    let iter1 = xs.iter().take(xs.len() - 1);
    let iter2 = xs.iter().skip(1);

    iter1
        .zip(iter2)
        .fold(0, |acc, (v0, v1)| if v1 > v0 { acc + 1 } else { acc })
}

fn part_b_functional(xs: &Vec<usize>) -> usize {
    /*
        xs =    [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        iter1 = [199, 200, 208, 210, 200, 207, 240, 269]
        iter2 = [200, 208, 210, 200, 207, 240, 269, 260]
        iter3 = [208, 210, 200, 207, 240, 269, 260, 263]
        ys    = [607, 618, 618, 617, 647, 716, 769, 792]
     */

    let iter1 = xs.iter().take(xs.len() - 2);
    let iter2 = xs.iter().skip(1).take(xs.len() - 2);
    let iter3 = xs.iter().skip(2);
    let ys = iter1
        .zip(iter2)
        .zip(iter3)
        .map(|((v1, v2), v3)| v1 + v2 + v3)
        .collect();

    part_a_functional(&ys)
}

#[test]
fn day01_part_a() {
    assert_eq!(7, part_a_imperative(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
    assert_eq!(7, part_a_functional(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
}


#[test]
fn day01_part_b() {
    assert_eq!(5, part_b_imperative(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
    assert_eq!(5, part_b_functional(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
}