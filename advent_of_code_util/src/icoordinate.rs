use std::{
    ops::{Add, Sub},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct ICoordinate {
    pub x: isize,
    pub y: isize,
}
impl FromStr for ICoordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .map(|num| num.parse::<isize>().unwrap())
            .collect_tuple::<(isize, isize)>()
            .unwrap();
        Ok(ICoordinate { x, y })
    }
}
impl Add for ICoordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ICoordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for ICoordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ICoordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
