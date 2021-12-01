//! Scratchpad for Programming Rust, Chapter 9
#![allow(dead_code, unused_variables)]

/// Video resolution
#[derive(Debug)]
struct Resolution(u32, u32);

/// Movie metadata
#[derive(Debug)]
struct Movie {
    date: String,
    description: String,
    resolution: Resolution,
    title: String,
}

pub(crate) fn chapter_nine_main() {
    println!("\nChapter Nine");

    let m_gb = Movie {
        date: "June 1, 1984".to_string(),
        description: "We ain't afraid of no ghosts".to_string(),
        resolution: Resolution(3840, 2160),
        title: "Ghostbusters".to_string()
    };

    println!("{:#?}", m_gb);

    let m_gb2 = Movie {
        date: "June 16, 1989".to_string(),
        title: "Ghostbusters II".to_string(),
        .. m_gb
    };

    println!("{:#?}", m_gb2);
}
