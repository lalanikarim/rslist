
use std::{fmt::*, ops::*};

#[derive(Debug)]
pub struct List<T> {
    head: Option<T>,
    tail: Option<Box<Self>>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<T> List<T> {
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.head.is_some() && self.tail.is_none()
    }

    pub fn leaf(element: T) -> Self {
        Self {
            head: Some(element),
            tail: None,
        }
    }

    pub fn get_head(&self) -> Option<&T> {
        self.head.as_ref()
    }

    pub fn get_tail(&self) -> Option<&Box<Self>> {
        self.tail.as_ref()
    }

    pub fn push(&mut self, element: T) -> &mut Self {
        let head = self.head.take();
        let tail = self.tail.take();

        self.head = Some(element);
        self.tail = if head.is_none() && tail.is_none() {
            None
        } else {
            Some(Box::new(Self { head, tail }))
        };
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        let tail = self.tail.take();
        match tail {
            None => {
                self.head = None;
                self.tail = None;
            }
            Some(list) => {
                self.head = list.head;
                self.tail = list.tail;
            }
        }
        head
    }

    pub fn last(&self) -> &Self {
        let mut pointer = self;
        loop {
            if pointer.is_leaf() {
                return pointer;
            }
            match &pointer.tail {
                None => return pointer,
                Some(ref list) => {
                    pointer = list;
                }
            }
        }
    }

    pub fn last_mut(&mut self) -> &mut Self {
        let pointer = self;
        if pointer.is_leaf() || pointer.is_empty() {
            return pointer;
        } else {
            return pointer.tail.as_mut().unwrap().last_mut();
        }
    }

    pub fn append(&mut self, element: T) -> &mut Self {
        if self.is_empty() {
            self.head = Some(element);
        } else {
            let last = self.last_mut();
            last.tail = Some(Box::new(List::leaf(element)));
        }
        self
    }

    pub fn at(&self, index: usize) -> &Option<T> {
        if index == 0 {
            &self.head
        } else {
            match &self.tail {
                None => &None,
                Some(list) => list.at(index - 1),
            }
        }
    }

    pub fn depth(&self) -> usize {
        let mut d = 0usize;
        let mut list = self;

        loop {
            if list.is_empty() {
                break;
            } else {
                if list.is_leaf() {
                    d = d + 1;
                    break;
                } else {
                    list = list.tail.as_ref().unwrap();
                    d = d + 1;
                }
            }
        }
        d
    }

    pub fn insert_at(&mut self, index: usize, element: T) {
        if index == 0 {
            self.push(element);
        } else {
            if index >= self.depth() {
                self.append(element);
            } else {
                let right = self.split(2);
                self.append(element);
                self.splice(*right.unwrap());
            }
        }
    }

    pub fn split(&mut self, index: usize) -> Option<Box<Self>> {
        if index == 0 && index >= self.depth() - 1 {
            None
        } else {
            let pointer = self;
            if index == 1 {
                pointer.tail.take()
            } else {
                pointer.tail.as_mut().unwrap().split(index - 1)
            }
        }
    }

    pub fn splice(&mut self, other: Self) -> &mut Self {
        self.last_mut().tail = Some(Box::new(other));
        self
    }

    pub fn splice_before(&mut self, other: Self) -> &mut Self {
        let head = self.head.take();
        let tail = self.tail.take();
        self.head = other.head;
        self.tail = other.tail;
        if head.is_some() {
            self.append(head.unwrap());
            if tail.is_some() {
                self.splice(*tail.unwrap());
            }
        }
        self
    }
}

impl<T> Shl<List<T>> for List<T> {
    type Output = Self;

    fn shl(mut self, rhs: List<T>) -> Self::Output {
        self.splice_before(rhs);
        self
    }
}

impl<T> Shl<T> for List<T> {
    type Output = Self;

    fn shl(mut self, rhs: T) -> Self::Output {
        self.push(rhs);
        self
    }
}

impl<T> Shr<List<T>> for List<T> {
    type Output = Self;

    fn shr(mut self, rhs: List<T>) -> Self::Output {
        self.splice(rhs);
        self
    }
}

impl<T> Shr<T> for List<T> {
    type Output = Self;

    fn shr(mut self, rhs: T) -> Self::Output {
        self.append(rhs);
        self
    }
}

impl<T: std::cmp::PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.tail.as_ref().eq(&other.tail.as_ref())
    }
}

impl<T> PartialEq<Vec<T>> for List<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        todo!()
    }
}

impl<T:Copy> From<Vec<T>> for List<T> {
    fn from(input: Vec<T>) -> Self {
        let mut list:Self = List::default();

        for idx in 0..input.len() {
            list.append(input.get(idx).take().unwrap().to_owned());
        }
        list
    }
}

#[allow(unused_must_use)]
impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut head = &self.head;
        let mut tail = &self.tail;
        loop {
            match head {
                Some(item) => write!(f, "{}::", item),
                None => return write!(f, "Empty"),
            };
            match &tail {
                Some(list) => {
                    head = &list.head;
                    tail = &list.tail;
                }
                None => return write!(f, "End"),
            };
        }
    }
}
