fn main() {
    let mut buf = String::new();
    let mut i = 1;
    loop {
        buf.clear();
        let len = std::io::stdin().read_line(&mut buf).expect("failed to read stdin");

        if len == 0 {
            break;
        }

        print!("{:>5} {}", i, &buf);
        i += 1;
    }
}
