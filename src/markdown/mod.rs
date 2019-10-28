mod vimrc;

pub trait ToMarkDown {
    fn to_markdown(&self) -> String;
}
