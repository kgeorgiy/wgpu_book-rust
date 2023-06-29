trait HList {
}

impl HList for () {
}

impl<H, T> HList for (H, T) where T: HList {
}

fn main() {
    println!("hello");
}