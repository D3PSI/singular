use std::ops::{Index, IndexMut};

use super::markers::El;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<EL: El, const M: usize> {
    inner: [EL; M],
}

impl<EL: El, const M: usize> Index<usize> for Vector<EL, M> {
    type Output = EL;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<EL: El, const M: usize> IndexMut<usize> for Vector<EL, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<EL: El, S: Into<EL>, const M: usize> From<[S; M]> for Vector<EL, M> {
    fn from(data: [S; M]) -> Self {
        Self {
            inner: data
                .into_iter()
                .map(Into::into)
                .collect::<Vec<EL>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<EL: El, const M: usize, const N: usize> {
    inner: [Vector<EL, N>; M],
}

impl<EL: El, const M: usize, const N: usize> Index<usize> for Matrix<EL, M, N> {
    type Output = Vector<EL, N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<EL: El, const M: usize, const N: usize> IndexMut<usize> for Matrix<EL, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<EL: El, S: Into<Vector<EL, N>>, const M: usize, const N: usize> From<[S; M]>
    for Matrix<EL, M, N>
{
    fn from(data: [S; M]) -> Self {
        Self {
            inner: data
                .into_iter()
                .map(Into::into)
                .collect::<Vec<Vector<EL, N>>>()
                .try_into()
                .unwrap(),
        }
    }
}

pub struct FloatingPointAddition {}
pub struct FloatingPointMultiplication {}
