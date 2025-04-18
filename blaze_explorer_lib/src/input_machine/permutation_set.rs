use std::ops::BitOr;
/// Enumeration of possible permutation sets to use when creating a permutated keymap
#[derive(Clone, Debug)]
pub enum PermutationSet {
    LowerAlpha,
    UpperAlpha,
    Digits,
    Custom(Vec<char>),
}

impl PermutationSet {
    /// Obtain the list of all possible elements based on the permutation set
    /// ```
    /// use blaze_explorer_lib::input_machine::permutation_set::PermutationSet;
    /// let set = PermutationSet::Digits;
    /// assert!(set.elements() == vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    /// ```
    pub fn elements(&self) -> Vec<char> {
        match self {
            PermutationSet::LowerAlpha => ('a'..='z').collect(),
            PermutationSet::UpperAlpha => ('A'..='Z').collect(),
            PermutationSet::Digits => ('0'..='9').collect(),
            PermutationSet::Custom(v) => v.clone(),
        }
    }
}

impl BitOr for PermutationSet {
    type Output = PermutationSet;

    fn bitor(self, rhs: PermutationSet) -> PermutationSet {
        let mut combined = self.elements();
        combined.extend(rhs.elements());

        // Optional: remove duplicates
        combined.sort_unstable();
        combined.dedup();

        PermutationSet::Custom(combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitor() {
        let result = PermutationSet::LowerAlpha | PermutationSet::UpperAlpha;
        assert!(result.elements().len() == 52);
    }
}
