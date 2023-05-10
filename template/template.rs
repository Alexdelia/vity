#![allow(dead_code)]

const CONST_YES: &str = r#"yes"#;
const NO: &str = "no";

#[derive(Debug)]
pub enum Bar {
    Yes(u8),
    No,
}

struct BarStruct<T> {
    yes: u8,
    no: T,
}

trait BarTrait {
    fn yes(&self) -> u8;
    fn no(&self) -> u8;
}

impl BarTrait for BarStruct<u8> {
    fn yes(&self) -> u8 {
        self.yes
    }

    fn no(&self) -> u8 {
        self.no
    }
}

// comment

/*
multi line comment
*/

/// doc comment
///
/// # Example
///
/// ```
/// let x = 1;
/// ```
fn foo(s: impl Into<String>, b: &[u8]) -> Result<Bar, ()> {
    let mut s = s.into();

    let mut v = vec![1, 2, 3];
    let mut x = 0u32;
    let y: u32 = 0;
    let c = 'c';
    let s = "string\n\x1b[1;35m";
    let s = r#"raw string\n"#;
    let s = format!("fstring {x} {}", y);

    let d = 10;
    let b = 0b10;
    let o = 0o10;
    let h = 0x10;

    x = x + x;
    x = x - x;
    x = x * x;
    x = x / x;
    x = x % x;
    x += x;
    x -= x;
    x *= x;
    x /= x;
    x %= x;
    x += 1;
    x -= 1;

    v[0] = 42;
    v.len();

    let raw_ptr = &x as *const u32;
    let addr = raw_ptr as usize;

    x = if x == y {
        1
    } else if x != y {
        2
    } else {
        3
    };

    x = match x {
        1 => 1,
        2 => 2,
        _ => 3,
    };

    if let Some(x) = Some(1) {
        println!("{x}");
    } else {
        println!("no x");
    };

    while x < 10 {
        x += 1;
    }

    for x in 0..=42 {
        let d = Some(x);
        println!("{d:?}");
    }

    loop {
        println!("loop");
        break;
    }

    let b: bool = true;
    let b = false;

    foo("str", &[1, 2, 3]);

    let bar = BarStruct::<u8> { yes: 1, no: 2 };

    if bar.yes() == 1 {
        Ok(Bar::Yes(1))
    } else {
        Err(())
    }
}
