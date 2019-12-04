use std::ops::{Add, AddAssign};
use log::{debug, error, info, trace, warn};
use std::convert::TryInto;

extern crate num;
use num::{PrimInt, range, NumCast};

pub struct RLEArray<T>
where   T: PartialEq,
        T: Copy
{
    pub data: Vec<(T, u8)>
}

impl<T> RLEArray<T>
where   T: PartialEq,
        T: Copy
{
    pub fn new() -> RLEArray<T> {
        RLEArray {
            data: Vec::new()
        }
    }

    pub fn with_capacity(default: T, capacity: usize) -> RLEArray<T> {
        let mut data = Vec::<(T, u8)>::new();

        let full_sets = capacity / (std::u8::MAX as usize);
        let remainder = capacity - full_sets * (std::u8::MAX as usize);

        for _i in 0..full_sets {
            data.push((default, std::u8::MAX));
        }
        data.push((default, remainder.try_into().unwrap()));

        RLEArray {
            data: data
        }
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn len(&self) -> usize {
        let mut total_size: usize = 0;
        for i in 0..(self.data_len()) {
            let (_data, count) = self.data[i];
            total_size += count as usize;
        }
        total_size
    }

    pub fn push(&mut self, value: T) {
        let len = self.data_len();
        if len == 0 {
            self.data.push((value, 1));
        } else {
            let (data, _count) = self.data[len - 1];
            if data == value {
                //This is supposed to stop overflows.
                //NOTE: Untested
                if self.data[len - 1].1 == std::u8::MAX {
                    self.data.push((value, 1));
                }
                else {
                    self.data[len - 1].1 += 1;
                }
            } else {
                self.data.push((value, 1));
            }
        }
    }

    pub fn insert(&mut self, value: T, idx: usize) -> Result<(), &'static str> {
        let mut total_idx = 0;
        for i in 0..(self.data_len()) {
            let (data, count) = self.data[i];
            total_idx += count as usize;
            if idx < total_idx {
                //Found the right `group`
                if data == value {
                    if self.data[i].1 == std::u8::MAX {
                        self.data.insert(i + 1, (value, 1));
                    } else {
                        self.data[i].1 += 1;
                    }
                } else {
                    let left_count = (count as usize) - (total_idx - idx);
                    let right_count = (count as usize) - left_count;
                    if left_count > 0 {
                        self.data[i].1 = left_count.try_into().unwrap();
                        self.data.insert(i + 1, (data, right_count.try_into().unwrap()));
                        self.data.insert(i + 1, (value, 1));
                    } else {
                        self.data.remove(i);
                        if right_count > 1 {
                            self.data.insert(i, (data, right_count.try_into().unwrap()));
                            self.data.insert(i, (value, 1));
                        }
                    }
                }
                return Ok(());
            }
        }

        Err("Index out of range")
    }

    pub fn remove(&mut self, idx: usize) -> Result<(), &'static str> {
        let mut total_idx = 0;
        for i in 0..(self.data_len()) {
            let (_data, count) = self.data[i];
            total_idx += count as usize;
            if idx < total_idx {
                //Found the right index
                if count > 1 {
                    self.data[i].1 -= 1;
                    return Ok(());
                } else {
                    self.data.remove(i);
                    return Ok(());
                }
            }
        }

        Err("Index out of range")
    }

    pub fn set(&mut self, value: T, idx: usize) -> Result<(), &'static str> {
        if idx < self.len() - 1 {
            let mut total_idx = 0;
            for i in 0..(self.len()) {
                let (_data, count) = self.data[i];
                total_idx += count as usize;
                if idx < total_idx {
                    if count > 1 {
                        self.data[i].1 -= 1;
                        self.insert(value, idx)?;
                    } else {
                        self.data[idx].0 = value; //We can simply overwrite the value
                    }
                    return Ok(());
                }
            }
        }

        Err("Index out of range")
    }

    pub fn get(&self, idx: usize) -> Result<T, &'static str> {
        let mut total_idx = 0;
        for i in 0..(self.data_len()) {
            let (data, count) = self.data[i];
            // if idx < total_idx { return Ok(data) }
            total_idx += count as usize;
            if idx < total_idx { return Ok(data) }
        }

        Err("Index out of range")
    }
}

impl<T> RLEArray<T>
where   T: PartialEq,
        T: Copy,
        T: std::fmt::Display
{
    pub fn get_pretty_string(&self) -> String {
        let mut result = String::new();

        for i in 0..(self.data_len()) {
            let (data, count) = self.data[i];
            result.push_str(&*format!("[{data}-{count}]; ", data = data, count = count));
        }

        result
    }
}
