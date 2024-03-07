use std::fmt::{Display, Formatter};

///This is the main thing handling positions in files.

//TODO: listen to: Wunderbar 1940

#[derive(Copy, Clone, Eq, Debug)]
pub struct Position {
    /// The Line that this is on.
    pub line: usize,
    /// Where the slice starts
    pub chr_start: usize,
    /// Length of the slice
    pub length: usize
}

impl Position {
    pub fn new(line: usize, start: usize, length: usize) -> Position {
        Position {
            line,
            chr_start: start,
            length
        }
    }

    /// finds this in a string and returns Some(&str) if it found it.
    pub fn find<'a>(&self, source: &'a str) -> Option<&'a str> {
        let mut lines = source.lines();
        let line = lines.nth(self.line)?;
        if line.len() < self.length + self.chr_start { return None }

        Some(
            &line[self.chr_start..self.length + self.chr_start]
        )
    }

    /// Finds the string that's before the string defined by the position.
    pub fn find_before<'a>(&self, source: &'a str) -> Option<&'a str> {
        let mut lines = source.lines();
        let line = lines.nth(self.line)?;
        if line.len() < self.chr_start { return None }

        Some(
            &line[..self.chr_start]
        )
    }

    /// finds the string that's after the position.
    pub fn find_after<'a>(&self, source: &'a str) -> Option<&'a str> {
        let mut lines = source.lines();
        let line = lines.nth(self.line)?;
        if line.len() < self.length+self.chr_start { return None }

        Some(
            &line[self.length+self.chr_start..line.len()]
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.chr_start)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.chr_start == other.chr_start && self.length == other.length
    }
}

#[cfg(test)]
mod tests {
    use crate::helium::position::Position;

    #[test]
    fn find() {
        let src = "a\nabcdefg\nhalo\n";
        let pos = Position::new(1, 2, 4);
        assert_eq!(pos.find(src), Some("cdef"));
    }
}