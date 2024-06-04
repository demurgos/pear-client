use crate::context::EmptyContext;
use compact_str::CompactString;

/// General package information
///
/// <https://pear.php.net/dtd/rest.package.xsd>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetPackageInfoQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub package: Str,
}

pub type GetPackageInfoQueryView<'req, Cx, Str> = GetPackageInfoQuery<&'req Cx, Str>;

impl<Cx, Str> GetPackageInfoQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetPackageInfoQuery<NewCx, Str> {
    GetPackageInfoQuery {
      context: new_context,
      package: self.package,
    }
  }
}


impl<Cx, Str> GetPackageInfoQuery<Cx, Str>
where Str: AsRef<str>
{
  pub fn as_view(&self) -> GetPackageInfoQueryView<'_, Cx, &str>
  {
    GetPackageInfoQueryView {
      context: &self.context,
      package: self.package.as_ref(),
    }
  }
}

impl GetPackageInfoQuery<EmptyContext> {
  pub const fn new(package: CompactString) -> Self {
    Self {
      context: EmptyContext::new(),
      package,
    }
  }
}
