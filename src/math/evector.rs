// 1. imports
use num_traits::Float;
use std::ops::{Add, DivAssign, Index, IndexMut, Mul, Sub};

// 2. types
#[derive(Debug, Clone, Copy)]
pub struct EVector<const N: usize, T> {
    data: [T; N],
}

//alias point2
pub type FPoint2 = EVector<2, f64>;
pub type FVector2 = EVector<2, f64>;
pub type FPoint3 = EVector<3, f64>;
pub type FVector3 = EVector<3, f64>;
pub type FVector4 = EVector<4, f64>;

// 3. impls
/// fgfdfds
/// sdfsdfssfd
impl<T: Copy, const N: usize> EVector<N, T> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }

    pub fn sqr_distance(&self, p2: &Self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Default,
    {
        let diff = *p2 - *self;
        Self::sqr_magnitude(&diff)
    }

    pub fn dot(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Default,
    {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a * *b)
            .fold(T::default(), |acc, val| acc + val)
    }

    pub fn cross(&self, other: &Self) -> Self
    where
        T: Mul<Output = T> + Add<Output = T> + Default + Copy + Sub<Output = T>,
    {
        let mut result = Self::default();

        result.data[0] = self.data[1] * other.data[2] - self.data[2] * other.data[1];
        result.data[1] = self.data[2] * other.data[0] - self.data[0] * other.data[2];
        result.data[2] = self.data[0] * other.data[1] - self.data[1] * other.data[0];
        result
    }

    pub fn sqr_magnitude(&self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Default,
    {
        print!("sqr_magnitude: ");
        self.dot(self)
    }

    pub fn magnitude(&self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Default + Float,
    {
        Self::sqr_magnitude(self).sqrt()
    }

    pub fn distance(&self, other: &Self, test: f64) -> T
    where
        T: Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Default + Float,
    {
        self.sqr_distance(other).sqrt()
    }

    pub fn normalize(&mut self)
    where
        T: Mul<Output = T> + Add<Output = T> + Default + Float + DivAssign,
    {
        let mag = self.magnitude();
        self.data.iter_mut().for_each(|x| *x /= mag);
    }

    pub fn get_normalized(&self) -> Self
    where
        T: Mul<Output = T> + Add<Output = T> + Default + Float + DivAssign,
    {
        let mut result = *self;
        result.normalize();
        result
    }

    pub fn reflect(&self, normal: &Self) -> Self
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy + From<u8>,
    {
        *self - *normal * (self.dot(normal) * <T as From<u8>>::from(2))
    }

    pub fn project(&self, other: &Self) -> Self
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy,
    {
        let mag = other.sqr_magnitude();
        *other * (self.dot(other) / mag)
    }

    pub fn reject(&self, other: &Self) -> Self
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy,
    {
        *self - self.project(other)
    }

    pub fn get_angle_radians(&self, other: &Self) -> T
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy,
    {
        let dot = self.dot(other);
        let mag = self.magnitude() * other.magnitude();
        let result = dot / mag;
        result.acos()
    }

    pub fn get_angle_degrees(&self, other: &Self) -> T
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy,
    {
        self.get_angle_radians(other).to_degrees()
    }

    pub fn get_signed_angle_radians(&self, other: &Self, axis: &Self) -> T
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy,
    {
        let _myfloat: f64 = 5.0;
        let cross = axis.cross(self);
        let dot = cross.dot(other);
        let mag = self.magnitude() * other.magnitude();
        let result = dot / mag;
        result.asin()
    }

    pub fn get_signed_angle_degrees(&self, other: &Self, axis: &Self) -> T
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy,
    {
        self.get_signed_angle_radians(other, axis).to_degrees()
    }

    pub fn lerp(&self, other: &Self, t: T) -> Self
    where
        T: Default + Float + Mul<Output = T> + Add<Output = T> + Copy,
    {
        *other + ((*other - *self) * t)
    }
}

impl<T: Default + Copy, const N: usize> Default for EVector<N, T> {
    fn default() -> Self {
        Self { data: std::array::from_fn(|_| T::default()) }
    }
}

// 4. Operator overloads
impl<const N: usize, T: Copy + Sub<Output = T>> Sub for EVector<N, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(std::array::from_fn(|i| self.data[i] - rhs.data[i]))
    }
}

impl<const N: usize, T: Copy + Add<Output = T>> Add for EVector<N, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(std::array::from_fn(|i| self.data[i] + rhs.data[i]))
    }
}

impl<const N: usize, T: Copy + Mul<Output = T>> Mul for EVector<N, T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(std::array::from_fn(|i| self.data[i] * rhs.data[i]))
    }
}

impl<const N: usize, T: Copy + Mul<Output = T>> Mul<T> for EVector<N, T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(std::array::from_fn(|i| self.data[i] * rhs))
    }
}

impl<const N: usize, T: Copy + DivAssign> DivAssign for EVector<N, T> {
    fn div_assign(&mut self, rhs: Self) {
        self.data
            .iter_mut()
            .zip(rhs.data.iter())
            .for_each(|(self_val, rhs_val)| *self_val /= *rhs_val);
    }
}

// 5. Indexing
impl<const N: usize, T: Copy> Index<usize> for EVector<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, T: Copy> IndexMut<usize> for EVector<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
