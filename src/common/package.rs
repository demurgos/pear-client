use compact_str::CompactString;
use markup5ever_rcdom::{Node, NodeData, RcDom};
use xml5ever::tendril::{TendrilSink};
use xml5ever::driver::{parse_document, XmlParseOpts};
use crate::xml_util::{find_root, get_text};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageListing<Str = CompactString> {
  pub category: Str,
  pub items: Vec<Str>,
}

impl PackageListing<CompactString> {
  pub fn from_xml(mut input: &[u8]) -> Self {
    let input = &mut input;
    let sink = RcDom::default();
    let dom: RcDom = parse_document(sink, XmlParseOpts::default()).from_utf8().read_from(input).unwrap();
    Self::from_rc_dom(dom).unwrap()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum PackageListingFromRcDomError {
  #[error("failed to find root node")]
  RootNotFound,
  #[error("failed to read listing from XML Node")]
  Read(#[from] PackageListingFromXmlNodeError),
}

impl PackageListing<CompactString> {
  pub fn from_rc_dom(dom: RcDom) -> Result<Self, PackageListingFromRcDomError> {
    let doc = dom.document;
    let root = find_root(&doc, "a").map_err(|_| PackageListingFromRcDomError::RootNotFound)?;
    Ok(Self::from_xml_node(&root)?)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum PackageListingFromXmlNodeError {
  #[error("unexpected child node type at index {0}")]
  ChildType(usize),
  #[error("failed to read category at index {0}")]
  ReadCategory(usize),
  #[error("category node is missing")]
  MissingCategory,
  #[error("category node is duplicated")]
  DuplicateCategory,
  #[error("failed to read package at index {0}")]
  ReadPackage(usize),
}

impl PackageListing<CompactString> {
  pub fn from_xml_node(node: &Node) -> Result<Self, PackageListingFromXmlNodeError> {
    let mut category: Option<CompactString> = None;
    let mut items: Vec<CompactString> = Vec::new();

    for (i, handle) in node.children.borrow().iter().enumerate() {
      let node: &Node = handle;
      match &node.data {
        NodeData::Element { name, .. } => {
          if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("c") {
            let new = get_text(node).map_err(|_| PackageListingFromXmlNodeError::ReadCategory(i))?;
            let old = category.replace(new);
            if old.is_some() {
              return Err(PackageListingFromXmlNodeError::DuplicateCategory);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("p") {
            if category.is_none() {
              // the XSD schema requires packages to follow the category node
              return Err(PackageListingFromXmlNodeError::MissingCategory);
            }
            let p = get_text(node).map_err(|_| PackageListingFromXmlNodeError::ReadPackage(i))?;
            items.push(p);
          } else {
            return Err(PackageListingFromXmlNodeError::ChildType(i));
          }
        },
        NodeData::Text { .. } | NodeData::Comment { .. } => { continue },
        _ => return Err(PackageListingFromXmlNodeError::ChildType(i)),
      }
    }

    Ok(Self {
      category: category.ok_or(PackageListingFromXmlNodeError::MissingCategory)?,
      items,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageInfo<Str = CompactString> {
  pub name: Str,
  pub channel: Str,
  pub category: Str,
  pub license: Str,
  pub license_uri: Option<Str>,
  pub summary: Str,
  pub description: Str,
  pub release_uri: Str,
  pub parent_package: Option<Str>,
  /// If this package is deprecated, deprecation info
  pub deprecation: Option<DeprecationInfo<Str>>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeprecationInfo<Str = CompactString> {
  /// Channel of the recommended replacement
  recommended_channel: Str,
  /// Name of the recommended replacement
  recommended_package: Str,
}

impl PackageInfo<CompactString> {
  pub fn from_xml(mut input: &[u8]) -> Self {
    let input = &mut input;
    let sink = RcDom::default();
    let dom: RcDom = parse_document(sink, XmlParseOpts::default()).from_utf8().read_from(input).unwrap();
    Self::from_rc_dom(dom).unwrap()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum PackageInfoFromRcDomError {
  #[error("failed to find root node")]
  RootNotFound,
  #[error("failed to read listing from XML Node")]
  Read(#[from] PackageInfoFromXmlNodeError),
}

impl PackageInfo<CompactString> {
  pub fn from_rc_dom(dom: RcDom) -> Result<Self, PackageInfoFromRcDomError> {
    let doc = dom.document;
    let root = find_root(&doc, "p").map_err(|_| PackageInfoFromRcDomError::RootNotFound)?;
    Ok(Self::from_xml_node(&root)?)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum PackageInfoFromXmlNodeError {
  #[error("unexpected child node type at index {0}")]
  ChildType(usize),
  #[error("name node <n> is malformed at index {0}")]
  ReadName(usize),
  #[error("name node <n> is missing")]
  MissingName,
  #[error("name node <n> is duplicated")]
  DuplicateName,
  #[error("channel node <c> is malformed at index {0}")]
  ReadChannel(usize),
  #[error("channel node <c> is missing")]
  MissingChannel,
  #[error("channel node <c> is duplicated")]
  DuplicateChannel,
  #[error("category node <ca> is malformed at index {0}")]
  ReadCategory(usize),
  #[error("category node <ca> is missing")]
  MissingCategory,
  #[error("category node <ca> is duplicated")]
  DuplicateCategory,
  #[error("license node <l> is malformed at index {0}")]
  ReadLicense(usize),
  #[error("license node <l> is missing")]
  MissingLicense,
  #[error("license node <l> is duplicated")]
  DuplicateLicense,
  #[error("license uri node <lu> is malformed at index {0}")]
  ReadLicenseUri(usize),
  #[error("license uri node <lu> is duplicated")]
  DuplicateLicenseUri,
  #[error("summary node <s> is malformed at index {0}")]
  ReadSummary(usize),
  #[error("summary node <s> is missing")]
  MissingSummary,
  #[error("summary node <s> is duplicated")]
  DuplicateSummary,
  #[error("description node <d> is malformed at index {0}")]
  ReadDescription(usize),
  #[error("description node <d> is missing")]
  MissingDescription,
  #[error("description node <d> is duplicated")]
  DuplicateDescription,
  #[error("release node <r> is malformed at index {0}")]
  ReadRelease(usize),
  #[error("release node <r> is missing")]
  MissingRelease,
  #[error("release node <r> is duplicated")]
  DuplicateRelease,
  #[error("parent node <pa> is malformed at index {0}")]
  ReadParent(usize),
  #[error("parent node <pa> is duplicated")]
  DuplicateParent,
  #[error("deprecation channel node <dc> is malformed at index {0}")]
  ReadDeprecationChannel(usize),
  #[error("deprecation channel node <dc> is missing")]
  MissingDeprecationChannel,
  #[error("deprecation channel node <dc> is duplicated")]
  DuplicateDeprecationChannel,
  #[error("deprecation package node <dp> is malformed at index {0}")]
  ReadDeprecationPackage(usize),
  #[error("deprecation package node <dp> is missing")]
  MissingDeprecationPackage,
  #[error("deprecation package node <dp> is duplicated")]
  DuplicateDeprecationPackage,
  // #[error("failed to read release at index {1}")]
  // ReadRelease(#[source] ReleaseFromXmlNodeError, usize),
}

impl PackageInfo<CompactString> {
  pub fn from_xml_node(node: &Node) -> Result<Self, PackageInfoFromXmlNodeError> {
    use PackageInfoFromXmlNodeError as E;

    let mut package_name: Option<CompactString> = None;
    let mut channel: Option<CompactString> = None;
    let mut category: Option<CompactString> = None;
    let mut license: Option<CompactString> = None;
    let mut license_uri: Option<CompactString> = None;
    let mut summary: Option<CompactString> = None;
    let mut description: Option<CompactString> = None;
    let mut release: Option<CompactString> = None;
    let mut parent: Option<CompactString> = None;
    let mut deprecation_channel: Option<CompactString> = None;
    let mut deprecation_package: Option<CompactString> = None;
    // let mut deprecation: Option<DeprecationInfo<CompactString>> = None;

    for (i, handle) in node.children.borrow().iter().enumerate() {
      let node: &Node = handle;
      match &node.data {
        NodeData::Element { name, .. } => {
          if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("n") {
            let new = get_text(node).map_err(|_| E::ReadName(i))?;
            let old = package_name.replace(new);
            if old.is_some() {
              return Err(E::DuplicateName);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("c") {
            if package_name.is_none() {
              return Err(E::MissingName);
            }
            let new = get_text(node).map_err(|_| E::ReadChannel(i))?;
            let old = channel.replace(new);
            if old.is_some() {
              return Err(E::DuplicateChannel);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("ca") {
            if channel.is_none() {
              return Err(E::MissingChannel);
            }
            let new = get_text(node).map_err(|_| E::ReadCategory(i))?;
            let old = category.replace(new);
            if old.is_some() {
              return Err(E::DuplicateCategory);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("l") {
            if category.is_none() {
              return Err(E::MissingCategory);
            }
            let new = get_text(node).map_err(|_| E::ReadLicense(i))?;
            let old = license.replace(new);
            if old.is_some() {
              return Err(E::DuplicateLicense);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("lu") {
            // todo: check that summary is none
            if license.is_none() {
              return Err(E::MissingLicense);
            }
            let new = get_text(node).map_err(|_| E::ReadLicenseUri(i))?;
            let old = license_uri.replace(new);
            if old.is_some() {
              return Err(E::DuplicateLicenseUri);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("s") {
            if license.is_none() {
              // we still check `license` since `license_uri` is optional
              return Err(E::MissingLicense);
            }
            let new = get_text(node).map_err(|_| E::ReadSummary(i))?;
            let old = summary.replace(new);
            if old.is_some() {
              return Err(E::DuplicateSummary);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("d") {
            if summary.is_none() {
              return Err(E::MissingSummary);
            }
            let new = get_text(node).map_err(|_| E::ReadDescription(i))?;
            let old = description.replace(new);
            if old.is_some() {
              return Err(E::DuplicateDescription);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("r") {
            if description.is_none() {
              return Err(E::MissingDescription);
            }
            let new = get_text(node).map_err(|_| E::ReadRelease(i))?;
            let old = release.replace(new);
            if old.is_some() {
              return Err(E::DuplicateRelease);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("pa") {
            // todo: check that deprecation channel is none
            if description.is_none() {
              return Err(E::MissingRelease);
            }
            let new = get_text(node).map_err(|_| E::ReadParent(i))?;
            let old = parent.replace(new);
            if old.is_some() {
              return Err(E::DuplicateParent);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("dc") {
            if release.is_none() {
              // we still check `release` since `parent` is optional
              return Err(E::MissingRelease);
            }
            let new = get_text(node).map_err(|_| E::ReadDeprecationChannel(i))?;
            let old = deprecation_channel.replace(new);
            if old.is_some() {
              return Err(E::DuplicateDeprecationChannel);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("dp") {
            if deprecation_channel.is_none() {
              return Err(E::MissingDeprecationChannel);
            }
            let new = get_text(node).map_err(|_| E::ReadDeprecationPackage(i))?;
            let old = deprecation_package.replace(new);
            if old.is_some() {
              return Err(E::DuplicateDeprecationPackage);
            }
          } else {
            return Err(E::ChildType(i));
          }
        },
        NodeData::Text { .. } | NodeData::Comment { .. } => { continue },
        _ => return Err(E::ChildType(i)),
      }
    }

    Ok(Self {
      name: package_name.ok_or(E::MissingName)?,
      channel: channel.ok_or(E::MissingChannel)?,
      category: category.ok_or(E::MissingCategory)?,
      license: license.ok_or(E::MissingLicense)?,
      license_uri,
      summary: summary.ok_or(E::MissingSummary)?,
      description: description.ok_or(E::MissingDescription)?,
      release_uri: release.ok_or(E::MissingRelease)?,
      parent_package: parent,
      deprecation: match (deprecation_channel, deprecation_package) {
        (Some(dc), Some(dp)) => Some(DeprecationInfo { recommended_channel: dc, recommended_package: dp }),
        (Some(_), None) => return Err(E::MissingDeprecationPackage),
        (None, Some(_)) => return Err(E::MissingDeprecationChannel),
        (None, None) => None,
      },
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn test_package_listing_from_xml() {
    let input = include_bytes!("../../test-resources/get_package_list/pecl/input.xml");
    let actual = PackageListing::from_xml(input);
    assert_eq!(actual.category.as_str(), "pecl.php.net");
    assert_eq!(actual.items.len(), 434);
  }

  #[test]
  pub fn test_package_info_from_xml() {
    let input = include_bytes!("../../test-resources/get_package_info/pecl_protobuf/input.xml");
    let actual = PackageInfo::from_xml(input);
    assert_eq!(actual.name.as_str(), "protobuf");
    assert_eq!(actual.category.as_str(), "Tools and Utilities");
  }
}
