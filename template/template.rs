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

impl FromStr for Bar {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(Template::Yes(0)),
            "no" => Ok(Template::No),
            _ => Err(()),
        }
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

	let x = 0u32;
	let y: u32 = 0;

	x = x + x;
	x = x - x;
	x = x * x;
	x = x / x;
	x += x;
	x -= x;
	x *= x;
	x /= x;
	x++; ++x;
	x--; --x;
	y++; ++y;
	y--; --y;

	av[0] = NULL;
	av->hey;
	av.hey;
	y->begin();
	y->hey;
	(*(*y)).begin();

	x = &sl;
	x = *sl;

	if ((x == x || x != x) && x == x && sizeof(x))
	while (((((true)))))
		x = false;

	foo(x, NULL, NULL);

	)
	// comment

    s.parse()
}
