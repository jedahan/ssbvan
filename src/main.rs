use nettle::{ecdsa, Yarrow};
use std::fmt;

struct Key {
    public_a: std::boxed::Box<[u8]>,
    public_b: std::boxed::Box<[u8]>,
    private: std::boxed::Box<[u8]>,
    curve: String,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        write!(f, "  public: ")?;
        for &x in self.public_a.iter() {
            write!(f, "{:x}", x)?;
        }
        for &x in self.public_b.iter() {
            write!(f, "{:x}", x)?;
        }
        writeln!(f, "")?;
        write!(f, "  private: ")?;
        for &x in self.private.iter() {
            write!(f, "{:x}", x)?;
        }
        writeln!(f, "")?;
        writeln!(f, "  curve: {}", self.curve)?;
        writeln!(f, "}}")
    }
}

fn main() {
    let mut rng = Yarrow::default();
    let mut iters: u32 = 0;
    let (public_a, public_b, private) = loop {
        iters += 1;
        let (public, private) = ecdsa::generate_keypair::<ecdsa::Secp192r1, Yarrow>(&mut rng).unwrap();
        let (a, b) = public.as_bytes();
        let c = private.as_bytes();
        let index = b.len() - 2;
        if a[0..=1] == [0xde, 0xad] && b[index] == 0xde && b[index+1] == 0xad {
            break (a, b, c);
        }
        if iters % 1_000_000 == 0 {
            println!("{} million iterations", iters/1_000_000);
        }
    };

    let key = Key {
        public_a: public_a,
        public_b: public_b,
        private: private,
        curve: "p192".into()
    };

    println!("key: {}", key);
}
