// Base/Default acceleration structure
// KD-Tree
//  - cycled/alternating binary mid-point splits (ie: x then y then z)
//  - set depth for fast and distributed-friendly storage and lookup

use rand::{self, Rng, SeedableRng, rngs::SmallRng};

use crate::Vector3D;
use crate::element::KDElementT;

#[derive()]
pub struct KDTree<E: KDElementT> {
  pub size: Vector3D,
  pub location: Vector3D,
  pub depth: usize,
  pub split_max: usize,
  pub population_max: usize,
  pub nodes: Vec<KDNode>,
  pub elements: Vec<E>,
}

impl<E: KDElementT> KDTree<E> {
  pub fn new(size: Vector3D, location: Vector3D, split_max: usize, population_max: usize) -> Self {
    let depth = KDTree::<E>::depth_from_node_index(population_max / split_max) + 1;
    let mut tree = KDTree {
      size,
      location,
      depth,
      split_max,
      population_max,
      nodes: vec![KDNode::default(); 2_usize.pow(depth as u32)],
      elements: vec![E::default(); population_max],
    };
    tree.fill();
    tree
  }

  // TODO: use 'fill' traits and include type arguments
  pub fn fill(&mut self) {
    self.nodes.fill(KDNode::default());
    // TODO: fill_with_fn somewhere?
    let max = self.nodes.len();
    for (i, node) in self.nodes.iter_mut().enumerate() {
      let child = (i + 1) * 2;
      node.right = if child < max { child } else { 0 };
      node.left = if child < max { child - 1 } else { 0 };
      node.size = self.size;
    }
  }

  // TODO: move to debug, this isn't super useful otherwise.
  //  And, if it is useful, it should be part of whatever library uses it.
  pub fn randomize(&mut self, _size: Vector3D, seed: u64) {
    let size = self.size;
    let root = &self.nodes[0];
    let mean = (self.size.x - root.position.x) / 2.0 + root.position.x;
    let div = 0.3 * size.x;
    let mut rng = SmallRng::seed_from_u64(seed);
    for element in &mut self.elements {
      element.x = (rng.random_range(root.position.x..(root.position.x + size.x)) - mean) / div;
      element.y = (rng.random_range(root.position.y..(root.position.y + size.y)) - mean) / div;
      element.z = (rng.random_range(root.position.z..(root.position.z + size.z)) - mean) / div;
    }
  }

  // Note: misnomer, as it doesn't distribute, but set the limits for each branch/leaf
  //  The tree holds all elements in a vector and the nodes only reference the range within
  //  that vector--use left/right indices.
  pub fn distribute(&mut self) {
    let mut working_stack = vec![0]; // Start with root with index 0

    let root = &mut self.nodes[0];
    root.range = (0, self.elements.len());
    root.sort(&mut self.elements, 0);

    while let Some(index) = working_stack.pop() {
      let current = &mut self.nodes[index];
      if KDNode::element_count(&current) <= self.split_max {
        continue;
      }

      // TODO: A lot of this is for Rust's memory safety rules; can definitely be cleaned up
      let depth = KDTree::<E>::depth_from_node_index(index);
      let direction = depth % 3; // x/y/z ordered-split
      let size = current.size;
      let position = current.position;

      let range_start = current.range.0;
      let range_end = current.range.1;
      // TODO: better flag for leaf needed... this just uses a negative range for branches
      current.range = (range_end, range_start);
      let range_mid = (range_start + range_end) / 2;
      let child_left_range = (range_start, range_mid);
      let child_right_range = (range_mid, range_end);

      let child_left_index = current.left;
      let child_right_index = current.right;

      let div1 = self.elements[child_left_range.1 - 1][direction];
      let div2 = self.elements[child_left_range.1][direction];
      let division = (div1 + div2) / 2.;
      let left_size = division - current.position[direction];
      let right_size = current.size[direction] - left_size;

      let left = &mut self.nodes[child_left_index];
      left.range = child_left_range;
      left.size = size;
      left.size[direction] = left_size;
      left.position = position;
      left.sort(&mut self.elements, depth + 1);
      working_stack.push(child_left_index);

      let right = &mut self.nodes[child_right_index];
      right.range = child_right_range;
      right.size = size;
      right.size[direction] = right_size;
      right.position = position;
      right.position[direction] = division;
      right.sort(&mut self.elements, depth + 1);
      working_stack.push(child_right_index);
    }
  }

  pub fn iterate_all(&self, cb: fn(&E, &Vec<&[E]>, f32)) {
    self.leaf_indices().iter().for_each(|i| {
      self.iterate_node(*i, cb);
    });
  }

  pub fn iterate_node(&self, node_index: usize, cb: fn(&E, &Vec<&[E]>, f32)) {
    let radius = 0.5;
    let node = &self.nodes[node_index];
    let elements = &self.elements[node.range.0..node.range.1];
    let neighbors = &self.gather_nearest_elements(node_index, radius);
    elements.iter().for_each(|e| cb(e, neighbors, radius));
  }

  pub fn leaf_nodes(&self) -> Vec<&KDNode> {
    let mut leaf_nodes = vec![];
    self.nodes.iter().for_each(|node| {
      if node.is_leaf() {
        leaf_nodes.push(node);
      }
    });
    leaf_nodes
  }

