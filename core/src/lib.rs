use std::{fmt, io::Write};

pub type Result<T, E = ()> = std::result::Result<T, E>;

pub trait IntoNixFmt {
    type Foo: NixFormat;
}

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
pub mod air {
    use crate::{NixFormat, NixFormatter, Result};
    use std::collections::BTreeMap;

    pub enum Value {
        String(String),
        AttributeSet(AttributeSet),
    }
    impl NixFormat for Value {
        fn nix_format<F: NixFormatter>(&self, f: &mut F) -> Result<()> {
            match self {
                Self::String(v) => v.nix_format(f),
                Self::AttributeSet(v) => v.nix_format(f),
            }
        }
    }
    impl From<&str> for Value {
        fn from(value: &str) -> Self {
            Self::from(value.to_string())
        }
    }
    impl From<String> for Value {
        fn from(value: String) -> Self {
            Self::String(value)
        }
    }
    impl From<AttributeSet> for Value {
        fn from(value: AttributeSet) -> Self {
            Self::AttributeSet(value)
        }
    }
    impl NixFormat for String {
        fn nix_format<F: NixFormatter>(&self, f: &mut F) -> Result<()> {
            f.write_value("\"")?;
            f.write_value(self)?;
            f.write_value("\"")
        }
    }
    pub struct AttributeSet(pub BTreeMap<String, Value>);
    impl NixFormat for AttributeSet {
        fn nix_format<F: NixFormatter>(&self, f: &mut F) -> Result<()> {
            f.write_line("{")?;
            for (k, v) in self.0.iter() {
                f.write_value("  ")?;
                k.nix_format(f)?;
                f.write_value(" = ")?;
                v.nix_format(f)?;
                f.write_value(";\n")?;
            }
            f.write_line("}")?;
            Ok(())
        }
    }
    #[test]
    fn attribute_set_format() {
        panic!("woo");

        let attr_set = AttributeSet({
            let mut b = BTreeMap::new();
            b.insert("foo".into(), "bar".into());
            b
        });
        assert_eq!(
            attr_set.to_string().unwrap(),
            r#"{
  "foo" = "bar";
}
"#
        );
        let attr_set = AttributeSet({
            let mut b = BTreeMap::<_, Value>::new();
            b.insert("bing".into(), "bang".into());
            b.insert("bang".into(), attr_set.into());
            b
        });
        println!("{}", attr_set.to_string().unwrap());
        assert_eq!(
            attr_set.to_string().unwrap(),
            r#"{
  "foo" = "bar";
}
"#
        );
    }
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
    // impl NixFormat for AttrSet {
    //     fn nix_format<W: std::io::Write>(&self, mut w: W, depth: u8) -> Result<(), ()> {
    //         write!(w, "foo").unwrap();
    //         Ok(())
    //     }
    // }
    #[test]
    fn attr_nix() {}
}
pub mod example_flake {
    use crate::flake::Input;
    #[derive(Debug)]
    pub struct Inputs {}
    pub struct Outputs {}
}

pub mod foo {
    use rnix::{
        ast::{AttrSet, Ident, List},
        parser::parse,
        tokenize, Root, SyntaxKind, SyntaxNode,
    };

    #[test]
    fn main() {
        let code = r#"{ hello = "world"; }"#;
        let tokens = tokenize(code);
        dbg!(&tokens);
        let (node, _) = parse(tokens.into_iter());
        dbg!(&node);
        // let root = ast.root();
        // // traverse_ast(root);

        use rowan::ast::AstNode;
        println!("{}", Root::parse(code).tree().syntax().text());
        // println!("{}", List.tree().syntax().text());
        // let i = Ident::from(SyntaxNode::new(Node::Ident("x".into()), 0));
        let (node, errs) = parse(
            vec![
                // foo
                (List::KIND, "foo"),
            ]
            .into_iter(),
        );
        dbg!(&node, &errs);
        println!("{node:#?}");

        let code = r#"{ hello }: hello"#;
        let tokens = tokenize(code);
        let (node, _) = parse(tokens.into_iter());
        dbg!(&node);

        // let code = r#"{ hello  }: hello"#;
        dbg!(nixpkgs_fmt::explain(code));
        dbg!(nixpkgs_fmt::reformat_string(code));

        panic!("woo");
    }
}
