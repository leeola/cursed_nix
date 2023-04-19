use nir::{Nir, NixFormat};
use std::{fmt, io::Write};

pub type Result<T, E = NoError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct NoError(pub ());
impl From<std::fmt::Error> for NoError {
    fn from(_: std::fmt::Error) -> Self {
        Self(())
    }
}

pub trait NixExt: Into<Nir> {
    fn nix_to_string(self) -> Result<String> {
        let nir = self.into();
        let mut s = String::new();
        nir.nix_format(&mut s)?;
        // Super hacky, but convenient prototyping.
        Ok(nixpkgs_fmt::reformat_string(&s))
    }
    fn cloned_nix_to_string(&self) -> Result<String>
    where
        Self: Clone,
    {
        self.clone().nix_to_string()
    }
}
impl<T> NixExt for T where T: Into<Nir> {}

pub mod nir {
    use crate::{NixExt, Result};
    use std::{collections::BTreeMap, fmt};

    /// A centralized trait for internal writing of nix syntax to a std writer.
    ///
    /// Note that in the future this may change to be some sort of rnix Ast writer,
    /// but for now i'm just pushing strings because rnix ast creation is awkward. Tokens
    /// are easy, but a bit gross to deal with directly.
    ///
    /// See also [`Nir`] for rationale.
    pub trait NixFormat {
        fn nix_format<W: fmt::Write>(&self, f: &mut W) -> Result<()>;
    }

    /// An intermediate representation of a printable nix expression, value, etc. May not be a 1:1
    /// to the actual Nix primitives.
    ///
    /// A simplified representation of the rnix AST because i was having trouble finding
    /// ergonomic ways to create the tree.  
    #[derive(Debug, Clone)]
    pub enum Nir {
        String(String),
        AttributeSet(AttributeSet),
    }
    impl NixFormat for Nir {
        fn nix_format<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
            match self {
                Self::String(v) => v.nix_format(w),
                Self::AttributeSet(v) => v.nix_format(w),
            }
        }
    }
    impl From<&str> for Nir {
        fn from(value: &str) -> Self {
            Self::from(value.to_string())
        }
    }
    impl From<String> for Nir {
        fn from(value: String) -> Self {
            Self::String(value)
        }
    }
    impl From<AttributeSet> for Nir {
        fn from(value: AttributeSet) -> Self {
            Self::AttributeSet(value)
        }
    }
    impl NixFormat for String {
        fn nix_format<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
            Ok(w.write_fmt(format_args!("\"{self}\""))?)
        }
    }
    #[derive(Debug, Clone)]
    pub struct AttributeSet(pub BTreeMap<String, Nir>);
    impl NixFormat for AttributeSet {
        fn nix_format<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
            w.write_char('{')?;
            for (k, v) in self.0.iter() {
                k.nix_format(w)?;
                w.write_char('=')?;
                v.nix_format(w)?;
                w.write_char(';')?;
            }
            w.write_char('}')?;
            Ok(())
        }
    }
    #[test]
    fn attribute_set_format() {
        let attr_set = AttributeSet({
            let mut b = BTreeMap::new();
            b.insert("foo".into(), "bar".into());
            b
        });
        assert_eq!(
            attr_set.clone().nix_to_string().unwrap(),
            r#"{ "foo" = "bar"; }
"#
        );
        let attr_set = AttributeSet({
            let mut b = BTreeMap::<_, Nir>::new();
            b.insert("bing".into(), "bang".into());
            b.insert("bang".into(), attr_set.clone().into());
            b
        });
        println!("{}", attr_set.clone().nix_to_string().unwrap());
        assert_eq!(
            attr_set.nix_to_string().unwrap(),
            r#"{ "foo" = "bar";
}"#
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
