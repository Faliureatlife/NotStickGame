fn draw(pix: &mut [u8]){// world, spritelist
    //read the entire pixel map with fs::read
    //unwrap to take from result<Vec[u8],e> to Vec[u8]
    let colors = fs::read("WorldData/Houses").unwrap();
    //iterator var
    i:u32 = 0;
    for pixel in pix.chunks_exact_mut(4) {
        //i*6 is the byte chunk
        let pos = i*6;
        //i hate this part
        //First 4 bits
        //r is mutable because its added on to
        //takes u8 at pos and turns into utf8
        //unwrap to take from Result to String
        let mut r = str::from_utf8(&colors[pos]).unwrap();
        //Take rest of byte, convert to char, and push onto r
        r.push(str::from_utf8(&colors[pos + 1]).unwrap())

        let mut g = str::from_utf8(&colors[pos + 2]).unwrap();
        g.push(str::from_utf8(&colors[pos + 3]).unwrap())

        let mut b = str::from_utf8(&colors[pos + 4]).unwrap();
        b.push(str::from_utf8(&colors[pos + 5]).unwrap())
        //Shoves string pointer into u8 sized hole
        pixel[0] = r.parse().unwrap(); // R
        pixel[1] = g.parse().unwrap(); // G
        pixel[2] = b.parse().unwrap(); // B
        pixel[3] = 0xFF; // A
        i += 1;
    }
}
