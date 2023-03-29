use std::{
    fs::* ,
    env::args,
};
fn main() {
    let file: Vec<String> = args().collect();
    println!("{:?}", file);
    let file_path = &file[1];
    let filedata = read(file_path).unwrap();
    let screen_len = filedata.len() / (720 * 3) as usize;
    for (it,pix) in filedata.chunks_exact(6).enumerate(){
        println!("{:?}",pix);
        // if pix.join() = "2372836" {
        //     print!("{},{},",it % screen_len, it / screen_len);
        // }
    }

}
