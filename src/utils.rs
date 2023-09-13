/// Helper functions for bit operations.
pub mod bits {
    /// Calculates the number of bits a rust type requires.
    pub const fn bit_size<T>() -> usize {
        std::mem::size_of::<T>() * 8
    }

    /// Calculates the minimum number of bits required to represent a number n.
    pub fn min_repr(n: u32) -> u32 {
        bit_size::<u32>() as u32 - n.leading_zeros() - 1
    }

    /// Checks if a number is a power of 2.
    pub fn is_pow2(n: u32) -> bool {
        n.count_ones() == 1
    }

    /// Splits a u32 into two u32s at the bit index.
    pub fn split_at(x: u32, n: u32) -> (u32, u32) {
        let mask = (1 << n) - 1;
        let right = x & mask;
        let left = ( x &! mask) >> n;
        (left, right)
    }

    /// Joins two u32s together at the bit index.
    pub fn join_at(x: u32, y: u32, n: u32) -> u32 {
        let x_shifted = x << n;
        x_shifted | y 
    }
}