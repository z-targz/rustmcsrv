pub struct Octree {//octree representing a 16^3 area
    node: OctreeNode,
    depth: u8,
}

pub enum OctreeNode {
    Material,
    Octree(Box<OctreeNode>),
}

pub struct Chunk; //24 octrees vertically