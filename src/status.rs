use std::fmt::{Debug, Formatter, Error};

extern crate git2;

struct StatusFormatter<'repo> {
    status: &'repo git2::Status,
}

impl<'repo> StatusFormatter<'repo> {
    fn new(status: &'repo git2::Status) -> StatusFormatter {
        StatusFormatter { status: status }
    }
}

impl<'repo> Debug for StatusFormatter<'repo> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.status {
            &git2::STATUS_WT_MODIFIED => write!(f, "   modified"),
            &git2::STATUS_WT_NEW => write!(f, "        new"),
            &git2::STATUS_WT_DELETED => write!(f, "    deleted"),
            &git2::STATUS_WT_RENAMED => write!(f, "    renamed"),
            &git2::STATUS_WT_TYPECHANGE => write!(f, "type change"),
            _ => self.status.fmt(f),
        }
    }
}

// TODO: modified submodule
pub fn pprint(status: &git2::StatusEntry) {
    let st = status.status();
    if let Some(path) = status.path() {
        match st {
            git2::STATUS_IGNORED => {}
            _ => {
                println!("  {:?}: {}", StatusFormatter::new(&st), path);
            }
        }
    } else {
        fail!("# non UTF-8 file path");
    }
}

pub fn is_clean(statuses: &git2::Statuses) -> bool {
    statuses.iter()
        .map(|st| if st.status() == git2::STATUS_IGNORED {
            0
        } else {
            1
        })
        .sum::<i32>() == 0
}
