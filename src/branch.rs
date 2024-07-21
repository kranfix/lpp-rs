use std::ops::Deref;

pub trait Branchable: Sized {
  type BranchData: Clone;
  type CommitError;

  fn branch<'r>(&'r self) -> Branch<'r, 'r, Self>;
  fn commit_branch<'r, 'p>(branch: &Branch<'r, 'p, Self>) -> Result<(), Self::CommitError>;
}

pub struct Branch<'r, 'p, R: 'r + Branchable> {
  root: &'r R,
  parent: Option<&'p Branch<'r, 'p, R>>,
  data: R::BranchData,
}
impl<'r, 'p, R: 'r + Branchable> Deref for Branch<'r, 'p, R> {
  type Target = R::BranchData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<'r, 'p, R: 'r + Branchable> Branch<'r, 'p, R> {
  pub fn root(&self) -> &'r R {
    &self.root
  }
  pub fn parent(&self) -> Option<&Self> {
    self.parent
  }
  pub fn child<'b>(&'b self) -> Branch<'r, 'b, R> {
    Branch {
      root: &self.root,
      parent: Some(self),
      data: self.data.clone(),
    }
  }
  pub fn commit(&self) -> Result<(), R::CommitError> {
    Branchable::commit_branch(self)
  }
}
