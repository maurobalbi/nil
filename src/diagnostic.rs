use std::fmt;
use syntax::{ErrorKind as SynErrorKind, TextRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diagnostic {
    pub range: TextRange,
    pub kind: DiagnosticKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    // Syntax.
    SyntaxError(SynErrorKind),

    // Lowering.
    InvalidDynamic,
    DuplicatedKey,
    EmptyInherit,
    EmptyLetIn,
    LetAttrset,
    UriLiteral,
    MergePlainRecAttrset,
    MergeRecAttrset,

    // Liveness.
    UnusedBinding,
    UnusedWith,
    UnusedRec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    IncompleteSyntax,
}

impl Diagnostic {
    pub fn severity(&self) -> Severity {
        match self.kind {
            DiagnosticKind::InvalidDynamic | DiagnosticKind::DuplicatedKey => Severity::Error,
            DiagnosticKind::EmptyInherit
            | DiagnosticKind::EmptyLetIn
            | DiagnosticKind::LetAttrset
            | DiagnosticKind::UriLiteral
            | DiagnosticKind::MergePlainRecAttrset
            | DiagnosticKind::MergeRecAttrset
            | DiagnosticKind::UnusedBinding
            | DiagnosticKind::UnusedWith
            | DiagnosticKind::UnusedRec => Severity::Warning,
            DiagnosticKind::SyntaxError(kind) => match kind {
                SynErrorKind::MultipleRoots
                | SynErrorKind::PathTrailingSlash
                | SynErrorKind::PathDuplicatedSlashes
                | SynErrorKind::MultipleNoAssoc => Severity::Error,
                SynErrorKind::UnexpectedToken
                | SynErrorKind::MissingToken(_)
                | SynErrorKind::MissingExpr
                | SynErrorKind::MissingAttr => Severity::IncompleteSyntax,
            },
        }
    }

    pub fn message(&self) -> String {
        match self.kind {
            DiagnosticKind::SyntaxError(kind) => return kind.to_string(),

            DiagnosticKind::InvalidDynamic => "Invalid location of dynamic attribute",
            DiagnosticKind::DuplicatedKey => "Duplicated name definition",
            DiagnosticKind::EmptyInherit => "Nothing inherited",
            DiagnosticKind::EmptyLetIn => "Empty let-in",
            DiagnosticKind::LetAttrset => {
                "`let { ... }` is deprecated. Use `let ... in ...` instead"
            }
            DiagnosticKind::UriLiteral => {
                "URL literal is confusing and deprecated. Use strings instead"
            }
            DiagnosticKind::MergePlainRecAttrset => {
                "Merging non-rec-attrset with rec-attrset, the latter `rec` is implicitly ignored"
            }
            DiagnosticKind::MergeRecAttrset => {
                "Merging rec-attrset with other attrsets or attrpath. Merged values can unexpectedly reference each other remotely as in a single `rec { ... }`."
            }

            DiagnosticKind::UnusedBinding => "Unused binding",
            DiagnosticKind::UnusedWith => "Unused `with`",
            DiagnosticKind::UnusedRec => "Unused `rec`",
        }
        .into()
    }

    pub fn is_unnecessary(&self) -> bool {
        matches!(
            self.kind,
            DiagnosticKind::EmptyInherit
                | DiagnosticKind::UnusedBinding
                | DiagnosticKind::UnusedWith
                | DiagnosticKind::UnusedRec
        )
    }

    pub fn is_deprecated(&self) -> bool {
        matches!(
            self.kind,
            DiagnosticKind::LetAttrset | DiagnosticKind::UriLiteral
        )
    }
}

impl From<syntax::Error> for Diagnostic {
    fn from(err: syntax::Error) -> Self {
        Self {
            range: err.range,
            kind: DiagnosticKind::SyntaxError(err.kind),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {}..{}",
            self.message(),
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )
    }
}