  pub fn leaf_indices(&self) -> Vec<usize> {
    let mut leaf_node_indices = vec![];
    self.nodes.iter().enumerate().for_each(|(i, node)| {
      if node.is_leaf() {
        leaf_node_indices.push(i);
      }
    });
    leaf_node_indices
  }

  pub fn elements_from_node_indices(&self, element_indices: &[usize]) -> Vec<&[E]> {
    let mut elements = vec![];
    element_indices.iter().for_each(|i| {
      let node = &self.nodes[*i];
      elements.push(&self.elements[node.range.0..node.range.1]);
    });
    elements
  }

  pub fn depth_from_node_index(index: usize) -> usize {
    // TODO: Really need to check this math and document why it's ALWAYS correct
    64 - (index + 1).leading_zeros() as usize
  }

  pub fn find_leaf(&self, query: Vector3D) -> &KDNode {
    &self.nodes[self.find_index(0, query)]
  }

  pub fn gather_nearest_node_indices(&self, node_index: usize, radius: f32) -> Vec<usize> {
    let node = &self.nodes[node_index];
    let (min, max) = (node.position - radius, node.position + node.size + radius);
    let mut node_indices = vec![];
    let mut index_stack = vec![0];

    while let Some(index) = index_stack.pop() {
      let current = &self.nodes[index];
      if current.position < max && current.position + current.size > min {
        if current.is_leaf() {
          node_indices.push(index);
        } else {
          index_stack.push(current.left);
          index_stack.push(current.right);
        }
      }
    }
    node_indices
  }

  pub fn gather_nearest_ranges(&self, node_index: usize, radius: f32) -> Vec<(usize, usize)> {
    let nearest_nodes = self.gather_nearest_node_indices(node_index, radius);
    let mut element_ranges = vec![];

    nearest_nodes
      .iter()
      .for_each(|i| element_ranges.push(self.nodes[*i].range));

    element_ranges
  }

  // TODO: Currently returns vector or slices to theoretically help caching (and was quicker to code)
  pub fn gather_nearest_elements(&self, node_index: usize, radius: f32) -> Vec<&[E]> {
    let ranges = self.gather_nearest_node_indices(node_index, radius);
    let mut element_groups = vec![];

    ranges.iter().for_each(|i| {
      let range = &self.nodes[*i].range;
      element_groups.push(&self.elements[range.0..range.1])
    });

    element_groups
  }

  // TODO: replay with non-recursive
  fn find_index(&self, index: usize, query: Vector3D) -> usize {
    let mut found = 0;

    let node = &self.nodes[index];
    if node.contains(query) {
      if !node.is_leaf() {
        found = self.find_index(node.right, query);
        // + self.find_index(node.left, query);
        // Does removing a branch help?
        if found == 0 {
          found = self.find_index(node.left, query);
        }
      } else {
        found = index;
      }
    }
    found
  }
}

#[derive(Default, Clone)]
pub struct KDNode {
  pub position: Vector3D,
  pub size: Vector3D,
  pub left: usize,
  pub right: usize,
  pub range: (usize, usize),
}

impl KDNode {
  pub fn new(
    position: Vector3D,
    size: Vector3D,
    left: usize,
    right: usize,
    range: (usize, usize),
  ) -> Self {
    Self {
      position,
      size,
      left,
      right,
      range,
    }
  }

  pub fn root(size: Vector3D) -> Self {
    Self {
      position: Vector3D::zero(),
      size,
      left: 0,
      right: 0,
      range: (0, 0),
    }
  }

  pub fn element_count(node: &KDNode) -> usize {
    node.range.1.abs_diff(node.range.0)
  }

  // Flip range when children as assigned ranges
  pub fn is_leaf(&self) -> bool {
    self.range.0 < self.range.1
  }

  pub fn contains(&self, vector: Vector3D) -> bool {
    self.position <= vector && (self.position + self.size) > vector
  }

  // TODO: compare sorting choices, index versus member versus "other"
  pub fn sort<E: KDElementT>(&self, elements: &mut [E], depth: usize) {
    let min = self.range.0;
    let max = self.range.1;
    match depth % 3 {
      0 => elements[min..max].sort_unstable_by(|a, b| a.x.total_cmp(&b.x)),
      1 => elements[min..max].sort_unstable_by(|a, b| a.y.total_cmp(&b.y)),
      2 => elements[min..max].sort_unstable_by(|a, b| a.z.total_cmp(&b.z)),
      _ => unreachable!(),
    }
  }

  // TODO: move to debug
  pub fn print(&self, level: usize) {
    self.print_child(level, "".to_string());
  }

  // TODO: move to debug
  pub fn print_child(&self, level: usize, prefix: String) {
    let levels: String = if level > 0 {
      "| ".repeat(level - 1)
    } else {
      "".to_string()
    };
    if self.is_leaf() {
      println!(
        "{levels}{prefix}{}{} : ({},{})",
        "node",
        level.to_string(),
        self.range.0,
        self.range.1,
      );
    } else {
      println!(
        "{levels}{prefix}{}{} : ({},{})",
        "node",
        level.to_string(),
        self.range.1,
        self.range.0,
      );
    }
    if self.left > 0 {
      // self.tree
      //     .node(self.left)
      //     .print_child(level + 1, "|-".to_string());
    }
    if self.right > 0 {
      // self.tree
      //     .node(self.right)
      //     .print_child(level + 1, "\\-".to_string());
    }
  }
}
