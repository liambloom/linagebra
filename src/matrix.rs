use std::{
    ops::{Add, Sub, Mul, Neg, Index, IndexMut},
    cmp::PartialEq,
    convert::{From, TryFrom},
    borrow::ToOwned,
    //fmt::{self, Display, Formatter},
};
use crate::{Vector, /*diff_len::**/};

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T> {
    m: Vec<Vector<T>>,
    rows: usize,
    columns: usize,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, i: (usize, usize)) -> &Self::Output {
        &self.m[i.1 - 1][i.0]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut Self::Output {
        &mut self.m[i.1][i.0]
    }
}

impl<T> Add for Matrix<T>
    where T: Add<T, Output = T> + From<i32> + Copy
{
    type Output = Option<Self>;

    fn add(self, o: Self) -> Self::Output {
        if self.rows == o.rows && self.columns == o.columns {
            Some(Self {
                m: self.m.iter().enumerate().map(|i|  (i.1 + &o.m[i.0])).collect(),
                ..self
            })
        }
        else { None }
    }
}

impl<T> Sub for Matrix<T>
    where T: Sub<T, Output = T> + From<i32> + Copy
{
    type Output = Option<Self>;

    fn sub(self, o: Self) -> Self::Output {
        if self.rows == o.rows && self.columns == o.columns {
            Some(Self {
                m: self.m.iter().enumerate().map(|i|  (i.1 - &o.m[i.0])).collect(),
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
    where T: Add<T, Output = T> + Mul<T, Output = T> + From<i32> + Copy
{
    type Output = Option<Vector<T>>;

    fn mul(self, o: Vector<T>) -> Self::Output {
        if self.rows == o.len() {
            if self.rows == 0 {
                Some(Vector::from(Vec::new()))
            }
            else {
                let mut vecs: Vec<Vector<T>> = self.m.iter().enumerate().map(|i| i.1 * o[i.0]).collect();
                let mut builder = vecs.remove(0);
                for i in vecs.iter() {
                    builder = &builder + i;
                }
                Some(builder)
            }
        }
        else { None }
    }
}

impl<T> From<Vector<T>> for Matrix<T> {
    fn from(v: Vector<T>) -> Self {
        Self {
            rows: v.len(), // This need to be before m because m moves v into the Vec
            m: vec![v],
            columns: 1,
        }
    }
}

impl<T> TryFrom<Vec<Vector<T>>> for Matrix<T>
    where T: Copy {
    type Error = &'static str;

    fn try_from(v: Vec<Vector<T>>) -> Result<Self, Self::Error> {
        if v.len() == 0 {
            Ok(Self {
                m: Vec::new(),
                rows: 0,
                columns: 0,
            })
        }
        else {
            let len = v[0].len();
            if v.iter().all(|vec| vec.len() == len) {
                Ok(Self {
                    rows: v.len(),
                    m: v,
                    columns: len,
                }.transpose())
            }
            else {
                Err("Cannot make Matrix from a jagged Vec")
            }
        }
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T>
    where T: Copy {
    type Error = &'static str;

    fn try_from(v: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Self::try_from(v.iter().map(|vec| Vector::from(vec.to_owned())).collect::<Vec<Vector<T>>>())
    }
}

impl<T> Matrix<T> {
    pub fn get_rows(&self) -> usize {
        self.rows
    }
    pub fn get_columns(&self) -> usize {
        self.columns
    }
}
impl<T> Matrix<T>
    where T: Copy 
{
    pub fn row(&self, i: usize) -> Vec<T> {
        self.m.iter().map(|e| e[i]).collect()
    }
    pub fn column(&self, i: usize) -> Vec<T> {
        self.m[i].vec().to_owned()
    }
    pub fn transpose(&self) -> Self {
        let mut rows: Vec<Vector<T>> = Vec::new();
        for i in 0..self.rows {
            rows.push(self.row(i).into());
        }
        Self {
            m: rows,
            columns: self.rows,
            rows: self.columns,
        }
    }
}

impl<T> Matrix<T>
    where T: From<i32> + Clone,
{
    pub fn identity(size: usize) -> Self {
        let mut m: Vec<Vector<T>> = vec![vec![0.into(); size].into(); size];
        for i in m.iter_mut().enumerate() {
            i.1[i.0] = 1.into()
        }
        Self {
            m,
            columns: size,
            rows: size,
        }
    }
}