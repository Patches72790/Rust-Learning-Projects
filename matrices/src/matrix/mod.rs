use crate::Vector;
use std::fmt::Debug;

impl<T> std::fmt::Display for Matrix<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[\n");
        for v in self.0.iter() {
            s += &v.to_string();
        }
        //let s = self.0.iter().map(|v| format!("{v:?}")).collect::<String>();

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Matrix<T>(Vec<Vector<T>>);

pub struct MatrixIterator<'a, T> {
    _matrix: &'a Matrix<T>,
    index: usize,
}

impl<'a, T> Iterator for MatrixIterator<'a, &'a T> {
    type Item = &'a Vector<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self._matrix.0.get(self.index) {
            Some(v) => Some(v),
            _ => None,
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self::elements(&[&[]])
    }

    pub fn elements(e: &[&[T]]) -> Self {
        let vs = e.iter().map(|e| Vector::elements(e)).collect();
        Self(vs)
    }
}

impl<T> Default for Matrix<T>
where
    T: Clone,
{
    fn default() -> Self {
        Matrix::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let m = Matrix::elements([[1, 2, 3].as_ref()].as_ref());

        println!("{m:?}");
    }
}
