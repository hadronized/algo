fn solve(input: &str) {
  if input.len() < 4 {
    return;
  }

  let mut counts: [usize; 256] = [0; 256];
  let mut h = 0; // the current “hash”
  let mut it = input.chars();

  // hash the first sequence
  for _ in 0..4 {
    h <<= 2;
    let x = map(it.next().unwrap());
    h += x;
  }

  counts[h as usize] += 1;

  for c in it {
    h <<= 2;
    h += map(c);
    counts[h as usize] += 1;
  }

  for (i, c) in counts.into_iter().enumerate().filter(|&(_, &c)| c >= 1) {
    eprintln!("{:?} occurs {} times", unmap(i as u8), c);
  }
}

fn map(c: char) -> u8 {
  match c {
    'a' => 0b00,
    'c' => 0b01,
    'g' => 0b10,
    't' => 0b11,
    _ => panic!("input is not a correct DNA sequence: {}", c)
  }
}

fn unmap(b: u8) -> [char; 4] {
  let unmap2 = |x| match x & 0b11 {
    0b00 => 'a',
    0b01 => 'c',
    0b10 => 'g',
    0b11 => 't',
    _ => panic!("sequence is incorrectly encoded {}; cannot unmap", x & 0b11)
  };

  [unmap2(b >> 6), unmap2(b >> 4), unmap2(b >> 2), unmap2(b)]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let s = "aacttacgacagatatagcagacgttagtagtagacgacgtacgatcgcagcagtcagtcagtcagtcaaaaaaaatcggatcagtactgacgagaggagaggatgcatgactgtattatttatcagtcagtcagtcagtcagtcagtactgactactgacgtctgacagtacgaagtatgatgatgatgacgacat";
    solve(s);
    panic!();
  }
}
