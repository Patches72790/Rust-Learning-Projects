use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(Debug)]
struct Vector<T>(Vec<T>);

impl<T> Vector<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self::elements(&[])
    }

    pub fn elements(e: &[T]) -> Self {
        Self(e.to_vec())
    }
}

impl<T> std::fmt::Display for Vector<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

struct VectorIterator<T>
where
    T: Clone,
{
    _vector: Vector<T>,
    index: usize,
}

impl<T> Default for Vector<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Vector(vec![])
    }
}

impl<T> Iterator for VectorIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self._vector.0.get(self.index) {
            Some(val) => {
                self.index += 1;
                Some(val.clone())
            }
            None => None,
        }
    }
}

impl<T> FromIterator<T> for Vector<T>
where
    T: Mul<Output = T>,
{
    fn from_iter<I: IntoIterator<Item = T::Output>>(iter: I) -> Self {
        let mut v: Vector<T> = Vector(vec![]);
        for i in iter {
            v.0.push(i);
        }

        v
    }
}

impl<T> IntoIterator for Vector<T>
where
    T: Clone + Default,
{
    type Item = T;
    type IntoIter = VectorIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator {
            _vector: self,
            index: 0,
        }
    }
}

impl<T> IntoIterator for &Vector<T>
where
    T: Clone + Default,
{
    type Item = T;
    type IntoIter = VectorIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator {
            _vector: self,
            index: 0,
        }
    }
}

impl<T> Mul for Vector<T>
where
    T: Clone + Default + Debug + Mul,
    T: Borrow<T>,
    Vector<T>: FromIterator<<T as Mul>::Output>,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.0
            .iter()
            .zip(rhs)
            .map(|(a, b)| a.clone() * b)
            .collect::<Vector<T>>()
    }
}

impl<T> Mul for &Vector<T>
where
    T: Clone + Default + Debug + Mul,
    T: Borrow<T>,
    Vector<T>: FromIterator<<T as Mul>::Output>,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.0
            .iter()
            .zip(rhs)
            .map(|(a, b)| a.clone() * b)
            .collect::<Vector<T>>()
    }
}

impl<T> Add for Vector<T>
where
    T: Clone + Default + Debug + Add,
    Vector<T>: FromIterator<<T as Add>::Output>,
{
    type Output = Vector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        self.0
            .iter()
            .zip(rhs)
            .map(|(a, b)| a.clone() + b)
            .collect::<Vector<T>>()
    }
}

fn main() {
    let v1 = Vector::elements(&[1, 2, 3, 4]);
    let v2 = Vector::elements(&[4, 5, 6, 7]);

    let v3 = v1 * v2;

    println!("{v3}");
}
