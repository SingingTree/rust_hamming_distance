/// A trait for calculating hamming distance
pub trait HammingDistancable<RHS = Self> {
    /// The output type of the hamming distance
    type Output;
    fn hamming_distance(self, other: RHS) -> Self::Output;
}

impl<'a, 'b, T> HammingDistancable<&'b Vec<T>> for &'a Vec<T>
    where T : Eq {
    type Output = Result<u32, &'static str>;
    /// Calculate the hamming distance between vectors
    fn hamming_distance(self, other: &'b Vec<T>) -> Result<u32, &'static str> {
        if self.len() != other.len() {
            return Err("Vectors do not have equal length");
        }

        let mut distance = 0;
        for (a, b) in self.iter().zip(other.iter()) {
            if a != b {
                distance += 1;
            }
        }

        return Ok(distance);
    }
}

impl<'a, 'b, T> HammingDistancable<&'b [T]> for &'a [T]
    where T : Eq {
    type Output = Result<u32, &'static str>;
    /// Calculate the hamming distance between slices
    fn hamming_distance(self, other: &'b [T]) -> Result<u32, &'static str> {
        if self.len() != other.len() {
            return Err("Slices do not have equal length");
        }

        let mut distance = 0;
        for (a, b) in self.iter().zip(other.iter()) {
            if a != b {
                distance += 1;
            }
        }

        return Ok(distance);
    }
}

impl <'a, 'b> HammingDistancable<&'b String> for &'a String {
    type Output = Result<u32, &'static str>;
    /// Calculate the hamming distance between strings
     fn hamming_distance(self, other: &'b String) -> Result<u32, &'static str> {
        if self.len() != other.len() {
            return Err("Strings do not have equal length");
        }

        let mut distance = 0;
        for (a, b) in self.chars().zip(other.chars()) {
            if a != b {
                distance += 1;
            }
        }

        return Ok(distance);
    }
}

impl <'a, 'b> HammingDistancable<&'b str> for &'a str {
    type Output = Result<u32, &'static str>;
    /// Calculate the hamming distance between borrowed strings
     fn hamming_distance(self, other: &'b str) -> Result<u32, &'static str> {
        if self.len() != other.len() {
            return Err("Strings do not have equal length");
        }

        let mut distance = 0;
        for (a, b) in self.chars().zip(other.chars()) {
            if a != b {
                distance += 1;
            }
        }

        return Ok(distance);
    }
}

#[cfg(test)]
mod tests {
    use hamming_distance::HammingDistancable;
    
    #[test]
    fn string_hamming_distance() {
        let string1 = "Cat";
        let string2 = "Hat";

        assert!(string1.hamming_distance(string2).unwrap() == 1);
    }
}