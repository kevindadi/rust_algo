pub fn move2tower(height: u32, src_p: &str,
                    des_p: &str, mid_p: &str) {
    if height >= 1 {
        move2tower(height - 1, src_p, mid_p, des_p);
        println!("moving disk from {} to {}", src_p, des_p);
        move2tower(height - 1, mid_p, des_p, src_p);
    }                
}