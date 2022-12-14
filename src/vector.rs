use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};


/// A Vector can represent a point / a direction ...  containing one x & y coordinate
/// The type of the coordinates can be any type of number.
#[derive(Debug, Clone, Copy)]
pub struct Vector<T: num::Num> (pub T, pub T);


impl<T: num::Num + num::NumCast + Copy> Vector<T> {
    /// Cast the x & y coordinates to the given type.
    /// 
    /// ### Generic Arguments
    /// * `U`: [num::Num] + [num::NumCast] - The new type of x & y.
    pub fn cast<U: num::Num + num::NumCast>(self) -> Vector<U> {
        Vector(num::cast(self.0).unwrap(), num::cast(self.1).unwrap())
    }

    /// Get the scalar product of this vector and rhs.
    /// The scalar product of two vectors means multiplying each coordinate of lhs with 
    /// the opposite coordinate of lhs and adding them all together.
    /// 
    /// ### Generic Arguments
    /// * `U`: [num::NumCast] - The type of the scalar product.
    pub fn scalar<U: num::NumCast>(self, rhs: Vector<T>) -> U {
        num::cast(self.0 * rhs.0 + self.1 * rhs.1).unwrap()
    }

    /// Get the magnitude / length of this vector.
    /// The length can be calculated by getting the square root of 
    /// the scalar product with this vector as both operands.
    /// 
    /// ### Generic Arguments
    /// * `U`: [num::Float] - The type of magnitude.
    pub fn mag<U: num::Float>(self) -> U {
        let scalar: U = self.scalar(self);
        scalar.sqrt()
    }

    /// Normalize the vector by setting its magnitude / length to 1,
    /// turning it into what is called a unit vector.
    /// This is done by dividing both coordinates with the magnitude / length.
    /// This vector does not get modified. This function returns a copy of normalized self.
    /// 
    /// ### Generic Arguments
    /// * `U`: [num::Float] - The type of the result vector.
    pub fn norm<U: num::Float>(self) -> Vector<U> {
        let mag = self.mag::<U>();
        let mut new = Vector::<U>(num::cast(self.0).unwrap(), num::cast(self.1).unwrap());
        if mag > num::cast(0).unwrap() {
            new.0 = new.0 / mag;
            new.1 = new.1 / mag;
        }
        new
    }
}


impl<T: num::Num> From<(T, T)> for Vector<T>  {
    fn from(val: (T, T)) -> Self {
        Vector(val.0, val.1)
    }
}

// <========================================>
// <=== Arith. Operator with rhs: Vector ===>
// <========================================>

// Add with rhs: Vector
impl<T: num::Num> Add<Vector<T>> for Vector<T> {
    type Output = Self;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
// Sub with rhs: Vector
impl<T: num::Num> Sub<Vector<T>> for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
// Mul with rhs: Vector
impl<T: num::Num> Mul<Vector<T>> for Vector<T> {
    type Output = Self;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}
// Div with rhs: Vector
impl<T: num::Num> Div<Vector<T>> for Vector<T> {
    type Output = Self;
    fn div(self, rhs: Vector<T>) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}


// <===================================>
// <=== Arith. Operator with rhs: T ===>
// <===================================>

// Add with rhs: T
impl<T: num::Num + Copy> Add<T> for Vector<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}
// Sub with rhs: T
impl<T: num::Num + Copy> Sub<T> for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}
// Mul with rhs: T
impl<T: num::Num + Copy> Mul<T> for Vector<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
// Div with rhs: T
impl<T: num::Num + Copy> Div<T> for Vector<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}


// <============================================>
// <=== Assignment Operator with rhs: Vector ===>
// <============================================>

// Add assign with rhs: Vector
impl<T: num::Num + AddAssign> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
// Sub assign with rhs: Vector
impl<T: num::Num + SubAssign> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}
// Mul assign with rhs: Vector
impl<T: num::Num + MulAssign> MulAssign for Vector<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}
// Div assign with rhs: Vector
impl<T: num::Num + DivAssign> DivAssign for Vector<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}


// <============================================>
// <=== Assignment Operator with rhs: T ========>
// <============================================>

// Add assign with rhs: T
impl<T: num::Num + AddAssign + Copy> AddAssign<T> for Vector<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
        self.1 += rhs;
    }
}
// Sub assign with rhs: T
impl<T: num::Num + SubAssign + Copy> SubAssign<T> for Vector<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs;
        self.1 -= rhs;
    }
}
// Mul assign with rhs: T
impl<T: num::Num + MulAssign + Copy> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}
// Div assign with rhs: T
impl<T: num::Num + DivAssign + Copy> DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}


// <======================>
// <=== Unary Operator ===>
// <======================>

impl<T: num::Num + Neg<Output = T>> Neg for Vector<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}
