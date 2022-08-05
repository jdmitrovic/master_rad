fn main() {
   println!("{}", num());
}

fn num() -> &'static u32 {
    static N: u32 = 1234;
    return &N;
}
