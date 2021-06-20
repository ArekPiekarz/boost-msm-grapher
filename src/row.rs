#[derive(Debug)]
pub(crate) struct Row
{
    pub(crate) start: String,
    pub(crate) event: String,
    pub(crate) target: String,
}

impl Row
{
    pub(crate) fn new() -> Self
    {
        Self{start: "".into(), event: "".into(), target: "".into()}
    }
}
