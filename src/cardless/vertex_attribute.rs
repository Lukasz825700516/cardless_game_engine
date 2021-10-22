pub enum VertexAttributeType {
    F32,
    I32,
    U32,
}

pub struct VertexAttribute {
    pub attribute_type: VertexAttributeType,
    pub size: usize,
    pub normalized: bool,
    pub stride: usize,
    pub width: usize,
}

impl VertexAttribute {
    pub fn new(attribute_type: VertexAttributeType, size: usize, normalized: bool, stride: usize, width: usize) -> Self {
        Self { attribute_type, size, normalized, stride, width }
    }
}

pub trait Vertex: Sized {
    fn get_attributes_layout() -> Vec<VertexAttribute>;
}
