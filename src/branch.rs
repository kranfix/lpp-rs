use dupe::Dupe;
use std::ops::Deref;

pub trait Branchable: Sized {
  type BranchData: Dupe;
  type CommitError;

  fn branch<'r>(&'r self) -> Branch<'r, 'r, Self>;
  fn commit_branch<'r, 'p>(branch: &mut Branch<'r, 'p, Self>) -> Result<(), Self::CommitError>;
  fn on_drop_branch<'r, 'p>(branch: &mut Branch<'r, 'p, Self>);
}

#[derive(Debug)]
pub struct Branch<'r, 'p, R: Branchable> {
  root: &'r R,
  parent: Option<&'p Branch<'r, 'p, R>>,
  data: R::BranchData,
  committed: bool,
}
impl<'r, 'p, R: 'r + Branchable> Deref for Branch<'r, 'p, R> {
  type Target = R::BranchData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<'r, 'p, R: Branchable> Branch<'r, 'p, R> {
  pub fn new(root: &'r R, data: R::BranchData) -> Self {
    Branch {
      root,
      parent: None,
      data,
      committed: false,
    }
  }
  pub fn root(&self) -> &'r R {
    &self.root
  }
  pub fn parent(&self) -> Option<&'p Self> {
    self.parent
  }
  pub fn child<'b>(&'b self) -> Branch<'r, 'b, R> {
    Branch {
      root: &self.root,
      parent: Some(self),
      data: self.data.dupe(),
      committed: false,
    }
  }
  pub fn commit(mut self) -> Result<(), R::CommitError> {
    Branchable::commit_branch(&mut self)?;
    self.committed = true;
    Ok(())
  }
  pub fn abort(self) {
    drop(self)
  }
}

impl<'r, 'p, R: 'r + Branchable> Drop for Branch<'r, 'p, R> {
  fn drop(&mut self) {
    if !self.committed {
      R::on_drop_branch(self)
    }
  }
}
