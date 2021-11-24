fn gcd(a: u64, b: u64) -> u64 {
	if a < b {
		gcd(b, a)
	} else {
		if b == 0 {
			a
        } else {
			gcd(a - b, b)
        }    
	} 	
}

use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip( 1 ) {
        numbers.push( u64::from_str( &arg ).expect("arg not a number"));
    }
    
    assert!( numbers.len() > 0, "no args passed" );
    
    let mut d = numbers[0];
    for n in &numbers {
        d = gcd( d, *n);
    }

    println!( "Numbers: {:?} gcd: {}", numbers, d );
}

#[test] // This is a funciton attribute
fn test_gcd() {
    assert_eq!( gcd(12, 9), 3 );
    assert_eq!( gcd(14, 15), 1) ;
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                        3 * 7 * 11 * 13 * 19),
                   3 * 11);
}

/*
 * `assert!` and `debug_assert!` - the later is
 * excluded from compilation.
 */
