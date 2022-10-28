fn draw(pix: &mut [u8], world, Spritelist: [sprite]){
    colors = fs::read("WorldData/Houses").unwrap();
    i:u32 = 0;
    for pixel in pix.chunks_exact_mut(4) {
        r = colors[6 * i].to
        pixel[0] = r; // R
        pixel[1] = g; // G
        pixel[2] = b; // B
        pixel[3] = 0xFF; // A
        i += 1;
    }
}
