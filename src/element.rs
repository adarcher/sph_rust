// Basic KDElement
// To subclass see 'Deref' impl in this file to help fake polymorphism
// TODO: convert to Trait
//  - required members: position
//  - formatting
//  - iteration functions
//  - ...

use std::ops::{Deref, DerefMut};

use crate::Vector3D;

pub trait KDElementT: Clone + Copy + Default
where
  Self: Deref<Target = Vector3D>,
  Self: DerefMut<Target = Vector3D>,
{
  // TODO: Add proper iterators
}

#[derive(Clone, Copy, Default)]
pub struct KDElement {
  pub position: Vector3D,
}

impl Deref for KDElement {
  type Target = Vector3D;

  fn deref(&self) -> &Self::Target {
    &self.position
  }
}

impl DerefMut for KDElement {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.position
  }
}

impl KDElementT for KDElement {}

impl std::fmt::Display for KDElement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "E{}", &self.position)
  }
}
