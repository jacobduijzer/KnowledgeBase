pub mod converter {
    use std::fs;

    #[derive(Debug)]
    pub struct Chapter {
        pub title: String,
        pub quotes: Vec<String>
    }

    impl Chapter {
        pub fn new() -> Chapter {
            Chapter {
                title: "".to_string(),
                quotes: Vec::new()
            }
        }

        pub fn from(title: &str) -> Chapter {
            Chapter {
                title: title.to_string(),
                quotes: Vec::new()
            }
        }

        pub fn add_quote(&mut self, quote: &str) {
            self.quotes.push(quote.to_string());
        }
    }

    #[derive(Debug)]
    pub struct Book {
        pub title: String,
        pub author: String,
        pub chapters: Vec<Chapter>
    }

    impl Book {
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

            let mut chapters: Vec<Chapter> = Vec::new();

            let mut chapter: Chapter = Chapter::new();

            for line in input.lines().into_iter() {
                if line.starts_with("Chapter") && !line.starts_with("Chapter progress") && line != chapter.title {
                    // push chapter quotes
                    if chapter.title != "" {
                       chapters.push(chapter);
                    }
                    // set new chapter
                    chapter = Chapter::from(line);
                } else if line.starts_with("Highlight:") {
                    let quote = line.replace("Highlight: ", "");
                    chapter.add_quote(&quote);
                }
            }

            chapters.push(chapter);

            Book {
                title: title.to_string(),
                author: author.to_string(),
                chapters
            }
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

            for chapter in &self.chapters {
                post.push_str(format!("## {}\n\n", chapter.title).as_str());
                for quote in &chapter.quotes {
                    post.push_str(format!("> {}\n\n", quote).as_str());
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
        assert_eq!(book.chapters.len(), 11);

        let chapter_counts: Vec<(String, usize)> = book.chapters
            .into_iter()
            .map(|chapter| (chapter.title, chapter.quotes.len()))
            .collect();

        assert_eq!(chapter_counts.len(), 11);
        assert_eq!(chapter_counts[0].0, "Chapter 4: Preface");
        assert_eq!(chapter_counts[0].1, 7);
        assert_eq!(chapter_counts[chapter_counts.len()-1].0, "Chapter 27: 9. Plan");
        assert_eq!(chapter_counts[chapter_counts.len()-1].1, 16);
    }

    #[test]
    fn parse_and_export_annotations() {
        let book = converter::Book::from_file(test_file);
        let export = book.export_to_post();
        println!("{export:?}");
    }
}