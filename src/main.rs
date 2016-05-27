const N: usize = 4;
const XXX: u64 = 0xf;

fn rref(matrix: [u64; N]) -> [u64; N] {
  let mut matrix_rref: [u64; N];

  matrix_rref = matrix.clone();

  for pivot in 0..N {
    matrix_rref.sort_by(|a, b| b.cmp(a));
    
    if matrix_rref[pivot] == 0 {
      break;
    }
    
    if matrix_rref[pivot] >> (N - pivot - 1) == 0 {
      continue;
    }

    for row in 0..N {
      if row == pivot {
        continue;
      }
      if matrix_rref[row] & (1u64 << (N - pivot - 1)) != 0 {
        matrix_rref[row] ^= matrix_rref[pivot];
      }
    }
    
  }

  matrix_rref
}

fn dimension(matrix: [u64; N]) -> usize {

  let mut dim = 0;

  for row in rref(matrix).iter() {
    if *row == 0 {
      break;
    }
    dim += 1;
  }

  dim

}

fn matrix(d: u64) -> [u64; N] {
  let mut matrix: [u64; N] = [0; N];

  for i in 0..N {
    matrix[i] = cycle(d, i);
  }

  matrix
}

fn cycle(d: u64, i: usize) -> u64 {
  ((d << i) | (d >> (N - i))) & XXX
}


fn printmat(matrix:[u64; N]) {
  for i in matrix .iter() {
    println!("{:04b}", i);
  }
}


fn main() {
  let d: u64 = 0b11;
  printmat(rref(matrix(d)));
  println!("{}", dimension(matrix(d)));
}
