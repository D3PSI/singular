use std::fmt::Debug;
use std::fmt::Display;

/// Marker for a set element $e \in S$. A type should implement this trait if instances of that type
/// represent elements $e$ of a mathematical set $S$ that is of interest. In particular this requires that every
/// possible instance $e$ of this type is part of the set $S$.
pub trait El: Sized + Clone + Copy + Display + Debug + PartialEq {}

/// Marker for an operation $op : S^2 \longrightarrow S$ on a set $S$.
pub trait Op<EL: El> {
    fn op(lhs: EL, rhs: EL) -> EL;
}

/// Marker for an associative operation $op : S^2 \longrightarrow S$ on a set $S$.
///
/// This expresses the requirement that $x, y, z \in S \implies op(op(x, y), z) = op(x, op(y, z))$.
pub trait AssociativeOp<EL: El>: Op<EL> {}

/// Marker for a commutative operation $op : S^2 \longrightarrow S$ on a set $S$.
///
/// This expresses the requirement that $x, y \in S \implies op(x, y) = op(y, x)$.
pub trait CommutativeOp<EL: El>: Op<EL> {}

/// Marker for an operation $mul : S^2 \longrightarrow S$ that distributes over another operation
/// $add : S^2 \longrightarrow S$ on a set $S$.
///
/// This expresses the requirement that $x, y, z \in S \implies mul(x, add(y, z)) = add(mul(x, y),
/// mul(x, z))$.
pub trait DistributiveOp<EL: El, ADD: Op<EL>>: Op<EL> {}

/// Marker for a neutral element $e$ of an operation $op : S^2 \longrightarrow S$ on a set $S$.
/// A type that implements `El` should also implement `NeutralEl` if and only if there exists
/// a unique neutral element $e \in S$ for the operation $op$ such that $x \in S \implies op(x, e) = op(e, x) =
/// x$.
pub trait NeutralEl<OP: Op<Self>>: El {
    const NEUTRAL_ELEMENT: Self;
}

/// Marker for a monoid $\langle S; op, e \rangle$ in which a binary operation $op : S^2 \longrightarrow S$ is associative and a unique neutral element $e$ exists for $op$ in $S$.
pub trait MonoidEl<OP: AssociativeOp<Self>>: NeutralEl<OP> {}

/// Marker for an inverse element $a^{-1} \in S$ for an element $a \in S$ of an operation $op : S^2 \longrightarrow S$ on a
/// set $S$. A type that implements `NeutralEl` should implement `InverseEl` if and only if every
/// instance $a \in S$ has a unique inverse element $a^{-1} \in S$ such that $op(a, a^{-1}) =
/// op(a^{-1}, a) = e$ where $e \in S$ is the unique neutral element of $op$ on $S$.
pub trait InverseEl<OP: Op<Self>>: NeutralEl<OP> {
    fn inverse(&self) -> Self;
}

/// Marker for a group $\langle S; op, ^{-1}, e \rangle$ in which $\langle S; op, e  \rangle$ is a
/// monoid and a unique inverse element $a^{-1} \in S$ exists for every $a \in S$ for $op$.
pub trait GroupEl<OP: AssociativeOp<Self>>: MonoidEl<OP> + InverseEl<OP> {}

/// Marker for a group $\langle S; op, ^{-1}, e \rangle$ in which $S$ is a finite set.
pub trait FiniteGroupEl<OP: AssociativeOp<Self>>: GroupEl<OP> {
    const GROUP_ORDER: usize;

    fn order(&self) -> usize {
        todo!();
    }
}

/// Marker for a commutative (abelian) group $\langle S; op, ^{-1}, e \rangle$ in which $op$
/// commutes over $S$.
pub trait AbelianGroupEl<OP: AssociativeOp<Self> + CommutativeOp<Self>>: GroupEl<OP> {}

/// Marker for a ring $\langle S; add, ^{-1}, 0, mul, 1 \rangle$ in which $\langle S; add, ^{-1}, 0 \rangle$ is an abelian group, $\langle S; mul, 1 \rangle$ is a monoid and $mul$ distributes over $add$.
pub trait RingEl<
    ADD: AssociativeOp<Self> + CommutativeOp<Self>,
    MUL: AssociativeOp<Self> + DistributiveOp<Self, ADD>,
>: AbelianGroupEl<ADD> + MonoidEl<MUL>
{
    const ZERO: Self = <Self as NeutralEl<ADD>>::NEUTRAL_ELEMENT;
    const ONE: Self = <Self as NeutralEl<MUL>>::NEUTRAL_ELEMENT;
}

/// Marker for a commutative ring $\langle S; add, ^{-1}, 0, mul, 1 \rangle$ in which $mul$
/// commutes.
pub trait CommutativeRingEl<
    ADD: AssociativeOp<Self> + CommutativeOp<Self>,
    MUL: AssociativeOp<Self> + CommutativeOp<Self> + DistributiveOp<Self, ADD>,
>: RingEl<ADD, MUL>
{
}

pub trait NoZerodivisorEl<
    ADD: AssociativeOp<Self> + CommutativeOp<Self>,
    MUL: AssociativeOp<Self> + CommutativeOp<Self> + DistributiveOp<Self, ADD>,
>: CommutativeRingEl<ADD, MUL>
{
}

/// Marker for an integral domain (a commutative ring without zerodivisors).
pub trait IntegralDomainEl<
    ADD: AssociativeOp<Self> + CommutativeOp<Self>,
    MUL: AssociativeOp<Self> + CommutativeOp<Self> + DistributiveOp<Self, ADD>,
>: NoZerodivisorEl<ADD, MUL>
{
}

/// Marker to ensure that division (except by zero) is allowed
pub trait NonzeroMultiplicativeUnitEl<
    ADD: AssociativeOp<Self> + CommutativeOp<Self>,
    MUL: AssociativeOp<Self> + CommutativeOp<Self> + DistributiveOp<Self, ADD>,
>: IntegralDomainEl<ADD, MUL>
{
    fn inverse(&self) -> Self;
}

/// Marker for a field (a non-trivial commutative ring in which every non-zero element is a unit (invertible under multiplication))
pub trait FieldEl<
    ADD: AssociativeOp<Self> + CommutativeOp<Self>,
    MUL: AssociativeOp<Self> + CommutativeOp<Self> + DistributiveOp<Self, ADD>,
>: NonzeroMultiplicativeUnitEl<ADD, MUL>
{
}
