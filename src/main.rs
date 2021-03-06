fn gcd(a: u64, b: u64) -> u64 {
	if a < b {
		gcd(b, a)
	} else {
        assert!( a >= b );
		if b == 0 {
			a
        } else {
			gcd(a - b, b)
        }    
	} 	
}

fn gcd_seq( seq: &[u64] ) -> u64 {
    let mut d = seq[0];
    for i in &seq[1..] {
        d = gcd( d, *i );
    }
    d
}

use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip( 1 ) {
        numbers.push( u64::from_str( &arg ).expect("arg not a number"));
    }
    assert!( numbers.len() > 0, "no args passed" );
    
    let d = gcd_seq( &numbers );
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
