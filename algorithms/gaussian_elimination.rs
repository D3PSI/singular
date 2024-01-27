use crate::core::{
    markers::{
        AssociativeOp, CommutativeOp, DistributiveOp, FieldEl, InverseEl,
        NonzeroMultiplicativeUnitEl,
    },
    types::Matrix,
};

pub fn gaussian_elimination<EL, ADD, MUL, const M: usize, const N: usize>(
    matrix: Matrix<EL, M, N>,
) -> Matrix<EL, M, N>
where
    EL: FieldEl<ADD, MUL>,
    ADD: AssociativeOp<EL> + CommutativeOp<EL>,
    MUL: AssociativeOp<EL> + CommutativeOp<EL> + DistributiveOp<EL, ADD>,
{
    let mut res = matrix.clone();

    for j in 0..N {
        for i in j + 1..M {
            if res[j][j] == EL::ZERO {
                continue;
            }
            let factor = MUL::op(
                res[i][j],
                NonzeroMultiplicativeUnitEl::<ADD, MUL>::inverse(&res[j][j]),
            );
            for k in 0..N {
                res[i][k] = ADD::op(
                    res[i][k],
                    InverseEl::<ADD>::inverse(&MUL::op(factor, res[j][k])),
                );
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::gaussian_elimination::gaussian_elimination, core::types::Matrix};

    #[test]
    fn row_echelon_form() {
        let res: Matrix<f32, 4, 4> = gaussian_elimination(
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]
            .into(),
        );
        assert_eq!(
            res,
            [
                [1.0, 2.0, 3.0, 4.0],
                [0.0, -4.0, -8.0, -12.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
            .into()
        );

        let res: Matrix<f64, 2, 3> =
            gaussian_elimination([[2.0, -1.0, 1.0], [1.0, 1.0, 5.0]].into());
        assert_eq!(res, [[2.0, -1.0, 1.0], [0.0, 1.5, 4.5]].into());
    }
}
