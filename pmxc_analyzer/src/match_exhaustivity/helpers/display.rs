use crate::match_exhaustivity::*;
use std::fmt::{self, Display, Formatter};

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Pattern::Discard { .. } => write!(f, "_"),
            Pattern::Constructor { name, args } if args.is_empty() => write!(f, "{}", name),
            Pattern::Constructor { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}
