use solang_parser::pt::*;

#[derive(Debug)]
pub enum CommentType {
    Line,
    Block,
}

#[derive(Debug)]
pub struct DestructuredComment {
    pub ty: CommentType,
    pub loc: Loc,
    pub comment: String,
}

impl From<Comment> for DestructuredComment {
    fn from(comment: Comment) -> Self {
        match comment {
            Comment::Line(loc, comment) => Self { ty: CommentType::Line, loc, comment },
            Comment::Block(loc, comment) => Self { ty: CommentType::Block, loc, comment },
        }
    }
}

#[derive(Debug)]
pub struct Comments {
    prefixes: Vec<DestructuredComment>,
    postfixes: Vec<DestructuredComment>,
}

impl Comments {
    pub fn new(mut comments: Vec<Comment>, src: &str) -> Self {
        let mut prefixes = Vec::new();
        let mut postfixes = Vec::new();

        for comment in comments {
            let comment = comment.into();
            if Self::is_newline_comment(&comment, src) {
                // TODO check if there are blank lines after the prefixes which may need to be
                // incorporated
                prefixes.push(comment)
            } else {
                postfixes.push(comment)
            }
        }
        Self { prefixes, postfixes }
    }

    fn is_newline_comment(comment: &DestructuredComment, src: &str) -> bool {
        for ch in src[..comment.loc.start()].chars().rev() {
            if ch == '\n' {
                return true
            } else if !ch.is_whitespace() {
                return false
            }
        }
        true
    }

    pub(crate) fn pop_prefix(&mut self, byte: usize) -> Option<DestructuredComment> {
        if self.prefixes.first()?.loc.end() < byte {
            Some(self.prefixes.remove(0))
        } else {
            None
        }
    }

    pub(crate) fn peek_prefix(&mut self, byte: usize) -> Option<&DestructuredComment> {
        if self.prefixes.first()?.loc.end() < byte {
            self.prefixes.get(0)
        } else {
            None
        }
    }

    pub(crate) fn pop_postfix(&mut self, byte: usize) -> Option<DestructuredComment> {
        if self.postfixes.first()?.loc.end() < byte {
            Some(self.postfixes.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn comments() {
        let src = std::fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("testdata")
                .join("SimpleComments")
                .join("original.sol"),
        )
        .unwrap();
        let (mut pt, comments) = solang_parser::parse(&src, 1).unwrap();
        let coms = Comments::new(comments, &src);
        println!("{:?}", coms);
        unimplemented!()
    }
}
