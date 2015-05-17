//! Bitwise hamming distance calculation

/// A trait for calculating the bitwise hamming distance
pub trait BitwiseHammingDistancable<RHS = Self> {
    /// The output type of the hamming distance
    type Output;
    fn bitwise_hamming_distance(self, other: RHS) -> Self::Output;
}

impl<'a, 'b> BitwiseHammingDistancable<&'a u8> for &'b u8 {
    type Output = u32;
    /// Calculate the number of different bits between two `u8` bytes.
    fn bitwise_hamming_distance(self, other: &u8) -> u32 {
        return (self ^ other).count_ones();
    }
}

impl<'a, 'b> BitwiseHammingDistancable<&'a Vec<u8>> for &'b Vec<u8> {
    type Output = Result<u32, &'static str>;
    /// Calculate the number of different bits between two vectors of `u8` bytes.
    fn bitwise_hamming_distance(self, other: &Vec<u8>) -> Result<u32, &'static str> {
        if self.len() != other.len() {
            return Err("Vectors do not have equal length")
        }
        let mut distance : u32 = 0;
        for (b1, b2) in self.iter().zip(other.iter()) {
            distance += b1.bitwise_hamming_distance(b2);
        }
        return Ok(distance);
    }
}

impl<'a, 'b> BitwiseHammingDistancable<&'a [u8]> for &'b [u8] {
    type Output = Result<u32, &'static str>;
    /// Calculate the number of different bits between two slices of `u8` bytes.
    fn bitwise_hamming_distance(self, other: &[u8]) -> Result<u32, &'static str> {
        if self.len() != other.len() {
            return Err("Slices do not have equal length")
        }
        let mut distance : u32 = 0;
        for (b1, b2) in self.iter().zip(other.iter()) {
            distance += b1.bitwise_hamming_distance(b2);
        }
        return Ok(distance);
    }
}

#[cfg(test)]
mod tests {
    use bitwise_hamming_distance::BitwiseHammingDistancable;
    
    #[test]
    fn u8_bitwise_hamming_distance() {
        let byte1 : u8 = 0x01;
        let byte2 : u8 = 0x03;

        assert!(byte1.bitwise_hamming_distance(&byte2) == 1);
        assert!(byte2.bitwise_hamming_distance(&byte1) == 1);

        let byte3 : u8 = 0x01;
        let byte4 : u8 = 0xFF;

        assert!(byte3.bitwise_hamming_distance(&byte4) == 7);
        assert!(byte4.bitwise_hamming_distance(&byte3) == 7);
    }

    #[test]
    fn u8_vec_bitwise_hamming_distance() {
        let mut byte_vec1 : Vec<u8> = Vec::new();
        let mut byte_vec2 : Vec<u8> = Vec::new();

        byte_vec1.push(0x01);
        byte_vec2.push(0x03);

        byte_vec1.push(0x01);
        byte_vec2.push(0xFF);

        assert!(byte_vec1.bitwise_hamming_distance(&byte_vec2).unwrap() == 8);
    }

    #[test]
    fn u8_vec_bitwise_hamming_distance_error() {
        let mut byte_vec1 : Vec<u8> = Vec::new();
        let mut byte_vec2 : Vec<u8> = Vec::new();

        byte_vec1.push(0x01);
        byte_vec2.push(0x03);

        byte_vec2.push(0xFF);

        assert!(byte_vec1.bitwise_hamming_distance(&byte_vec2).unwrap_err() ==
            "Vectors do not have equal length");
    }

    #[test]
    fn u8_slice_bitwise_hamming_distance() {
        let byte_slice1 : &[u8] = &[0x01, 0x01];
        let byte_slice2 : &[u8] = &[0x03, 0xFF];

        assert!(byte_slice1.bitwise_hamming_distance(byte_slice2).unwrap() == 8);
    }

    #[test]
    fn u8_slice_bitwise_hamming_distance_error() {
        let byte_slice1 : &[u8] = &[0x01];
        let byte_slice2 : &[u8] = &[0x03, 0xFF];

        assert!(byte_slice1.bitwise_hamming_distance(byte_slice2).unwrap_err() ==
            "Slices do not have equal length");
    }
}