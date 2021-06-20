#[derive(Debug)]
pub(crate) struct Row
{
    pub(crate) kind: RowKind,
    pub(crate) start: String,
    pub(crate) event: String,
    pub(crate) target: String,
    pub(crate) action: String,
}

impl Row
{
    pub(crate) fn new(kind: RowKind) -> Self
    {
        Self{kind, start: "".into(), event: "".into(), target: "".into(), action: "".into()}
    }
}

#[derive(Debug)]
pub(crate) enum RowKind
{
    WithGuard,
    Other
}
