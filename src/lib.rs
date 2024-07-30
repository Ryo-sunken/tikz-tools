use std::fs::File;

enum DrawInst {
    Circle,
    Line,
}

impl DrawInst {
    fn new_circle() {}

    fn new_line() {}
}

pub struct TikzPainter {
    insts: Vec<DrawInst>,
}

impl TikzPainter {}

pub struct TikzPlotter {}

impl TikzPlotter {}

#[cfg(test)]
mod tests {
    use super::*;
}
