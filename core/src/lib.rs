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

pub trait NixLike: Into<Nir> {
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
impl<T> NixLike for T where T: Into<Nir> {}

pub mod nir {
    use crate::{NixLike, Result};
    use std::{
        collections::BTreeMap,
        fmt,
        ops::{Deref, DerefMut},
    };

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
    impl From<BTreeMap<String, Nir>> for Nir {
        fn from(value: BTreeMap<String, Nir>) -> Self {
            Self::AttributeSet(value.into())
        }
    }
    impl NixFormat for String {
        fn nix_format<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
            Ok(w.write_fmt(format_args!("\"{self}\""))?)
        }
    }
    #[derive(Debug, Clone)]
    pub struct AttributeSet(pub BTreeMap<String, Nir>);
    impl AttributeSet {
        pub fn new() -> Self {
            Self(Default::default())
        }
    }
    impl NixFormat for AttributeSet {
        fn nix_format<W: fmt::Write>(&self, w: &mut W) -> Result<()> {
            w.write_char('{')?;
            // NIT: Would be nice to base this on char len, or something.
            let new_line = self.0.len() >= 2;
            for (k, v) in self.0.iter() {
                k.nix_format(w)?;
                w.write_char('=')?;
                v.nix_format(w)?;
                w.write_char(';')?;
                if new_line {
                    w.write_char('\n')?;
                }
            }
            w.write_char('}')?;
            Ok(())
        }
    }
    impl Deref for AttributeSet {
        type Target = BTreeMap<String, Nir>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for AttributeSet {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl From<BTreeMap<String, Nir>> for AttributeSet {
        fn from(value: BTreeMap<String, Nir>) -> Self {
            Self(value)
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
            let kvs = ('a'..'z')
                .into_iter()
                .enumerate()
                .map(|(i, k)| {
                    let v = format!("{k} {i}");
                    (String::from(k), v)
                })
                .collect::<Vec<_>>();
            for (k, v) in kvs {
                b.insert(k, v.into());
            }
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
pub trait AttributeSetLike: NixLike {
    type ArgSet: ArgSetLike;
    fn keys() -> &'static [&'static str];
}
// NIT: This name is wrong.. i forget what nix calls this.
pub trait ArgSetLike: NixLike {
    type AttributeSet: AttributeSetLike;
    fn keys() -> &'static [&'static str];
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
    use crate::nir::{AttributeSet, Nir};

    #[derive(Debug, Clone)]
    pub struct Flake<Inputs> {
        pub inputs: Inputs,
        // pub outputs: Fn<_,_>, NixFn..?,
    }
    #[derive(Debug, Clone)]
    pub struct Input {
        pub url: String,
    }
    impl From<Input> for Nir {
        fn from(value: Input) -> Self {
            let mut b = AttributeSet::new();
            b.insert("url".into(), value.url.into());
            b.into()
        }
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
