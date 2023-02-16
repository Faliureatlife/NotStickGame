use std::{env};
use std::fs::*;

fn main() {
    let binding = read_to_string("0.txt").unwrap();
    let a = binding.as_str();
    let mut d:Vec<&str> = vec![];
    for pix in 0..486{
        let (_,bsd) = a.split_at(pix * 6);
        let (b,_) = bsd.split_at(6);
        d.push(b);
        d.push(b);
    }

    let mut bindingg = format!("{:?}", &d);
    bindingg = bindingg.replace(&[' ', '\'',',','"','[',']','\\'][..],"");
    let e = bindingg.as_str();
    let mut g:Vec<&str> = vec![];
    for pix in 0..27{
        let (_, asd) = e.split_at(216 * pix);
        let (f,_) = asd.split_at(216);
        g.push(f);
        g.push(f);
    }
    let mut h = format!("{:?}", g);
    h = h.replace(&[' ', '\'',',','"','[',']','\\'][..],"");
    write("done", &h).unwrap();
}
