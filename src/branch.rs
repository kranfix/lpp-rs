use core::{fmt::Debug, ops::Deref};

pub trait BranchRoot: Sized {
  type BranchData: BranchData;
  type CommitError;

  fn data(&self) -> &Self::BranchData;

  fn branch(&self) -> crate::branch::Branch<'_, Self> {
    let data = self.data();
    crate::branch::Branch::new(self, data)
  }
}

pub trait Inspect<Root: BranchRoot, Output = Self>
where
  Output: Sized,
{
  fn inspect(branch: &mut Branch<'_, Root>) -> Option<Output>;
}

pub trait InspectFrom<Root: BranchRoot> {
  type Output;

  fn inspect_from(branch: &mut Branch<'_, Root>, value: Self) -> Option<Self::Output>;
}

pub trait BranchData: Debug {
  fn child_data(&self) -> Self;
  fn update_from(&self, other: Self);
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
    Branch {
      root,
      data: parent_data.child_data(),
      parent_data,
    }
  }
  pub fn root(&self) -> &'p R {
    &self.root
  }

  pub fn inspect<T: Inspect<R>>(&mut self) -> Option<T> {
    let mut branch = self.child();
    let val = T::inspect(&mut branch)?;
    branch.commit(val)
  }
  pub fn inspect_for<T: InspectFrom<R>>(&mut self, value: T) -> Option<T::Output> {
    let mut branch = self.child();
    let val = T::inspect_from(&mut branch, value)?;
    branch.commit(val)
  }
  pub fn scoped<Out, F>(&mut self, f: F) -> Option<Out>
  where
    F: FnOnce(&mut Branch<'_, R>) -> Option<Out>,
  {
    let mut branch = self.child();
    let val = f(&mut branch)?;
    branch.commit(val)
  }

  fn child(&self) -> Branch<'_, R> {
    Branch::new(self.root, &self.data)
  }
  fn commit<T>(self, val: T) -> Option<T> {
    let Branch {
      parent_data, data, ..
    } = self;
    parent_data.update_from(data);
    Some(val)
  }
}

impl<R: BranchRoot, Output, F> InspectFrom<R> for F
where
  F: FnOnce(&mut Branch<'_, R>) -> Option<Output>,
{
  type Output = Output;

  fn inspect_from(branch: &mut Branch<'_, R>, f: Self) -> Option<Self::Output> {
    f(branch)
  }
}
