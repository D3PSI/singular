use super::markers::{
    AbelianGroupEl, AssociativeOp, CommutativeOp, CommutativeRingEl, DistributiveOp, El, FieldEl,
    GroupEl, IntegralDomainEl, InverseEl, MonoidEl, NeutralEl, NoZerodivisorEl,
    NonzeroMultiplicativeUnitEl, Op, RingEl,
};
use super::types::{FloatingPointAddition, FloatingPointMultiplication};

impl<EL: NeutralEl<OP>, OP: AssociativeOp<EL>> MonoidEl<OP> for EL {}

impl<EL: MonoidEl<OP> + InverseEl<OP>, OP: AssociativeOp<EL>> GroupEl<OP> for EL {}

impl<EL: GroupEl<OP>, OP: AssociativeOp<EL> + CommutativeOp<EL>> AbelianGroupEl<OP> for EL {}

impl<
        EL: AbelianGroupEl<ADD> + MonoidEl<MUL>,
        ADD: AssociativeOp<EL> + CommutativeOp<EL>,
        MUL: AssociativeOp<EL> + DistributiveOp<EL, ADD>,
    > RingEl<ADD, MUL> for EL
{
}

impl<
        EL: RingEl<ADD, MUL>,
        ADD: AssociativeOp<EL> + CommutativeOp<EL>,
        MUL: AssociativeOp<EL> + CommutativeOp<EL> + DistributiveOp<EL, ADD>,
    > CommutativeRingEl<ADD, MUL> for EL
{
}

impl<
        EL: NoZerodivisorEl<ADD, MUL>,
        ADD: AssociativeOp<EL> + CommutativeOp<EL>,
        MUL: AssociativeOp<EL> + CommutativeOp<EL> + DistributiveOp<EL, ADD>,
    > IntegralDomainEl<ADD, MUL> for EL
{
}

impl<
        EL: NonzeroMultiplicativeUnitEl<ADD, MUL>,
        ADD: AssociativeOp<EL> + CommutativeOp<EL>,
        MUL: AssociativeOp<EL> + CommutativeOp<EL> + DistributiveOp<EL, ADD>,
    > FieldEl<ADD, MUL> for EL
{
}

macro_rules! impl_float {
    ($($t:ty)*) => ($(
        impl El for $t {}

        impl Op<$t> for FloatingPointAddition {
            fn op(lhs: $t, rhs: $t) -> $t {
                lhs + rhs
            }
        }

        impl Op<$t> for FloatingPointMultiplication {
            fn op(lhs: $t, rhs: $t) -> $t {
                lhs * rhs
            }
        }

        impl NeutralEl<FloatingPointAddition> for $t {
            const NEUTRAL_ELEMENT: Self = 0.0;
        }

        impl AssociativeOp<$t> for FloatingPointAddition {}

        impl InverseEl<FloatingPointAddition> for $t {
            fn inverse(&self) -> Self {
                -self
            }
        }

        impl CommutativeOp<$t> for FloatingPointAddition {}

        impl NeutralEl<FloatingPointMultiplication> for $t {
            const NEUTRAL_ELEMENT: Self = 1.0;
        }

        impl AssociativeOp<$t> for FloatingPointMultiplication {}

        impl DistributiveOp<$t, FloatingPointAddition> for FloatingPointMultiplication {}

        impl CommutativeOp<$t> for FloatingPointMultiplication {}

        impl NoZerodivisorEl<FloatingPointAddition, FloatingPointMultiplication> for $t {}

        impl NonzeroMultiplicativeUnitEl<FloatingPointAddition, FloatingPointMultiplication>
            for $t
        {
            fn inverse(&self) -> Self {
                1.0 / self
            }
        }
    )*)
}

impl_float! { f32 f64 }
