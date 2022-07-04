use std::*;

#[derive(Clone)]
pub struct PostFile {
    pub content: &'static str,
    pub modified_time: u128,
    pub filename: &'static str,
}

pub static POSTS: [PostFile; 4] = [
    PostFile {
        content: include_str!("../../posts/build_blog.md"),
        modified_time: 1656959820000,
        filename: "build_blog",
    },
    PostFile {
        content: include_str!("../../posts/add_links.md"),
        modified_time: 1656959820000,
        filename: "add_links",
    },
    PostFile {
        content: include_str!("../../posts/project_test.md"),
        modified_time: 1656959820000,
        filename: "project_test",
    },
    PostFile {
        content: include_str!("../../posts/HackTheBox/late.md"),
        modified_time: 1656959820000,
        filename: "late",
    },
];
