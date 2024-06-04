use crate::context::EmptyContext;
use compact_str::CompactString;

/// List of all releases
///
/// <http://pear.php.net/dtd/rest.allreleases.xsd>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetReleaseListQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub package: Str,
}

pub type GetReleaseListQueryView<'req, Cx, Str> = GetReleaseListQuery<&'req Cx, Str>;

impl<Cx, Str> GetReleaseListQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetReleaseListQuery<NewCx, Str> {
    GetReleaseListQuery {
      context: new_context,
      package: self.package,
    }
  }
}


impl<Cx, Str> GetReleaseListQuery<Cx, Str>
where Str: AsRef<str>
{
  pub fn as_view(&self) -> GetReleaseListQueryView<'_, Cx, &str>
  {
    GetReleaseListQueryView {
      context: &self.context,
      package: self.package.as_ref(),
    }
  }
}

impl GetReleaseListQuery<EmptyContext> {
  pub const fn new(package: CompactString) -> Self {
    Self {
      context: EmptyContext::new(),
      package,
    }
  }
}
