// #![feature(vec_from_fn)]
// #![feature(sort_floats)]

use std::{
  env,
  sync::LazyLock,
  time::{Duration, Instant},
};

use crate::{element::KDElement, kdtree::KDTree, vector3d::Vector3D};

pub mod element;
pub mod kdtree;
pub mod vector3d;

static TIME_0: LazyLock<Instant> = LazyLock::new(Instant::now);

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  let num: usize = args.get(0).and_then(|a| a.parse().ok()).unwrap_or(100000);
  let lim: usize = args.get(1).and_then(|a| a.parse().ok()).unwrap_or(1000);
  let dim: f32 = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(10.0);

  println!("Hello, world!  Isn't it a wonderful day?");
  println!(
    "Tree example with element (count, limit, size): ({},{},{})",
    num, lim, dim
  );

  let size = Vector3D::new(dim, dim, dim);

  //   let TIME_0 = Instant::now();
  let mut tree = KDTree::<KDElement>::new(size, size / 2.0, lim, num);

  let t_allocate = timecheck(Duration::ZERO, "Allocate Tree", true);

  tree.randomize(size, 0);

  let t_random = timecheck(t_allocate, "Randomize Data", true);

  tree.distribute();

  let _t_distribute = timecheck(t_random, "Distribute/Sort into Tree", true);

  let _radius = 0.5;
  let leaves = tree.leaf_indices();
  let node_random_index = leaves[leaves.len() - 2];
  let node_random = &tree.nodes[node_random_index];
  if !node_random.is_leaf() {
    println!("Not a leaf: {}", node_random_index)
  } else {
    println!(
      "Leaf({}) has {} elements",
      node_random_index,
      node_random.range.1 - node_random.range.0
    );
  }
  //   let element = &tree.elements[node_random.range.0];
  //   let indices = tree.gather_nearest_node_indices(node_random_index, radius);

  //   let _t_find = timecheck(&TIME_0, t_distribute, "Find Nearest", true);

  //   println!("\tindices near [{}] {:?}", node_random_index, indices);
  //   let elements = &tree.elements_from_node_indices(&indices);

  let t_1 = TIME_0.elapsed();

  tree.iterate_all(count_neighbors);
  timecheck(t_1, "Iterate All", true);
  //   tree.iterate(node_random_index, count_neighbors);

  //   count_neighbors(element, elements, radius);

  //   count_neighbors(element, &vec![&tree.elements], radius);

  //   let _t_all = timecheck(&TIME_0, t_count, "Count All Elements", true);
}

// static TIME_P: LazyLock<Duration> = LazyLock::new(TIME_0::elapsed);
fn timecheck(previous: Duration, label: &str, print: bool) -> Duration {
  let elapsed = TIME_0.elapsed();
  let dt = elapsed - previous;
  if print {
    println!("t{} d{} :: {}", fmt_time(elapsed), fmt_time(dt), label);
  }
  elapsed
}

fn fmt_time(duration: Duration) -> String {
  format!(
    "({:02}:{:02}:{:06})",
    duration.as_secs() / 60,
    duration.as_secs(),
    duration.subsec_micros()
  )
}

fn count_neighbors(element: &KDElement, elements: &Vec<&[KDElement]>, radius: f32) {
  //   let t0 = TIME_0.elapsed();
  let mut near = 0;
  let mut far = 0;
  let radius2 = radius * 2.0;
  let mut total = 0;
  elements.iter().for_each(|e| {
    e.iter().for_each(|e2| {
      let distance = element.position.dist(e2.position);
      total += 1;
      if distance <= radius {
        near += 1;
      } else if distance > radius2 {
        far += 1;
      }
    });
  });
}
