use std::{marker::PhantomData, io::Error};

// Aliases
pub type SaveType<T> = Save<fn(T) -> Result<(), Error>, T>;
//pub type OptF = fn(x: &str) -> Result<(), Error>;
//type FnPtr<T> = for<'a> fn(t: &'a T) -> &'a str;
pub type FnPtr<T> = fn(t: T) -> String;

pub trait ISave<T> {
    fn save(&self, options: T) -> Result<(), Error>;
}

pub struct Save<F, T>
where F: Fn(T) -> Result<(), Error>
{
    _phantom: PhantomData<T>,
    func: F,
    t: FnPtr<T>,
}

impl<F, T> Save<F, T>
where F: Fn(T) -> Result<(), Error> {
    pub fn new(f: F, ptr: FnPtr<T>) -> Self {
        Self 
        { 
            func: f,
            t: ptr,
            _phantom: PhantomData  
        }
    }

    pub fn execute_func(&self, t: T) -> Result<(), Error> {
        (self.func)(t)
    }

    pub fn execute_t(&self, t: T) -> Result<String, Error> {
        Ok((self.t)(t))
    }
}

pub struct SaveParams{
    pub is_cool: bool
}

pub struct Saver{
    //save: Save<fn(SaveParams) -> Result<(), Error>, SaveParams>
    save: SaveType<SaveParams>
}

impl Saver {
    //fn new(saver: Save<fn(SaveParams) -> Result<(), Error>, SaveParams>) -> Self {
    pub fn new(saver: SaveType<SaveParams>) -> Self {
        Saver {save: saver }
    }
}

impl ISave<SaveParams> for Saver {
    fn save(&self, options: SaveParams) -> Result<(), Error> {
        (self.save.func)(options)
    }
}