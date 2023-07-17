use subtle::{ConditionallySelectable, Choice};

// Name:        cmov
//
// Description: Copy len bytes from x to r if b is 1;
//              don't modify x if b is 0. Requires b to be in {0,1};
//              assumes two's complement representation of negative integers.
//              Runs in constant time.
//
// Arguments:   [u8] r:       output byte array
//              const [u8] x: input byte array
//              size_t len:             Amount of bytes to be copied
//              [u8]  b:        Condition bit; has to be in {0,1}
pub fn cmov(r: &mut[u8], x: &[u8], len: usize, b: Choice)
{
  for i in 0..len {
    r[i].conditional_assign(&x[i], b);
  }
}
