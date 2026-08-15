use crate::integral::ApInt;

impl ApInt {
    pub(crate) fn _mul_impl(&self, rhs: &Self) -> Self {
        match self.width() {
            0..=511 => self._mul_schoolbook_impl(rhs),
            512.. => self._mul_karatsuba_impl(rhs),
        }
    }
}
