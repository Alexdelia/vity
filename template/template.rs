const CONST_YES: &str = r#"yes"#;
const NO: &str = "no";

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

	let v = vec![1, 2, 3];
	let x = 0u32;
	let y: u32 = 0;
	let c = 'c';
	let s = "string\n";
	let s = r#"raw string\n"#;
	let s = format!("fstring {x} {}", y);

	x = x + x;
	x = x - x;
	x = x * x;
	x = x / x;
	x += x;
	x -= x;
	x *= x;
	x /= x;
	x += 1;
	x -= 1;

	v[0] = 42;
	v.len();
	v[0..1];

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

	if let Some(x) = x {
		println!("{x}");
	} else {
		println!("no x");
	};

	while x < 10 {
		x += 1;
	}

	for x in 0..10 {
		let d = Some(x);
		println!("{d:?}");
	}

	loop {
		println!("loop");
		break;
	}

	let b: bool = true;
	let b = false;

	if ((x == x || x != x) && x == x && sizeof(x))
	while (((((true)))))
		x = false;

	foo(x, NULL, NULL);

	)
	// comment

    s.parse()
}
