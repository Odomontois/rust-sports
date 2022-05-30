use std::{error::Error, fs::File, io::Read};

fn ul(condition: bool) -> Result<(), Box<dyn Error>> {
    let mut file: File;
    let mut slice: &[u8];

    let source: &mut dyn Read = if condition {
        file = File::open("./path")?;
        &mut file
    } else {
        slice = &[72, 101, 108, 108, 111];
        &mut slice
    };

    drop(source);

    Ok(())
}

fn no_ul(condition: bool) -> Result<(), Box<dyn Error>> {
    let source = if condition {
        either::Left(File::open("./path")?)
    } else {
        either::Right(&[72, 101, 108, 108, 111][..])
    };

    drop(source);

    Ok(())
}
