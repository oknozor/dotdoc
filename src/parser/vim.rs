use super::*;

use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "vimrc.pest"]
pub struct VimRcParser {
    content: String,
}

impl<'a> DotFileParser<'a> for VimRcParser {
    type PairItem = Pair<'a, Rule>;
    type Ast = Ast;

    fn new(path: &str) -> VimRcParser {
        VimRcParser {
            content: get_file_content(path).unwrap(),
        }
    }

    fn parse_file(&'a self) -> Self::PairItem {
        Self::parse(Rule::file, &self.content)
            .expect("Failed to parse vimrc")
            .next()
            .unwrap()
    }

    fn into_ast(&self) -> Ast {
        let file = self.parse_file();
        let mut ast = Ast::new();

        file.into_inner().for_each(|pair| {
            if let Rule::key_mapping = pair.as_rule() {
                ast.key_bindings.extend(parse_mappings(pair.into_inner()))
            } else {
            }
        });

        ast
    }
}

fn parse_mappings(pairs: Pairs<'_, Rule>) -> Vec<KeyBinding> {
    pairs
        .map(|pair| match pair.as_rule() {
            Rule::map => KeyBinding::new(BindingType::Map, parse_keys(pair.into_inner())),
            Rule::imap => KeyBinding::new(BindingType::Imap, parse_keys(pair.into_inner())),
            Rule::vmap => KeyBinding::new(BindingType::Vmap, parse_keys(pair.into_inner())),
            Rule::nmap => KeyBinding::new(BindingType::Nmap, parse_keys(pair.into_inner())),
            Rule::tmap => KeyBinding::new(BindingType::Tmap, parse_keys(pair.into_inner())),
            Rule::noremap => KeyBinding::new(BindingType::Noremap, parse_keys(pair.into_inner())),
            Rule::inoremap => KeyBinding::new(BindingType::INoremap, parse_keys(pair.into_inner())),
            Rule::vnoremap => KeyBinding::new(BindingType::VNoremap, parse_keys(pair.into_inner())),
            Rule::nnoremap => KeyBinding::new(BindingType::NNoremap, parse_keys(pair.into_inner())),
            Rule::tnoremap => KeyBinding::new(BindingType::TNoremap, parse_keys(pair.into_inner())),
            _ => unreachable!(),
        })
        .collect()
}

fn parse_keys(pairs: Pairs<'_, Rule>) -> Vec<Key> {
    pairs
        .map(|pair| match pair.as_rule() {
            Rule::special_key => match pair.as_str().to_lowercase().as_str() {
                "<scrollwheelup>" => Key::ScrollWheelUp,
                "<scrollwheeldown>" => Key::ScrollWheelDown,
                "<up>" => Key::Up,
                "<down>" => Key::Down,
                "<left>" => Key::Left,
                "<right>" => Key::Right,
                "<esc>" => Key::Esc,
                "<nop>" => Key::Nop,
                "<c>" => Key::Ctrl,
                "<cr>" => Key::Enter,
                custom => Key::Custom(custom.into()),
            },
            Rule::normal_key => Key::Key(pair.as_str().into()),
            Rule::command => Key::Command(pair.as_str().into()),
            Rule::plugin_call => Key::PluginCall(pair.as_str().into()),
            _ => unreachable!(),
        })
        .collect()
}

#[derive(Debug)]
pub struct Ast {
    pub(crate) key_bindings: Vec<KeyBinding>,
}

impl Ast {
    fn new() -> Self {
        Ast {
            key_bindings: vec![],
        }
    }
}

#[derive(Debug)]
pub(crate) struct KeyBinding {
    pub binding_type: BindingType,
    pub keys: Vec<Key>,
}

impl KeyBinding {
    fn new(binding_type: BindingType, keys: Vec<Key>) -> Self {
        KeyBinding { binding_type, keys }
    }
}

#[derive(Debug)]
pub(crate) enum Key {
    Command(String),
    PluginCall(String),
    Custom(String), // (name, value)
    Key(String),
    Nop,
    Up,
    Down,
    Left,
    Right,
    ScrollWheelUp,
    ScrollWheelDown,
    Ctrl,
    Alt,
    Shift,
    Enter,
    Esc,
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match self {
            Key::Command(a) => format!("Command : `{}`", a),
            Key::PluginCall(a) => format!("Vim plug command : `{}`", a),
            Key::Custom(a) => format!("Custom key: `{}`", a),
            Key::Key(a) => format!("`{}`", a),
            Key::Nop => "`nop`".into(),
            Key::Up => "`up`".into(),
            Key::Down => "`down`".into(),
            Key::Left => "`left`".into(),
            Key::Right => "`right`".into(),
            Key::ScrollWheelUp => "`ScrollWheelUp`".into(),
            Key::ScrollWheelDown => "`ScrollWheelDown`".into(),
            Key::Ctrl => "``crtl".into(),
            Key::Alt => "`alt`".into(),
            Key::Shift => "`shift`".into(),
            Key::Enter => "`enter`".into(),
            Key::Esc => "`esc`".into(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum BindingType {
    Map,
    Vmap,
    Imap,
    Nmap,
    Tmap,
    Noremap,
    VNoremap,
    INoremap,
    NNoremap,
    TNoremap,
}

impl ToString for BindingType {
    fn to_string(&self) -> String {
        match self {
            BindingType::Map => "map".into(),
            BindingType::Vmap => "vmap".into(),
            BindingType::Imap => "imap".into(),
            BindingType::Nmap => "imap".into(),
            BindingType::Tmap => "tmap".into(),
            BindingType::Noremap => "noremap".into(),
            BindingType::VNoremap => "vnoremap".into(),
            BindingType::INoremap => "inoremap".into(),
            BindingType::NNoremap => "nnoremap".into(),
            BindingType::TNoremap => "tnoremap".into(),
        }
    }
}

#[cfg(test)]
mod test {}
