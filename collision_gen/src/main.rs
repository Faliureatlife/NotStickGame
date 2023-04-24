use std::{
    fs::* ,
    env::args,
};
fn main() {
    let file: Vec<String> = args().collect();
    println!("{:?}", file);
    let file_path = &file[1];
    let filedata = read (file_path).unwrap();
    let screen_len = filedata.len() / (540 * 6) as usize;
    for (it,pix) in filedata.chunks_exact(6).enumerate(){
        if std::str::from_utf8(pix).unwrap().to_string() == "ED1C24" {
            //print statement i actually want
            print!("{},{},",it % screen_len, it / screen_len);
        }
    }
}