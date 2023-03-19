use std::{fmt, io::Write};

pub type Result<T, E = ()> = std::result::Result<T, E>;

// TODO: write two traits to automatically manage nesting.
// Ie A -> B -> A -> B, where a user of these traits cannot
// pass the formatter into a child without converting it, and thereby
// forcing it to become a nested formatter, which then gets converted
// back to being usable.
pub trait NixFormatter {
    fn write_value(&mut self, s: &str) -> Result<()>;
    fn write_line(&mut self, s: &str) -> Result<()>;
}
pub struct NixWriter<W: Write>(W);
impl<W: Write> NixWriter<W> {
    pub fn new(w: W) -> Self {
        Self(w)
    }
    pub fn into_inner(self) -> W {
        self.0
    }
}
impl<W> NixFormatter for NixWriter<W>
where
    W: Write,
{
    fn write_value(&mut self, s: &str) -> Result<()> {
        let Self(w) = self;
        write!(w, "{}", s).map_err(|_| ())
    }
    fn write_line(&mut self, s: &str) -> Result<()> {
        let Self(w) = self;
        write!(w, "{}\n", s).map_err(|_| ())
    }
}
pub trait NixFormat {
    fn nix_format<F: NixFormatter>(&self, f: &mut F) -> Result<()>;
    fn to_string(&self) -> Result<String> {
        let mut w = NixWriter::new(Vec::new());
        self.nix_format(&mut w)?;
        let buf = w.into_inner();
        let s = String::from_utf8(buf).unwrap();
        Ok(s)
    }
}
pub mod ast {
    use crate::{NixFormat, NixFormatter, Result};
    use std::collections::BTreeMap;

    pub enum Value {
        String(String),
        // AttributeSet(AttributeSet)
    }
    impl NixFormat for Value {
        fn nix_format<F: NixFormatter>(&self, f: F) -> Result<()> {
            match self {
                Self::String(s) => f.write_value(s),
            }
        }
    }
    impl NixFormat for String {
        fn nix_format<F: NixFormatter>(&self, f: F) -> Result<()> {
            f.write_value(self)
        }
    }
    pub struct AttributeSet(pub BTreeMap<String, Value>);
}
pub trait Value {}
pub trait AttributeSet {
    type Value: Value;
    fn keys(&self) -> Vec<&str>;
    fn iter(&self) -> Box<dyn Iterator<Item = (&str, &Self::Value)>>;
}
// pub struct Fn<Input, Output> {
//     pub input: Input,
//     pub output: Output,
// }
pub trait FnLike {
    type Arg;
    type Return;
}
pub struct Variadic<T>(pub T);

pub mod flake {
    #[derive(Debug)]
    pub struct Input {}
    pub struct Flake<Inputs> {
        pub inputs: Inputs,
        // pub outputs: Fn<_,_>, NixFn..?,
    }
}

pub mod example_types {
    use crate::NixFormat;

    #[derive(Debug)]
    pub struct AttrSet {
        foo: String,
    }
    impl NixFormat for AttrSet {
        fn nix_format<W: std::io::Write>(&self, mut w: W, depth: u8) -> Result<(), ()> {
            write!(w, "foo").unwrap();
            Ok(())
        }
    }
    #[test]
    fn attr_nix() {}
}
pub mod example_flake {
    use crate::flake::Input;
    #[derive(Debug)]
    pub struct Inputs {}
    pub struct Outputs {}
}
