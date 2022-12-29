use i_am_closer as my_lib;
use std::io;

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let ans = my_lib::challenge_23(&input);
    println!("{:?}", ans);

    Ok(())
}
