extern crate num;

use std::ops::Add;
use std::ops::Neg;

use num::rational::BigRational;
use num::bigint::BigInt;

#[derive(Clone)]
struct Capture {
    r: Vec<Real>,
    q: Vec<BigRational>,
}

impl Capture {
    fn of(r: Real) -> Self {
        Capture { r: vec![r], q: vec![] }
    }

    fn of_reals(rs: Vec<Real>) -> Self {
        Capture { r: rs, q: vec![] }
    }

    fn of_rats(qs: Vec<BigRational>) -> Self {
        Capture { r: vec![], q: qs }
    }
}

struct Real {
    env: Capture,
    f: fn(BigInt, Capture) -> BigRational,
}

impl Clone for Real {
    fn clone(&self) -> Real {
        Real {
            env: self.env.clone(),
            f: self.f,
        }
    }
}

impl Real {
    fn approx(self, n: BigInt) -> BigRational {
        (self.f)(n, self.env)
    }
}

impl From<i32> for Real {
    fn from(n: i32) -> Self {
        fn new_n(_: BigInt, env: Capture) -> BigRational {
            env.q[0].clone()
        }
        Real {
            f: new_n,
            env: Capture::of_rats(vec![BigRational::from(BigInt::from(n))])
        }
    }
}

impl Neg for Real {
    type Output = Real;
    fn neg(self) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            -env.r[0].clone().approx(n)
        }
        Real { f: new_n, env: Capture::of(self) }
    }
}

impl<'a> Neg for &'a Real {
    type Output = Real;
    fn neg(self) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            -env.r[0].clone().approx(n)
        }
        Real { f: new_n, env: Capture::of(self.clone()) }
    }
}

impl<'a> Neg for &'a mut Real {
    type Output = Real;
    fn neg(self) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            -env.r[0].clone().approx(n)
        }
        Real { f: new_n, env: Capture::of(self.clone()) }
    }
}

impl Add for Real {
    type Output = Real;
    fn add(self, b: Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self, b]) }
    }
}

impl<'a> Add<&'a Real> for Real {
    type Output = Real;
    fn add(self, b: &'a Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self, b.clone()]) }
    }
}

impl<'a> Add<&'a mut Real> for Real {
    type Output = Real;
    fn add(self, b: &'a mut Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self, b.clone()]) }
    }
}

impl<'a> Add<Real> for &'a Real {
    type Output = Real;
    fn add(self, b: Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self.clone(), b]) }
    }
}

impl<'a, 'b> Add<&'a Real> for &'b Real {
    type Output = Real;
    fn add(self, b: &Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self.clone(), b.clone()]) }
    }
}

impl<'a, 'b> Add<&'a mut Real> for &'b Real {
    type Output = Real;
    fn add(self, b: &mut Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self.clone(), b.clone()]) }
    }
}

impl<'a> Add<Real> for &'a mut Real {
    type Output = Real;
    fn add(self, b: Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self.clone(), b]) }
    }
}

impl<'a, 'b> Add<&'a Real> for &'b mut Real {
    type Output = Real;
    fn add(self, b: &Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self.clone(), b.clone()]) }
    }
}

impl<'a, 'b> Add<&'a mut Real> for &'b mut Real {
    type Output = Real;
    fn add(self, b: &mut Real) -> Self::Output {
        fn new_n(n: BigInt, env: Capture) -> BigRational {
            let r0 = env.r[0].clone();
            let r1 = env.r[1].clone();
            r0.approx(BigInt::from(2) * &n) + r1.approx(BigInt::from(2) * n)
        }
        Real { f: new_n, env: Capture::of_reals(vec![self.clone(), b.clone()]) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg() {
        let r0 = Real::from(0);
        let r1 = Real::from(1);
        let q0 = BigRational::from(BigInt::from(0));
        let q1 = BigRational::from(BigInt::from(1));
        assert_eq!(-q1, (-r1).approx(BigInt::from(0)));
        assert_eq!(q0, (-r0).approx(BigInt::from(0)));
    }

    #[test]
    fn add() {
        let r0 = Real::from(0);
        let r1 = Real::from(1);
        let r2 = Real::from(2);
        let q0 = BigRational::from(BigInt::from(0));
        let q1 = BigRational::from(BigInt::from(1));
        let q2 = BigRational::from(BigInt::from(2));
        let q3 = BigRational::from(BigInt::from(3));
        let q4 = BigRational::from(BigInt::from(4));
        assert_eq!(q2, (&r1 + &r1).approx(BigInt::from(0)));
        assert_eq!(q1, (&r0 + &r1).approx(BigInt::from(0)));
        assert_eq!(q3, (&r1 + &r2).approx(BigInt::from(0)));
        assert_eq!(q4, (&r2 + &r2).approx(BigInt::from(0)));
        assert_eq!(q0, (&r0 + &r0).approx(BigInt::from(0)));
    }
}
