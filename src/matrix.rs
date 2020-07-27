use core::{ops::{Add, Sub, Mul, Neg, Index, IndexMut}, cmp::PartialEq};
//use std::fmt::{self, Display, Formatter};
use crate::vector::Vector;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T> {
    m: Vec<Vector<T>>,
    rows: usize,
    columns: usize,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, i: (usize, usize)) -> &Self::Output {
        &self.m[i.1][i.0]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut Self::Output {
        &mut self.m[i.1][i.0]
    }
}

impl<T> Add for Matrix<T>
    where T: Add<T, Output = T> + Copy
{
    type Output = Option<Self>;

    fn add(self, o: Self) -> Self::Output {
        if self.rows == o.rows && self.columns == o.columns {
            Some(Self {
                m: self.m.iter().enumerate().map(|i|  (i.1 + &o.m[i.0]).unwrap()).collect(),
                ..self
            })
        }
        else { None }
    }
}

impl<T> Sub for Matrix<T>
    where T: Sub<T, Output = T> + Copy
{
    type Output = Option<Self>;

    fn sub(self, o: Self) -> Self::Output {
        if self.rows == o.rows && self.columns == o.columns {
            Some(Self {
                m: self.m.iter().enumerate().map(|i|  (i.1 - &o.m[i.0]).unwrap()).collect(),
                ..self
            })
        }
        else { None }
    }
}

impl<T> Neg for Matrix<T>
    where T: Neg<Output = T> + Copy
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            m: self.m.iter().map(|i|  -i).collect(),
            ..self
        }
    }
}

impl<T, U> Mul<U> for Matrix<T>
    where T: Mul<U, Output = T> + Copy,
          U: Copy
{
    type Output = Self;

    fn mul(self, o: U) -> Self::Output {
        Self {
            m: self.m.iter().map(|e| e * o).collect(),
            ..self
        }
    }
}

impl<T> Mul<Vector<T>> for Matrix<T>
    where T: Add<T, Output = T> + Mul<T, Output = T> + Copy
{
    type Output = Option<Vector<T>>;

    fn mul(self, o: Vector<T>) -> Self::Output {
        if self.rows == o.len() {
            if self.rows == 0 {
                Some(Vector::new(Vec::new()))
            }
            else {
                let mut vecs: Vec<Vector<T>> = self.m.iter().enumerate().map(|i| i.1 * o[i.0]).collect();
                // reversing it twice is soooo inefficient, but it's the best way I could think of
                vecs.reverse();
                let mut builder = vecs.pop().unwrap();
                vecs.reverse();
                for i in vecs.iter() {
                    builder = (&builder + i).unwrap();
                }
                Some(builder)
            }
        }
        else { None }
    }
}

/*impl<T> Matrix<T>
    where T: Add + Sub + Neg + Clone + Copy + PartialEq
{
    fn row(&self, i: usize) -> Vec<T> {
        self.m.iter().map(|e| e[i]).collect()
    }
    fn transpose(&self) -> Self {
        Self {
            m: self.m
        }
    }
}*/