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
  fn inspect(branch: &Branch<'_, Root>) -> Option<Self>;
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

pub(crate) struct CommitableBranch<'p, R: BranchRoot> {
  branch: Branch<'p, R>,
  committed: bool,
}
impl<'p, R: BranchRoot> Deref for CommitableBranch<'p, R> {
  type Target = Branch<'p, R>;

  fn deref(&self) -> &Self::Target {
    &self.branch
  }
}
impl<'p, R: BranchRoot> CommitableBranch<'p, R> {
  pub fn commit(mut self) -> Result<(), R::CommitError> {
    let parent_data: &R::BranchData = self.parent_data;
    parent_data.update_from(&self.data);

    self.committed = true;
    Ok(())
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
  pub(crate) fn child(&self) -> CommitableBranch<'_, R> {
    CommitableBranch {
      branch: Branch::new(self.root, self.parent_data),
      committed: false,
    }
  }

  pub fn scoped<Out, F>(self: &'p Branch<'p, R>, f: F) -> Option<Out>
  where
    F: FnOnce(&Branch<'_, R>) -> Option<Out>,
  {
    let branch = self.child();
    let val = f(&branch)?;
    match branch.commit() {
      Ok(_) => Some(val),
      Err(_err) => None,
    }
  }

  pub fn inspect<Inspect: BranchInspect<R>>(self: &'p Branch<'p, R>) -> Option<Inspect> {
    self.scoped(Inspect::inspect)
  }
}
