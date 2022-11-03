fn draw(colors: &mut [u8]){
    //read the entire pixel map with fs::read
    //unwrap to take from result<Vec[u8],e> to Vec[u8]

    //iterator var
     let mut i:usize = 0;
    for pixel in colors.chunks_exact_mut(4) {
        //i*6 is the byte chunk
        let pos = i*6;
        //i hate this part
        //First 4 bits
        //r is mutable because its added on to
        //takes u8 at pos and turns into utf8
        //unwrap to take from Result to

        let r: Vec<u8> = Vec::new();
        r.push(colors[pos]);
        r.push(colors[pos+1]);
        let mut red = std::str::from_utf8(&r).unwrap();

        let g: Vec<u8> = Vec::new();
        g.push(colors[pos+2]);
        g.push(colors[pos+3]);
        let mut green = std::str::from_utf8(&g).unwrap();

        let b: Vec<u8> = Vec::new();
        b.push(colors[pos+4]);
        b.push(colors[pos+5]);
        let mut blue = std::str::from_utf8(&b).unwrap();

        //Shoves string pointer into u8 sized hole
        pixel[0] = red.parse::<u8>().unwrap(); // R
        pixel[1] = green.parse::<u8>().unwrap(); // G
        pixel[2] = blue.parse::<u8>().unwrap(); // B
        pixel[3] = 0xFF; // A
        i += 1;
    }
}
