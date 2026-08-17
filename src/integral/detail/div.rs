use crate::integral::ApInt;

impl ApInt {
    pub(crate) fn _udiv_impl(&self, rhs: &Self) -> Self {
        self._udiv_knuthd_impl(rhs)
    }

    pub(crate) fn _urem_impl(&self, rhs: &Self) -> Self {
        self._urem_knuthd_impl(rhs)
    }

    pub(crate) fn _udivrem_impl(&self, rhs: &Self) -> (Self, Self) {
        self._udivrem_knuthd_impl(rhs)
    }
}
