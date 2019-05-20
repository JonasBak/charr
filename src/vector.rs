use std::ops;

pub trait Vector<T>
where
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy
        + Clone,
{
    const DIMS: usize;
    fn zero() -> Self;
    fn get(&self, i: usize) -> T;
    fn set(&mut self, i: usize, value: T);
    fn len_sqr(&self) -> T {
        let mut sum = self.get(0) * self.get(0);
        for i in 1..Self::DIMS {
            sum = sum + self.get(i) * self.get(i);
        }
        sum
    }
    fn add(&self, other: &Self) -> Self
    where
        Self: std::marker::Sized,
    {
        let mut new = Self::zero();
        for i in 0..Self::DIMS {
            new.set(i, self.get(i) + other.get(i));
        }
        new
    }
    fn sub(&self, other: &Self) -> Self
    where
        Self: std::marker::Sized,
    {
        let mut new = Self::zero();
        for i in 0..Self::DIMS {
            new.set(i, self.get(i) - other.get(i));
        }
        new
    }
    fn mul(&self, other: &Self) -> Self
    where
        Self: std::marker::Sized,
    {
        let mut new = Self::zero();
        for i in 0..Self::DIMS {
            new.set(i, self.get(i) * other.get(i));
        }
        new
    }
    fn div(&self, other: &Self) -> Self
    where
        Self: std::marker::Sized,
    {
        let mut new = Self::zero();
        for i in 0..Self::DIMS {
            new.set(i, self.get(i) / other.get(i));
        }
        new
    }
    fn sum(&self) -> T {
        let mut sum = self.get(0);
        for i in 1..Self::DIMS {
            sum = sum + self.get(i)
        }
        sum
    }
    fn dot(&self, other: &Self) -> T
    where
        Self: std::marker::Sized,
    {
        self.add(other).sum()
    }
}

pub trait Zeroable {
    const ZERO: Self;
}

pub struct Vec3<T>(pub T, pub T, pub T)
where
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy
        + Clone
        + Zeroable;

impl<T> Vector<T> for Vec3<T>
where
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy
        + Clone
        + Zeroable,
{
    const DIMS: usize = 3;

    fn zero() -> Self {
        Vec3(T::ZERO, T::ZERO, T::ZERO)
    }
    fn get(&self, i: usize) -> T {
        match i {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("Illegal index"),
        }
    }
    fn set(&mut self, i: usize, value: T) {
        match i {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            _ => panic!("Illegal index"),
        };
    }
}

pub struct Vec2<T>(pub T, pub T)
where
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy
        + Clone
        + Zeroable;

impl<T> Vector<T> for Vec2<T>
where
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy
        + Clone
        + Zeroable,
{
    const DIMS: usize = 3;

    fn zero() -> Self {
        Vec2(T::ZERO, T::ZERO)
    }
    fn get(&self, i: usize) -> T {
        match i {
            0 => self.0,
            1 => self.1,
            _ => panic!("Illegal index"),
        }
    }
    fn set(&mut self, i: usize, value: T) {
        match i {
            0 => self.0 = value,
            1 => self.1 = value,
            _ => panic!("Illegal index"),
        };
    }
}

impl Zeroable for f32 {
    const ZERO: f32 = 0.0;
}

pub type Vec3f = Vec3<f32>;
pub type Vec2f = Vec2<f32>;

impl Zeroable for i32 {
    const ZERO: i32 = 0;
}

pub type Vec3i = Vec3<i32>;
pub type Vec2i = Vec2<i32>;
