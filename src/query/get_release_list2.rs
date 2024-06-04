use crate::context::EmptyContext;
use compact_str::CompactString;

/// List of all releases including minimum PHP version
///
/// <https://pear.php.net/dtd/rest.allreleases2.xsd>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetReleaseList2Query<Cx, Str = CompactString> {
  pub context: Cx,
  pub package: Str,
}

pub type GetReleaseList2QueryView<'req, Cx, Str> = GetReleaseList2Query<&'req Cx, Str>;

impl<Cx, Str> GetReleaseList2Query<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetReleaseList2Query<NewCx, Str> {
    GetReleaseList2Query {
      context: new_context,
      package: self.package,
    }
  }
}


impl<Cx, Str> GetReleaseList2Query<Cx, Str>
  where Str: AsRef<str>
{
  pub fn as_view(&self) -> GetReleaseList2QueryView<'_, Cx, &str>
  {
    GetReleaseList2QueryView {
      context: &self.context,
      package: self.package.as_ref(),
    }
  }
}

impl GetReleaseList2Query<EmptyContext> {
  pub const fn new(package: CompactString) -> Self {
    Self {
      context: EmptyContext::new(),
      package,
    }
  }
}
