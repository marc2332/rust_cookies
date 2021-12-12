#![forbid(non_ascii_idents)]
use std::{
    fmt::{self, Display},
    ops::{Add, Mul},
};


// COOKIE
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

impl Mul<i32> for Cookie {
    type Output = Cookies;

    fn mul(self, how_many: i32) -> Cookies {
        Cookies((0..how_many).map(|_| Cookie).collect::<Vec<Cookie>>())
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

impl Mul<i32> for Cookies {
    type Output = Cookies;

    fn mul(mut self, how_many: i32) -> Cookies {
        let current_cookies = self.0.clone();
        (0..how_many - 1).for_each(|_| self.0.extend(current_cookies.clone()));
        self
    }
}


impl Display for Cookies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            n if n >= 40 => write!(f, "go see your doctor."),
            n if n >= 10 => write!(f, "{} 🍪 ?? TOO MUCH.", self.0.len()),
            _ => write!(
                f,
                "yum yum!  {}",
                self.0.iter().map(|_| "🍪 ").collect::<String>()
            )
        }
        
    }
}

fn main() {
    println!("{}", Cookie + Cookie);
    println!("{}", Cookie);
    println!("{}", Cookie + Cookie + Cookie + Cookie);
    println!("{}", Cookie * 10);
    println!("{}", (Cookie + Cookie + Cookie) * 5);
    println!("{}", Cookie * 60);
}
