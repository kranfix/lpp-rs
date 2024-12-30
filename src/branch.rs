use std::ops::Deref;

use dupe::Dupe;

pub trait BranchRoot: Sized {
  type BranchData: UpdateFrom;
  type CommitError;

  fn data(&self) -> &Self::BranchData;

  fn branch(&self) -> crate::branch::Branch<'_, Self> {
    let data = self.data().dupe();
    crate::branch::Branch::new(self, data)
  }
}

pub trait BranchInspect<Root: BranchRoot>: Sized {
  fn inspect(branch: &CommitableBranch<'_, Root>) -> Option<Self>;
}

pub trait UpdateFrom: Dupe {
  fn update_from(&self, other: &Self);
}

#[derive(Debug)]
pub struct Branch<'p, R: BranchRoot> {
  root: &'p R,
  parent: Option<&'p Branch<'p, R>>,
  data: R::BranchData,
}
impl<'p, R: 'p + BranchRoot> Deref for Branch<'p, R> {
  type Target = R::BranchData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

pub struct CommitableBranch<'p, R: BranchRoot> {
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
    let root: &R = self.root;
    let data = &self.branch.data;

    match self.branch.parent {
      Some(parent) => {
        parent.data.update_from(data);
      }
      None => {
        root.data().update_from(data);
      }
    }

    self.committed = true;
    Ok(())
  }
}

impl<'p, R: BranchRoot> Branch<'p, R> {
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
  pub(crate) fn child(&self) -> CommitableBranch<'_, R> {
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
    F: FnOnce(&CommitableBranch<'_, R>) -> Option<Out>,
  {
    let branch = self.child();
    let val = f(&branch)?;
    match branch.commit() {
      Ok(_) => Some(val),
      Err(_err) => None,
    }
  }

  pub fn inspect<BI: BranchInspect<R>>(self: &'p Branch<'p, R>) -> Option<BI> {
    self.scoped(BI::inspect)
  }
}
