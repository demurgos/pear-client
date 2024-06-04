use crate::context::EmptyContext;
use compact_str::CompactString;

/// General package information
///
/// <https://pear.php.net/dtd/rest.package.xsd>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetReleaseQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub package: Str,
  pub version: Str,
}

pub type GetReleaseQueryView<'req, Cx, Str> = GetReleaseQuery<&'req Cx, Str>;

impl<Cx, Str> GetReleaseQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetReleaseQuery<NewCx, Str> {
    GetReleaseQuery {
      context: new_context,
      package: self.package,
      version: self.version,
    }
  }
}

impl<Cx, Str> GetReleaseQuery<Cx, Str>
  where Str: AsRef<str>
{
  pub fn as_view(&self) -> GetReleaseQueryView<'_, Cx, &str>
  {
    GetReleaseQueryView {
      context: &self.context,
      package: self.package.as_ref(),
      version: self.version.as_ref(),
    }
  }
}

impl GetReleaseQuery<EmptyContext> {
  pub const fn new(package: CompactString, version: CompactString) -> Self {
    Self {
      context: EmptyContext::new(),
      package,
      version,
    }
  }
}
