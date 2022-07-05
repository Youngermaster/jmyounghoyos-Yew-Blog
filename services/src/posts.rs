use std::*;

#[derive(Clone)]
pub struct PostFile {
    pub content: &'static str,
    pub modified_time: u128,
    pub filename: &'static str,
}

pub static POSTS: [PostFile; 2] = [
    PostFile {
        content: include_str!("../../posts/HackTheBox/late.md"),
        modified_time: 1656959820000,
        filename: "late",
    },
    PostFile {
        content: include_str!("../../posts/HackTheBox/unicode.md"),
        modified_time: 1656959820000,
        filename: "unicode",
    },
];
