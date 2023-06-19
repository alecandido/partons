use std::fmt::{self, Display};

#[derive(Clone)]
pub(crate) enum Data {
    Index,
    Info(String),
    Set(String),
    Member(String, u32),
}

impl Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Index => write!(f, "Index"),
            Self::Info(set) => write!(f, "Info: {set}"),
            Self::Set(set) => write!(f, "Set: {set}"),
            Self::Member(set, num) => write!(f, "Grid: {set}-{num}"),
        }
    }
}

#[derive(Clone)]
pub(crate) enum State {
    Regular,
    Original,
}

impl State {
    pub(crate) fn marker(&self) -> String {
        match self {
            Self::Regular => "",
            Self::Original => "original",
        }
        .to_owned()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let marker = self.marker();
        write!(f, "{marker}")
    }
}

pub(crate) struct Resource {
    pub(crate) data: Data,
    pub(crate) state: State,
}

impl Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.clone();
        let state = self.state.clone();
        write!(f, "{state} {data}")
    }
}
