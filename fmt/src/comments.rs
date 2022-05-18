use crate::solang_ext::*;
use solang_parser::pt::*;

#[derive(Debug, Clone, Copy)]
pub enum CommentType {
    Line,
    Block,
}

#[derive(Debug, Clone, Copy)]
pub enum CommentPosition {
    Prefix,
    Postfix,
}

#[derive(Debug, Clone)]
pub struct DestructuredComment {
    pub ty: CommentType,
    pub loc: Loc,
    pub comment: String,
    pub position: CommentPosition,
}

impl DestructuredComment {
    fn new(comment: Comment, position: CommentPosition) -> Self {
        let (ty, loc, comment) = match comment {
            Comment::Line(loc, comment) => (CommentType::Line, loc, comment),
            Comment::Block(loc, comment) => (CommentType::Block, loc, comment),
        };
        Self { ty, loc, comment, position }
    }
    pub fn is_line(&self) -> bool {
        matches!(self.ty, CommentType::Line)
    }
    pub fn is_prefix(&self) -> bool {
        matches!(self.position, CommentPosition::Prefix)
    }
}

#[derive(Debug, Clone)]
pub struct Comments {
    prefixes: Vec<DestructuredComment>,
    postfixes: Vec<DestructuredComment>,
}

impl Comments {
    pub fn new(comments: Vec<Comment>, src: &str) -> Self {
        let mut prefixes = Vec::new();
        let mut postfixes = Vec::new();

        for comment in comments {
            if Self::is_newline_comment(&comment, src) {
                prefixes.push(DestructuredComment::new(comment, CommentPosition::Prefix))
            } else {
                postfixes.push(DestructuredComment::new(comment, CommentPosition::Postfix))
            }
        }
        Self { prefixes, postfixes }
    }

    fn is_newline_comment(comment: &Comment, src: &str) -> bool {
        for ch in src[..comment.loc().start()].chars().rev() {
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

    pub(crate) fn drain(&mut self) -> Vec<DestructuredComment> {
        let mut out = std::mem::take(&mut self.prefixes);
        out.append(&mut self.postfixes);
        out.sort_by_key(|comment| comment.loc.start());
        out
    }
}
