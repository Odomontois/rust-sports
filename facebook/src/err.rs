use std::{error::Error};

pub type R<A> = Result<A, Box<dyn Error>>;
pub type T = R<()>;
pub type S = R<String>;

pub type RV<X, E> = Result<Vec<X>, E>;
