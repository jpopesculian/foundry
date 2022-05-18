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

impl DestructuredComment {
    pub fn is_line(&self) -> bool {
        matches!(self.ty, CommentType::Line)
    }
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
        self.prefixes.first().and_then(
            |comment| {
                if comment.loc.end() < byte {
                    Some(comment)
                } else {
                    None
                }
            },
        )
    }

    pub(crate) fn pop_postfix(&mut self, byte: usize) -> Option<DestructuredComment> {
        if self.postfixes.first()?.loc.end() < byte {
            Some(self.postfixes.remove(0))
        } else {
            None
        }
    }

    pub(crate) fn remove_comments_between(
        &mut self,
        range: impl std::ops::RangeBounds<usize>,
    ) -> Vec<DestructuredComment> {
        let mut prefixes = {
            let (cleared, remaining) = std::mem::take(&mut self.prefixes)
                .into_iter()
                .partition(|comment| range.contains(&comment.loc.start()));
            self.prefixes = remaining;
            cleared
        };
        let mut postfixes = {
            let (cleared, remaining) = std::mem::take(&mut self.postfixes)
                .into_iter()
                .partition(|comment| range.contains(&comment.loc.start()));
            self.postfixes = remaining;
            cleared
        };
        prefixes.append(&mut postfixes);
        prefixes
    }
}
