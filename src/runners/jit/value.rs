use crate::Register;
use std::fmt::{self, Display};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Imm(u64),
    Register(usize),
    Add(Rc<Value>, Rc<Value>),
    Sub(Rc<Value>, Rc<Value>),
}

impl Default for Value {
    fn default() -> Value {
        Value::zero()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Value {
        unimplemented!()
    }
}

impl BitAnd for Value {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Value {
        unimplemented!()
    }
}

impl BitOr for Value {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Value {
        unimplemented!()
    }
}

impl BitXor for Value {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Value {
        unimplemented!()
    }
}

impl Shl<Value> for Value {
    type Output = Self;

    fn shl(self, rhs: Self) -> Value {
        unimplemented!()
    }
}

impl Shr<Value> for Value {
    type Output = Self;

    fn shr(self, rhs: Self) -> Value {
        unimplemented!()
    }
}

impl Register for Value {
    // For now we only support JIT on 64 bit RISC-V machine
    const BITS: usize = 64;
    const SHIFT_MASK: usize = 0x3F;

    fn zero() -> Value {
        Value::Imm(0)
    }

    fn one() -> Value {
        Value::Imm(1)
    }

    fn min_value() -> Value {
        Value::Imm(u64::min_value())
    }

    fn max_value() -> Value {
        Value::Imm(u64::max_value())
    }

    fn eq(&self, other: &Value) -> Value {
        unimplemented!()
    }

    fn lt(&self, other: &Value) -> Value {
        unimplemented!()
    }

    fn lt_s(&self, other: &Value) -> Value {
        unimplemented!()
    }

    fn logical_not(&self) -> Value {
        unimplemented!()
    }

    fn cond(&self, true_value: &Value, false_value: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_add(&self, rhs: &Value) -> Value {
        // This is a very naive constant elimination optimization
        // served as a PoC purpose
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (self, rhs) {
            let imm = (*imm1).overflowing_add(*imm2).0;
            return Value::Imm(imm);
        }
        Value::Add(Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn overflowing_sub(&self, rhs: &Value) -> Value {
        if let (Value::Imm(imm1), Value::Imm(imm2)) = (self, rhs) {
            let imm = (*imm1).overflowing_sub(*imm2).0;
            return Value::Imm(imm);
        }
        Value::Sub(Rc::new(self.clone()), Rc::new(rhs.clone()))
    }

    fn overflowing_mul(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_div(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_rem(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_div_signed(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_rem_signed(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_mul_high_signed(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_mul_high_unsigned(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn overflowing_mul_high_signed_unsigned(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn signed_shl(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn signed_shr(&self, rhs: &Value) -> Value {
        unimplemented!()
    }

    fn zero_extend(&self, start_bit: &Value) -> Value {
        unimplemented!()
    }

    fn sign_extend(&self, start_bit: &Value) -> Value {
        unimplemented!()
    }

    fn to_i8(&self) -> i8 {
        unimplemented!()
    }

    fn to_i16(&self) -> i16 {
        unimplemented!()
    }

    fn to_i32(&self) -> i32 {
        unimplemented!()
    }

    fn to_i64(&self) -> i64 {
        unimplemented!()
    }

    fn to_isize(&self) -> isize {
        unimplemented!()
    }

    fn to_u8(&self) -> u8 {
        unimplemented!()
    }

    fn to_u16(&self) -> u16 {
        unimplemented!()
    }

    fn to_u32(&self) -> u32 {
        unimplemented!()
    }

    fn to_u64(&self) -> u64 {
        unimplemented!()
    }

    fn to_usize(&self) -> usize {
        unimplemented!()
    }

    fn from_i8(v: i8) -> Value {
        unimplemented!()
    }

    fn from_i16(v: i16) -> Value {
        unimplemented!()
    }

    fn from_i32(v: i32) -> Value {
        Value::Imm(i64::from(v) as u64)
    }

    fn from_i64(v: i64) -> Value {
        unimplemented!()
    }

    fn from_isize(v: isize) -> Value {
        unimplemented!()
    }

    fn from_u8(v: u8) -> Value {
        unimplemented!()
    }

    fn from_u16(v: u16) -> Value {
        unimplemented!()
    }

    fn from_u32(v: u32) -> Value {
        unimplemented!()
    }

    fn from_u64(v: u64) -> Value {
        unimplemented!()
    }

    fn from_usize(v: usize) -> Value {
        Value::Imm(v as u64)
    }
}
