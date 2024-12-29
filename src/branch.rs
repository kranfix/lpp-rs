use dupe::Dupe;
use std::{cell::Cell, ops::Deref};

pub trait Branchable: Sized {
  type BranchData: Dupe;
  type CommitError;

  fn branch<'r>(&'r self) -> Branch<'r, 'r, Self>;
  fn commit_branch<'r, 'p>(branch: &mut Branch<'r, 'p, Self>) -> Result<(), Self::CommitError>;
  fn on_drop_branch<'r, 'p>(branch: &mut Branch<'r, 'p, Self>);

  fn value_idx(&self) -> usize;
}

#[derive(Debug)]
pub struct Branch<'r, 'p, R: Branchable> {
  root: &'r R,
  parent: Option<&'p Branch<'r, 'p, R>>,
  data: R::BranchData,
  value_idx: Cell<usize>,
  committed: bool,
}
impl<'r, 'p, R: 'r + Branchable> Deref for Branch<'r, 'p, R> {
  type Target = R::BranchData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

pub(crate) struct CommitableBranch<'r, 'p, R: Branchable> {
  branch: Branch<'r, 'p, R>,
}
impl<'r, 'p, R: Branchable> Deref for CommitableBranch<'r, 'p, R> {
  type Target = Branch<'r, 'p, R>;

  fn deref(&self) -> &Self::Target {
    &self.branch
  }
}
impl<'r, 'p, R: Branchable> CommitableBranch<'r, 'p, R> {
  pub fn commit(mut self) -> Result<(), R::CommitError> {
    Branchable::commit_branch(&mut self.branch)?;
    self.branch.committed = true;
    Ok(())
  }
}

impl<'r, 'p, R: Branchable> Branch<'r, 'p, R> {
  pub fn new(root: &'r R, data: R::BranchData) -> Self {
    Branch {
      root,
      parent: None,
      data,
      committed: false,
      value_idx: Cell::new(root.value_idx()),
    }
  }
  pub fn root(&self) -> &'r R {
    &self.root
  }
  pub fn parent(&self) -> Option<&'p Self> {
    self.parent
  }
  pub(crate) fn child<'b>(&'b self) -> CommitableBranch<'r, 'b, R> {
    CommitableBranch {
      branch: Branch {
        root: &self.root,
        parent: Some(self),
        data: self.data.dupe(),
        committed: false,
        value_idx: Cell::new(self.root.value_idx()),
      },
    }
  }

  pub fn scoped<Out, F>(self: &'p Branch<'r, 'p, R>, f: F) -> Option<Out>
  where
    F: for<'b> FnOnce(&'b Branch<'r, 'b, R>) -> Option<Out>,
  {
    let branch = self.child();
    let val = f(&branch)?;
    match branch.commit() {
      Ok(_) => Some(val),
      Err(_err) => None,
    }
  }
}

impl<'r, 'p, R: 'r + Branchable> Drop for Branch<'r, 'p, R> {
  fn drop(&mut self) {
    if !self.committed {
      R::on_drop_branch(self)
    }
  }
}
