#![feature(generic_const_exprs)]
#![feature(unsized_const_params)]
#![feature(iterator_try_collect)]

use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::Add;
use std::fmt::Debug;

pub enum Assert<const COND: bool>{}
pub trait True{}
impl True for Assert<true>{}

pub const fn product(dims: &'static [usize]) -> usize {
    let mut total = 1;
    let mut index = 0;
    while index < dims.len() {
        total *= dims[index];
        index += 1;
    }
    total
}

pub const fn iota<const P: usize>() -> [usize; P] {
    let mut current = 0;
    let mut out = [0; P];
    while current < P {
        out[current] = current;
        current += 1;
    }
    out
}




pub trait ArrayTrait {
    type Item: Copy;
    type Data: IntoIterator<Item = Self::Item> + AsRef<[Self::Item]>;

    const DIMS: &'static [usize];

    const DEPTH: usize = Self::DIMS.len();
    const DEPTH_SMALLER: usize = Self::DEPTH - 1;  
    const SIZE: usize = product(Self::DIMS);

    fn fill(fill_value: Self::Item) -> Self;
    fn data(&self) -> &Self::Data;
    fn reshape<const NDIMS: &'static [usize]>(self) -> impl ArrayTrait
        where Assert<{product(Self::DIMS) == product(NDIMS)}>: True;

    // fn iota(self) -> impl ArrayTrait;

}


#[derive(Debug)]
pub struct Array<T, const DIMS: &'static [usize]> 
where [(); product(DIMS)]:
{
    data: [T; product(DIMS)],
}

impl <T: Copy, const DIMS: &'static [usize]> ArrayTrait for Array<T, DIMS> 
    where [(); product(DIMS)]:,
    {
        type Item = T;
        type Data = [T; product(DIMS)];
        const DIMS: &'static [usize] = DIMS;

        fn fill(fill_value: Self::Item) -> Self {
            Self { data: [fill_value; product(DIMS)]}
        }


        fn data(&self) -> &Self::Data {
           &self.data 
        }

        fn reshape<const NDIMS: &'static [usize]>(self) -> impl ArrayTrait
            where [(); product(NDIMS)]:,
        {
            unsafe {
                let no_drop = ManuallyDrop::new(self);
                let sp = &*no_drop as *const Array<T, DIMS> as *const Array<T, NDIMS>;
                std::ptr::read(sp)
            }
            
        }

    }

impl <const DIMS: &'static [usize]> Array<usize, DIMS> 
    where [(); product(DIMS)]:,
    {
        fn iota() -> Self
        {
            Self {data: iota::<{product(DIMS)}>()}
        }
    }

impl<T: Add<Output = T> + Copy + Debug, const DIMS: &'static [usize]> Add for Array<T, DIMS>
    where [(); product(DIMS)]:
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: self.data.iter().zip(rhs.data).map(|(l,r)| *l + r).collect::<Vec<_>>().try_into().unwrap()
        }
    }
}

impl<T: Add<Output = T> + Copy + Debug, const DIMS: &'static [usize]> Add<T> for Array<T, DIMS>
    where [(); product(DIMS)]:
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            data: self.data.iter().map(|l| *l + rhs).collect::<Vec<_>>().try_into().unwrap()
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let a1: Array<usize, {&[5,4]}> = Array{data: [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,7,8,9,1,2]};
        let a2 = a1.reshape::<{&[2,5,2]}>();
        let a3: Array<usize, {&[5,5,5]}> = Array::iota();
        let a3: Array<_, {&[5,5,5]}> = Array::fill(1.2);
    }
}
