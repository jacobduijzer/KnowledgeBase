pub mod converter {
    use std::arch::x86_64::__cpuid;
    use std::borrow::{Borrow, BorrowMut};
    use std::fs;
    use std::thread::current;

    #[derive(Debug, Default)]
    pub struct Quote {
        pub chapter: String,
        pub quote: Vec<String>,
        pub notes: Vec<String>
    }

    impl Quote {
        pub fn add_quote(&mut self, line: &str) {
            self.quote.push(line.to_string());
        }

        pub fn add_note(&mut self, line: &str) {
            self.notes.push(line.to_string());
        }
    }

    #[derive(Debug, Default)]
    pub struct Book {
        pub title: String,
        pub author: String,
        pub quotes: Vec<Quote>
    }

    impl Book {
        pub fn new(title: &str, author: &str) -> Book {
            Book {
                title: title.to_string(),
                author: author.to_string(),
                quotes: Vec::new()
            }
        }

        pub fn from_file(filepath: &str) -> Book {
            let f = fs::read_to_string(filepath);
            let input = f.expect("could not open input file");
            Self::from(&input)
        }

        fn from(input: &str) -> Book {
            let (title, author) = input
                .lines()
                .next()
                .unwrap()
                .split_once(" by ").unwrap();

            let first_chapter_position = input
                .lines()
                .position(|line| line.starts_with("Chapter "))
                .unwrap();

            let mut book = Book::new(title, author);

            let groups = input
                .lines()
                .skip(first_chapter_position)
                .filter(|line| !line.is_empty() && !line.ends_with("Highlight") && !line.starts_with("Chapter progress"))
                .fold(Vec::new(), |mut acc, x| {
                    if x.starts_with("Chapter ") {
                        acc.push((Vec::new()));
                    }
                    acc.last_mut().unwrap().push(x.trim());
                    acc
                });

            groups
                .into_iter()
                .for_each(|group| {
                    let title = group[0];
                    if group.contains(&"Annotation") {
                        let mut doing_notes = false;
                        let mut highlights: Vec<String> = Vec::new();
                        let mut notes: Vec<String> = Vec::new();
                        for line in group.into_iter().skip(2) {

                            if line.starts_with("Notes:") {
                                doing_notes = true;
                            }

                            if !doing_notes {
                                highlights.push(line.replace("Highlight:", "").trim().to_string());
                            } else {
                                notes.push(line.replace("Notes: ", "").trim().to_string());
                            }
                        }

                        book.quotes.push(Quote {
                            chapter: title.to_string(),
                            quote: highlights,
                            notes
                        })

                    } else {
                        let quotes: Vec<String> = group.into_iter().skip(1).map(|x| x.replace("Highlight: ", "").trim().to_string()).collect::<Vec<_>>();
                        book.quotes.push(Quote {
                            chapter: title.to_string(),
                            quote: quotes,
                            notes: Vec::default()
                        })
                    }
                });

            book
        }

        pub fn export_to_post(&self) -> String {
            let mut post = format!("---
title: \"{}\"
date: 2022-12-27T12:00:00+01:00
draft: true
categories: [\"books\"]
tags: [\"tag1\"]
---

Intro

<!--more-->

Author(s): {}
", self.title, self.author);

            let mut chapter_title = String::default();

            for quote in &self.quotes {
                if quote.chapter != chapter_title.to_string() {
                    chapter_title = quote.chapter.to_string();
                    post.push_str(format!("## {}\n\n", quote.chapter).as_str());
                }

                for highlight in &quote.quote {
                    post.push_str(format!("> {}\n\n", highlight).as_str());
                }

                for note in &quote.notes {
                    post.push_str(format!("{}\n\n", note).as_str());
                }
            }

            post
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{env, fs};

    const test_file: &str = "tests/input.txt";

    #[test]
    fn parse_title_and_author_from_file() {
        let book = converter::Book::from_file(test_file);
        assert_eq!(book.title, "Extreme Ownership: How U.S. Navy SEALs Lead and Win");
        assert_eq!(book.author, "Jocko Willink & Leif Babin");
    }

    #[test]
    fn parse_chapters_from_input() {
        let book = converter::Book::from_file(test_file);
        assert_eq!(book.quotes.len(), 167);
        assert_eq!(book.quotes[0].chapter, "Chapter 4: Preface");
        assert_eq!(book.quotes[0].quote.len(), 1);
        assert_eq!(book.quotes[0].notes.len(), 0);
        assert_eq!(book.quotes[19].chapter, "Chapter 6: Introduction");
        assert_eq!(book.quotes[19].quote.len(), 1);
        assert_eq!(book.quotes[19].quote[0], "developed");
        assert_eq!(book.quotes[19].notes.len(), 1);
        assert_eq!(book.quotes[19].notes[0], "Leadership program to make it scalable");
    }

    #[test]
    fn parse_and_export_annotations() {
        let book = converter::Book::from_file(test_file);
        let export = book.export_to_post();
        println!("{export:?}");
    }
}