use ruint::Uint;

pub trait IsTrue<const T: bool> {}
impl IsTrue<true> for () {}

trait BytesDiff<T> {
    const DIFF: usize;
}

impl<const BYTES1: usize, const LIMBS1: usize, const BYTES2: usize, const LIMBS2: usize> BytesDiff<Uint<BYTES2, LIMBS2>>
    for Uint<BYTES1, LIMBS1>
{
    const DIFF: usize = if BYTES1 >= BYTES2 {
        BYTES1 - BYTES2
    } else {
        BYTES2 - BYTES1
    };
}
