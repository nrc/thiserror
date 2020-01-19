#![feature(specialization)]

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("...")]
pub struct ErrorStruct {
    #[from]
    source: io::Error,
}

#[derive(Error, Debug)]
#[error("...")]
pub struct ErrorTuple(#[from] io::Error);

#[derive(Error, Debug)]
#[error("...")]
pub enum ErrorEnum {
    Test {
        #[from]
        source: io::Error,
    },
}

#[derive(Error, Debug)]
#[error("...")]
#[unwrap]
pub enum Many {
    Any(#[from] anyhow::Error),
    Io(#[from] io::Error),
}

fn assert_impl<T: From<io::Error>>() {}

#[test]
fn test_from() {
    assert_impl::<ErrorStruct>();
    assert_impl::<ErrorTuple>();
    assert_impl::<ErrorEnum>();
    assert_impl::<Many>();
    assert_impl::<Wrapped>();
}

#[derive(Error, Debug)]
#[error("...")]
pub enum Wrapped {
    Io(#[from] io::Error),
    ManyV(#[from_unwrap] Many),
}

#[test]
fn test_from_wrapped() {
    let e: Many = io::Error::from_raw_os_error(1).into();
    let e: Wrapped = e.into();
    match e {
        Wrapped::Io(_) => {}
        Wrapped::ManyV(_) => panic!(),
    }

    let e: anyhow::Error = ErrorTuple(io::Error::from_raw_os_error(1)).into();
    let e: Many = e.into();
    let e: Wrapped = e.into();
    match e {
        Wrapped::Io(_) => panic!(),
        Wrapped::ManyV(_) => {}
    }
}
