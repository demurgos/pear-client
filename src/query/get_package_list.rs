use crate::context::EmptyContext;

/// List of all packages
///
/// <http://pear.php.net/dtd/rest.allpackages.xsd>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPackageListQuery<Cx> {
  pub context: Cx,
}

pub type GetPackageListQueryView<'req, Cx> = GetPackageListQuery<&'req Cx>;

impl<Cx> GetPackageListQuery<Cx> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetPackageListQuery<NewCx> {
    GetPackageListQuery {
      context: new_context,
    }
  }

  pub fn as_view(&self) -> GetPackageListQueryView<'_, Cx>
  {
    GetPackageListQueryView {
      context: &self.context,
    }
  }
}

impl GetPackageListQuery<EmptyContext> {
  pub const fn new() -> Self {
    Self {
      context: EmptyContext::new(),
    }
  }
}

impl Default for GetPackageListQuery<EmptyContext> {
    fn default() -> Self {
        Self::new()
    }
}
