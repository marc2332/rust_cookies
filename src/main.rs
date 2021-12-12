#![forbid(non_ascii_idents)]
use std::{
    fmt::{self, Display},
    ops::Add,
};


// COOKIES
#[derive(Debug, Clone)]
struct Cookie;

impl Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "just got 1 🍪 :/ ")
    }
}

impl Add<Self> for Cookie {
    type Output = Cookies;

    fn add(self, other: Self) -> Cookies {
        Cookies(vec![self, other])
    }
}


// COOKIES
#[derive(Debug, Clone)]
struct Cookies(Vec<Cookie>);

impl Add<Cookie> for Cookies {
    type Output = Cookies;

    fn add(self, other: Cookie) -> Cookies {
        let mut cookies = self.clone();
        cookies.0.push(other);
        cookies
    }
}


impl Display for Cookies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "yum yum!  {}",
            self.0.iter().map(|_| "🍪 ").collect::<String>()
        )
    }
}

fn main() {
    println!("{}", Cookie + Cookie);
    println!("{}", Cookie);
    println!("{}", Cookie + Cookie + Cookie + Cookie);
}
