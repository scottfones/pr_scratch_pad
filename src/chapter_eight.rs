//! Scratchpad for Programming Rust, Chapter Eight
#![allow(dead_code, unused_variables)]

pub(crate) fn chapter_eight_main() {
    println!("Chapter 8");

    println!("Sum of [1,5]: {}", sum_to_n(5));
}

/// Given a u32 [`n`], return the sum from 1 to [`n`]
fn sum_to_n(n: u32) -> u32 {
    (1..=n).sum()
}

#[test]
fn test_sum_to_n() {
    assert_eq!(sum_to_n(1), 1);
    assert_eq!(sum_to_n(2), 3);
    assert_eq!(sum_to_n(3), 6);
    assert_eq!(sum_to_n(5), 15);
    assert_eq!(sum_to_n(7), 28);
    assert_eq!(sum_to_n(10), 55);
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_sum_to_n_panic() {
    println!("{}", sum_to_n(u32::pow(2, 31)));
}
