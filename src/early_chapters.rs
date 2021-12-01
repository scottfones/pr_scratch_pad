#![allow(dead_code, unused_variables)]

use crate::Point;

use rand::prelude::*;
use tmdb::model::*;
use tmdb::themoviedb::*;

pub(crate) fn early_chapters_main() {
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x: 0, y: 0 } => println!("origin is (0, 0)"),
        Point { x, y: 0 } if x != 0 => println!("x offset is {}", x),
        Point { x: 0, y } if y != 0 => println!("y offset is {}", y),
        Point { x, y } => println!("x offset is {}, y offset is {}", x, y),
    }

    let Point { x: pa, y: pb } = origin;

    let msg = {
        if pa == 0 {
            "zero"
        } else {
            "two"
        }
    };
    println!("{}", msg);

    let ta = { 4 };

    let s = "Hello, \
        momma";
    println!("{}", &s);

    let mut t = "Deno is the best!".to_string();
    println!("{:?}", t.pop());

    for c in t.chars() {
        println!("{:?}", c);
    }

    let u = t.chars().next();
    println!("{:?}", u);

    let v = fib_rec(1, 1, 10);
    println!("{:?}", v);

    let w = fib_iter(1, 1, 10);
    println!("{:?}", w);

    let mut x = rand_vec(20);
    println!("\nQuicksort:\n{:?}", x);
    quicksort(&mut x);
    println!("{:?}", x);

    let sum_tot = summn(3);
    println!("{:?}", sum_tot);

    println!("{:?}", bubblesort(&mut rand_vec(20)));

    findshow();
}

fn fib_rec(x0: i128, x1: i128, len: i32) -> Vec<i128> {
    if len == 2 {
        return vec![x1, x0 + x1];
    }
    let mut v0 = vec![x1];
    v0.append(&mut fib_rec(x1, x0 + x1, len - 1));
    v0
}

fn fib_iter(x0: i128, x1: i128, len: i32) -> Vec<i128> {
    let mut f = vec![x0, x1];
    for i in 2..len {
        f.push(f[(i - 1) as usize] + f[(i - 2) as usize]);
    }
    f
}

/// Generate random vec of length n
fn rand_vec(n: i32) -> Vec<i128> {
    let mut rng = thread_rng();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(rng.gen_range(-10..=10));
    }
    v
}

/// Bubblesort vec
fn bubblesort(v: &mut Vec<i128>) -> &mut Vec<i128> {
    for i in 0..v.len() {
        let mut is_swap = false;

        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                is_swap = true;
            }
        }
        if !is_swap {
            if i < v.len() - 1 {
                println!("Bubblesort: breaking early, {:?}", i);
            }
            break;
        }
    }
    v
}

// Quicksort vec
fn quicksort(v: &mut Vec<i128>) -> &mut Vec<i128> {
    if v.len() <= 1 {
        return v;
    }
    let pivot = v.remove(0);
    let mut left = Vec::new();
    let mut right = Vec::new();
    v.iter().for_each(|vi| {
        if *vi < pivot {
            left.push(*vi);
        } else {
            right.push(*vi);
        }
    });
    v.clear();
    v.append(&mut quicksort(&mut left));
    v.push(pivot);
    v.append(&mut quicksort(&mut right));
    v
}

fn summn(n: usize) -> usize {
    (1..n + 1).sum()
}

fn findshow() {
    let tmdb = TMDb {
        api_key: "01a09891363e68e9dfcfa846cf1e1031",
        language: "en",
    };

    let movies = tmdb
        .search()
        .title("Ghostbusters")
        .year(1984)
        .execute()
        .unwrap();

    let id = movies.results[0].id;

    let interstellar: Movie = tmdb.fetch().id(id).execute().unwrap();

    println!("{:#?}", interstellar);
}
