/// Location object
#[derive(Debug, Clone)]
pub struct Location
{
    filename: String,
    line: usize,
    column: usize
}

impl std::fmt::Display for Location
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}, line {}, column {}", self.filename, self.line, self.column)
    }
}

impl Location
{
    /// Create a Location object from a filename, and line, column positions
    pub fn construct(filename: &str, line: usize, column: usize) -> Self
    {
        Location
        {
            filename: String::from(filename),
            line, column
        }
    }
}