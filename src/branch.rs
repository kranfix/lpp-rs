use std::ops::Deref;

pub trait BranchRoot: Sized {
  type BranchData: BranchData;
  type CommitError;

  fn data(&self) -> &Self::BranchData;

  fn branch(&self) -> crate::branch::Branch<'_, Self> {
    let data = self.data();
    crate::branch::Branch::new(self, data)
  }
}

pub trait BranchInspect<Root: BranchRoot>: Sized {
  fn inspect(branch: &mut Branch<'_, Root>) -> Option<Self>;
}

pub trait BranchData {
  fn child_data(&self) -> Self;
  fn update_from(&self, other: &Self);
}

#[derive(Debug)]
pub struct Branch<'p, R: BranchRoot> {
  root: &'p R,
  parent_data: &'p R::BranchData,
  data: R::BranchData,
}
impl<'p, R: 'p + BranchRoot> Deref for Branch<'p, R> {
  type Target = R::BranchData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<'p, R: BranchRoot> Branch<'p, R> {
  pub fn new(root: &'p R, parent_data: &'p R::BranchData) -> Self {
    let data = parent_data.child_data();
    Branch {
      root,
      parent_data,
      data,
    }
  }
  pub fn root(&self) -> &'p R {
    &self.root
  }

  pub fn scoped<Out, F>(&mut self, f: F) -> Option<Out>
  where
    F: FnOnce(&mut Branch<'_, R>) -> Option<Out>,
  {
    let mut branch = self.child();
    let val = f(&mut branch)?;
    branch.commit(val)
  }

  pub fn inspect<Inspect: BranchInspect<R>>(&mut self) -> Option<Inspect> {
    self.scoped(Inspect::inspect)
  }

  fn child(&self) -> Branch<'_, R> {
    Branch::new(self.root, self.parent_data)
  }
  fn commit<T>(self, val: T) -> Option<T> {
    let parent_data: &R::BranchData = self.parent_data;
    parent_data.update_from(&self.data);
    Some(val)
  }
}
