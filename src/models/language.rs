use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum Language {
    JAVA,
    TYPESCRIPT,
    C,
    CPP,
    RUST,
}

impl Ord for Language {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Language::JAVA => {
                match other {
                    Language::JAVA => Ordering::Equal,
                    Language::TYPESCRIPT => Ordering::Greater,
                    Language::C => Ordering::Less,
                    Language::CPP => Ordering::Less,
                    Language::RUST => Ordering::Greater
                }
            },
            Language::TYPESCRIPT => {
                match other {
                    Language::JAVA => Ordering::Less,
                    Language::TYPESCRIPT => Ordering::Equal,
                    Language::C => Ordering::Less,
                    Language::CPP => Ordering::Less,
                    Language::RUST => Ordering::Less
                }
            },
            Language::C => {
                match other {
                    Language::JAVA => Ordering::Less,
                    Language::TYPESCRIPT => Ordering::Less,
                    Language::C => Ordering::Equal,
                    Language::CPP => Ordering::Less,
                    Language::RUST => Ordering::Less
                }
            },
            Language::CPP => {
                match other {
                    Language::JAVA => Ordering::Less,
                    Language::TYPESCRIPT => Ordering::Less,
                    Language::C => Ordering::Greater,
                    Language::CPP => Ordering::Equal,
                    Language::RUST => Ordering::Less
                }
            },
            Language::RUST => {
                match other {
                    Language::JAVA => Ordering::Greater,
                    Language::TYPESCRIPT => Ordering::Less,
                    Language::C => Ordering::Greater,
                    Language::CPP => Ordering::Greater,
                    Language::RUST => Ordering::Equal
                }
            }
        }
    }
}