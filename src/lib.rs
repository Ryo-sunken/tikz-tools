use std::cmp::PartialEq;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq)]
struct LineData {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct NodeData {
    x: f64,
    y: f64,
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
struct EdgeData {
    from: String,
    to: String,
}

#[derive(Clone, Debug, PartialEq)]
enum DrawInst {
    Node(NodeData),
    Edge(EdgeData),
}

impl DrawInst {
    fn node(x: f64, y: f64, name: String) -> Self {
        Self::Node(NodeData { x, y, name })
    }

    fn edge(from: String, to: String) -> Self {
        Self::Edge(EdgeData { from, to })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TikzPainter {
    aspect_ratio: f64,
    insts: Vec<DrawInst>,
}

impl TikzPainter {
    pub fn new(aspect_ratio: f64) -> Self {
        Self {
            aspect_ratio,
            insts: Vec::new(),
        }
    }

    pub fn draw_node(&mut self, x: f64, y: f64, name: String) {
        assert!(x <= 1.);
        assert!(y <= self.aspect_ratio);

        self.insts.push(DrawInst::node(x, y, name));
    }

    pub fn draw_edge(&mut self, from: String, to: String) {
        self.insts.push(DrawInst::edge(from, to));
    }

    pub fn save(&self, file: &mut File) {
        let mut insts_str = Vec::new();
        for inst in &self.insts {
            match inst {
                DrawInst::Node(NodeData { x, y, name }) => {
                    insts_str.push(format!(r"\node[draw, circle] ({}) at ({}\hsize, {}\hsize) {{{}}};", name, x, -y, name));
                }
                DrawInst::Edge(EdgeData { from, to }) => {
                    insts_str.push(format!(r"\draw ({}) to ({});", from, to));
                }
            }
        }
        write!(file, "{}", insts_str.join("\n")).unwrap();
    }
}

pub struct TikzPlotter {}

impl TikzPlotter {}

#[cfg(test)]
mod tests {
    use crate::DrawInst::*;
    use crate::*;

    #[test]
    fn save_test() {
        let mut painter = TikzPainter::new(1.);
        painter.draw_node(0., 0., "1".to_string());
        painter.draw_node(0., 0.1, "2".to_string());
        painter.draw_edge("1".to_string(), "2".to_string());
        painter.save(&mut File::create("test.txt").unwrap());
    }
}
