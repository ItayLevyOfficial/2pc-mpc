// Author: dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use group::{AffineXCoordinate, PrimeGroupElement};

use crate::sign::verify_signature;

#[cfg_attr(feature = "benchmarking", derive(Clone))]
pub struct Party<const SCALAR_LIMBS: usize, GroupElement: PrimeGroupElement<SCALAR_LIMBS>> {
    pub(super) message: GroupElement::Scalar,
    pub(super) public_key: GroupElement,
}

impl<
        const SCALAR_LIMBS: usize,
        GroupElement: PrimeGroupElement<SCALAR_LIMBS> + AffineXCoordinate<SCALAR_LIMBS>,
    > Party<SCALAR_LIMBS, GroupElement>
{
    pub fn verify_signature(
        self,
        nonce_x_coordinate: GroupElement::Scalar,
        signature_s: GroupElement::Scalar,
    ) -> crate::Result<()> {
        verify_signature(
            nonce_x_coordinate,
            signature_s,
            self.message,
            self.public_key,
        )?;

        Ok(())
    }
}