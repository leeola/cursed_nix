use std::io::Write;

pub trait Format {
    fn nix_format<W: Write>(&self, writer: W, depth: u8) -> Result<(), ()>;
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
pub trait Fn {
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

pub mod example_flake {
    use crate::flake::Input;
    #[derive(Debug)]
    pub struct Inputs {}
    pub struct Outputs {}
}

pub fn add(left: usize, right: usize) -> usize {
    left + right + 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
