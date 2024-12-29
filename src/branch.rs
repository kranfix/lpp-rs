use dupe::Dupe;
use std::{cell::Cell, ops::Deref};

pub trait Branchable: Sized {
  type BranchData: Dupe;
  type CommitError;

  fn branch<'r>(&'r self) -> Branch<'r, Self>;
  fn commit_branch<'p>(branch: &mut Branch<'p, Self>) -> Result<(), Self::CommitError>;
  fn on_drop_branch<'p>(branch: &mut Branch<'p, Self>);

  fn value_idx(&self) -> usize;
}

#[derive(Debug)]
pub struct Branch<'p, R: Branchable> {
  root: &'p R,
  parent: Option<&'p Branch<'p, R>>,
  data: R::BranchData,
}
impl<'p, R: 'p + Branchable> Deref for Branch<'p, R> {
  type Target = R::BranchData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

pub(crate) struct CommitableBranch<'p, R: Branchable> {
  branch: Branch<'p, R>,
  committed: bool,
}
impl<'p, R: Branchable> Deref for CommitableBranch<'p, R> {
  type Target = Branch<'p, R>;

  fn deref(&self) -> &Self::Target {
    &self.branch
  }
}
impl<'p, R: Branchable> CommitableBranch<'p, R> {
  pub fn commit(mut self) -> Result<(), R::CommitError> {
    Branchable::commit_branch(&mut self.branch)?;
    self.committed = true;
    Ok(())
  }
}

impl<'p, R: Branchable> Branch<'p, R> {
  pub fn new(root: &'p R, data: R::BranchData) -> Self {
    Branch {
      root,
      parent: None,
      data,
    }
  }
  pub fn root(&self) -> &'p R {
    &self.root
  }
  pub fn parent(&self) -> Option<&'p Self> {
    self.parent
  }
  pub(crate) fn child<'b: 'p>(&'b self) -> CommitableBranch<'b, R> {
    CommitableBranch {
      branch: Branch {
        root: &self.root,
        parent: Some(self),
        data: self.data.dupe(),
      },
      committed: false,
    }
  }

  pub fn scoped<Out, F>(self: &'p Branch<'p, R>, f: F) -> Option<Out>
  where
    F: for<'b> FnOnce(&'b Branch<'b, R>) -> Option<Out>,
  {
    let branch = self.child();
    let val = f(&branch)?;
    match branch.commit() {
      Ok(_) => Some(val),
      Err(_err) => None,
    }
  }
}

impl<'p, R: 'p + Branchable> Drop for CommitableBranch<'p, R> {
  fn drop(&mut self) {
    if !self.committed {
      R::on_drop_branch(&mut self.branch)
    }
  }
}
