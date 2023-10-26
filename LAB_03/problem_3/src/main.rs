#[derive(Debug)]
enum ERR {
    OverflowSum,
    OverflowMult,
}

fn sum(val1: u32, val2: u32) -> Result<u32, ERR> {
    let s = val1 + val2;
    if s > std::u16::MAX as u32 {
        Err(ERR::OverflowSum)
    } else {
        Ok(s)
    }
}

fn mult(val1: u32, val2: u32) -> Result<u32, ERR> {
    let m = val1 * val2;
    if m > std::u16::MAX as u32 {
        Err(ERR::OverflowMult)
    } else {
        Ok(m)
    }
}

fn f() -> Result<u32,ERR>{
    let varSum=sum(100,1)?;
    let varMult=mult(1000, 1)?;
    Ok(varSum+varMult)
}

fn main() {
    println!("{:?}, {:?}", sum(10, 20), mult(10, 20));
    println!("{:?}, {:?}", sum(1000, 20303), mult(1000, 20303));
    println!("{:?}",f());
}
