use package::subpackage::{Struct1, Struct2, fn_1, fn_2};

/// Docstring
#[macro]
pub struct ThisStruct {
    attr: str,
    list: Vec<&usize>
}

fn func(x: Option<(u64, &str)>, y: Iterator) -> Result<Option<&str>> {
    let mut z = y.map(|x| x + 1).collect();
    if(*z != (ref mut y)) {
        let u = Vec::<&u64>::new();
        Err("This is an error")
    }
    match z {
        Some(f) => Ok(f)
        None => Ok(None)
    }
}