use compact_str::CompactString;
use markup5ever_rcdom::{Node, NodeData, RcDom};
use xml5ever::tendril::{TendrilSink};
use xml5ever::driver::{parse_document, XmlParseOpts};
use crate::xml_util::{find_root, get_link_attr, get_text};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseListing<Str = CompactString> {
  pub package: Str,
  pub channel: Str,
  pub items: Vec<ShortRelease<Str>>,
}

impl ReleaseListing<CompactString> {
  pub fn from_xml(mut input: &[u8]) -> Self {
    let input = &mut input;
    let sink = RcDom::default();
    let dom: RcDom = parse_document(sink, XmlParseOpts::default()).from_utf8().read_from(input).unwrap();
    Self::from_rc_dom(dom).unwrap()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum ReleaseListingFromRcDomError {
  #[error("failed to find root node")]
  RootNotFound,
  #[error("failed to read listing from XML Node")]
  Read(#[from] ReleaseListingFromXmlNodeError),
}

impl ReleaseListing<CompactString> {
  pub fn from_rc_dom(dom: RcDom) -> Result<Self, ReleaseListingFromRcDomError> {
    let doc = dom.document;
    let root = find_root(&doc, "a").map_err(|_| ReleaseListingFromRcDomError::RootNotFound)?;
    Ok(Self::from_xml_node(&root)?)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum ReleaseListingFromXmlNodeError {
  #[error("unexpected child node type at index {0}")]
  ChildType(usize),
  #[error("failed to read package at index {0}")]
  ReadPackage(usize),
  #[error("package node is missing")]
  MissingPackage,
  #[error("package node is duplicated")]
  DuplicatePackage,
  #[error("failed to read channel at index {0}")]
  ReadChannel(usize),
  #[error("channel node is missing")]
  MissingChannel,
  #[error("channel node is duplicated")]
  DuplicateChannel,
  #[error("failed to read release at index {1}")]
  ReadRelease(#[source] ShortReleaseFromXmlNodeError, usize),
}

impl ReleaseListing<CompactString> {
  pub fn from_xml_node(node: &Node) -> Result<Self, ReleaseListingFromXmlNodeError> {
    let mut package: Option<CompactString> = None;
    let mut channel: Option<CompactString> = None;
    let mut items: Vec<ShortRelease<CompactString>> = Vec::new();

    for (i, handle) in node.children.borrow().iter().enumerate() {
      let node: &Node = handle;
      match &node.data {
        NodeData::Element { name, .. } => {
          if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("p") {
            let new = get_text(node).map_err(|_| ReleaseListingFromXmlNodeError::ReadPackage(i))?;
            let old = package.replace(new);
            if old.is_some() {
              return Err(ReleaseListingFromXmlNodeError::DuplicatePackage);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("c") {
            if package.is_none() {
              // the XSD schema requires the channel to follow the package node
              return Err(ReleaseListingFromXmlNodeError::MissingPackage);
            }
            let new = get_text(node).map_err(|_| ReleaseListingFromXmlNodeError::ReadChannel(i))?;
            let old = channel.replace(new);
            if old.is_some() {
              return Err(ReleaseListingFromXmlNodeError::DuplicateChannel);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("r") {
            if channel.is_none() {
              // the XSD schema requires releases to follow the channel node
              return Err(ReleaseListingFromXmlNodeError::MissingChannel);
            }
            let r = ShortRelease::from_xml_node(node).map_err(|e| ReleaseListingFromXmlNodeError::ReadRelease(e, i))?;
            items.push(r);
          } else {
            return Err(ReleaseListingFromXmlNodeError::ChildType(i));
          }
        },
        NodeData::Text { .. } | NodeData::Comment { .. } => { continue },
        _ => return Err(ReleaseListingFromXmlNodeError::ChildType(i)),
      }
    }

    Ok(Self {
      package: package.ok_or(ReleaseListingFromXmlNodeError::MissingPackage)?,
      channel: channel.ok_or(ReleaseListingFromXmlNodeError::MissingChannel)?,
      items,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortRelease<Str = CompactString> {
  pub version: Str,
  pub stability: Str,
  // pub php_version: ...
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum ShortReleaseFromXmlNodeError {
  #[error("unexpected child node type at index {0}")]
  ChildType(usize),
  #[error("failed to read version at index {0}")]
  ReadVersion(usize),
  #[error("version node is missing")]
  MissingVersion,
  #[error("version node is duplicated")]
  DuplicateVersion,
  #[error("failed to read channel at index {0}")]
  ReadStability(usize),
  #[error("channel node is missing")]
  MissingStability,
  #[error("channel node is duplicated")]
  DuplicateStability,
}

impl ShortRelease<CompactString> {
  pub fn from_xml_node(node: &Node) -> Result<Self, ShortReleaseFromXmlNodeError> {
    let mut version: Option<CompactString> = None;
    let mut stability: Option<CompactString> = None;
    for (i, handle) in node.children.borrow().iter().enumerate() {
      let node: &Node = handle;
      match &node.data {
        NodeData::Element { name, .. } => {
          if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("v") {
            let new = get_text(node).map_err(|_| ShortReleaseFromXmlNodeError::ReadVersion(i))?;
            let old = version.replace(new);
            if old.is_some() {
              return Err(ShortReleaseFromXmlNodeError::DuplicateVersion);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("s") {
            if version.is_none() {
              // the XSD schema requires the stability to follow the version node
              return Err(ShortReleaseFromXmlNodeError::MissingVersion);
            }
            let new = get_text(node).map_err(|_| ShortReleaseFromXmlNodeError::ReadStability(i))?;
            let old = stability.replace(new);
            if old.is_some() {
              return Err(ShortReleaseFromXmlNodeError::DuplicateStability);
            }
          }
        },
        NodeData::Text { .. } | NodeData::Comment { .. } => { continue },
        _ => return Err(ShortReleaseFromXmlNodeError::ChildType(i)),
      }
    }

    Ok(Self {
      version: version.ok_or(ShortReleaseFromXmlNodeError::MissingVersion)?,
      stability: stability.ok_or(ShortReleaseFromXmlNodeError::MissingStability)?,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Release<Str = CompactString> {
  pub package: ReleasePackage<Str>,
  pub channel: Str,
  pub version: Str,
  pub status: Str,
  pub license: Str,
  pub maintainer: Str,
  pub summary: Str,
  pub description: Str,
  pub time: Str,
  pub release_notes: Str,
  pub archive: ReleaseArchive<Str>,
  /// Link to extracted release info
  pub extracted_link: Str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleasePackage<Str = CompactString> {
  pub name: Str,
  pub link: Str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseArchive<Str = CompactString> {
  pub size: u64,
  pub link: Str,
}

impl Release<CompactString> {
  pub fn from_xml(mut input: &[u8]) -> Self {
    let input = &mut input;
    let sink = RcDom::default();
    let dom: RcDom = parse_document(sink, XmlParseOpts::default()).from_utf8().read_from(input).unwrap();
    Self::from_rc_dom(dom).unwrap()
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum ReleaseFromRcDomError {
  #[error("failed to find root node")]
  RootNotFound,
  #[error("failed to read release from XML Node")]
  Read(#[from] crate::common::release::ReleaseFromXmlNodeError),
}

impl Release<CompactString> {
  pub fn from_rc_dom(dom: RcDom) -> Result<Self, ReleaseFromRcDomError> {
    let doc = dom.document;
    let root = find_root(&doc, "r").map_err(|_| ReleaseFromRcDomError::RootNotFound)?;
    Ok(Self::from_xml_node(&root)?)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum ReleaseFromXmlNodeError {
  #[error("unexpected child node type at index {0}")]
  ChildType(usize),
  #[error("package node <p> is malformed at index {0}")]
  ReadPackage(usize),
  #[error("package node <p> is missing attribute `xlink:hrf` at index {0}")]
  MissingPackageLink(usize),
  #[error("package node <p> has duplicate attribute `xlink:hrf` at index {0}")]
  DuplicatePackageLink(usize),
  #[error("package node <p> is missing")]
  MissingPackage,
  #[error("package node <p> is duplicated")]
  DuplicatePackage,
  #[error("channel node <c> is malformed at index {0}")]
  ReadChannel(usize),
  #[error("channel node <c> is missing")]
  MissingChannel,
  #[error("channel node <c> is duplicated")]
  DuplicateChannel,
  #[error("version node <v> is malformed at index {0}")]
  ReadVersion(usize),
  #[error("version node <v> is missing")]
  MissingVersion,
  #[error("version node <v> is duplicated")]
  DuplicateVersion,
  #[error("status node <st> is malformed at index {0}")]
  ReadStatus(usize),
  #[error("status node <st> is missing")]
  MissingStatus,
  #[error("status node <st> is duplicated")]
  DuplicateStatus,
  #[error("license node <l> is malformed at index {0}")]
  ReadLicense(usize),
  #[error("license node <l> is missing")]
  MissingLicense,
  #[error("license node <l> is duplicated")]
  DuplicateLicense,
  #[error("maintainer node <m> is malformed at index {0}")]
  ReadMaintainer(usize),
  #[error("maintainer node <m> is missing")]
  MissingMaintainer,
  #[error("maintainer node <m> is duplicated")]
  DuplicateMaintainer,
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
  #[error("date node <da> is malformed at index {0}")]
  ReadDate(usize),
  #[error("date node <da> is missing")]
  MissingDate,
  #[error("date node <da> is duplicated")]
  DuplicateDate,
  #[error("release notes node <n> is malformed at index {0}")]
  ReadReleaseNotes(usize),
  #[error("release notes node <n> is missing")]
  MissingReleaseNotes,
  #[error("release notes node <n> is duplicated")]
  DuplicateReleaseNotes,
  #[error("archive size node <f> is malformed at index {0}")]
  ReadArchiveSize(usize),
  #[error("archive size node <f> is missing")]
  MissingArchiveSize,
  #[error("archive size node <f> is duplicated")]
  DuplicateArchiveSize,
  #[error("archive size node <f> contains invalid value")]
  InvalidArchiveSize,
  #[error("archive link node <g> is malformed at index {0}")]
  ReadArchiveLink(usize),
  #[error("archive link node <g> is missing")]
  MissingArchiveLink,
  #[error("archive link node <g> is duplicated")]
  DuplicateArchiveLink,
  #[error("extracted link node <x> is malformed at index {0}")]
  ReadExtracted(usize),
  #[error("extracted link node <x> is missing")]
  MissingExtracted,
  #[error("extracted link node <x> is duplicated")]
  DuplicateExtracted,
  #[error("extracted link node <x> has invalid attribute xlink:href")]
  InvalidExtracted,
}

impl Release<CompactString> {
  pub fn from_xml_node(node: &Node) -> Result<Self, ReleaseFromXmlNodeError> {
    use ReleaseFromXmlNodeError as E;

    let mut package: Option<ReleasePackage<CompactString>> = None;
    let mut channel: Option<CompactString> = None;
    let mut version: Option<CompactString> = None;
    let mut status: Option<CompactString> = None;
    let mut license: Option<CompactString> = None;
    let mut maintainer: Option<CompactString> = None;
    let mut summary: Option<CompactString> = None;
    let mut description: Option<CompactString> = None;
    let mut date: Option<CompactString> = None;
    let mut release_notes: Option<CompactString> = None;
    let mut archive_size: Option<CompactString> = None;
    let mut archive_link: Option<CompactString> = None;
    let mut extracted: Option<CompactString> = None;

    for (i, handle) in node.children.borrow().iter().enumerate() {
      let node: &Node = handle;
      match &node.data {
        NodeData::Element { name, attrs, .. } => {
          if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("p") {
            let attrs = &*attrs.borrow();
            let new_name = get_text(node).map_err(|_| E::ReadPackage(i))?;
            let new_link = get_link_attr(attrs.as_slice()).map_err(|_| E::DuplicatePackageLink(i) )?.ok_or(E::MissingPackageLink(i))?;
            let new_package = ReleasePackage {
              name: new_name,
              link: CompactString::new(new_link.value.as_ref()),
            };
            let old = package.replace(new_package);
            if old.is_some() {
              return Err(E::DuplicatePackage);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("c") {
            if package.is_none() {
              return Err(E::MissingPackage);
            }
            let new = get_text(node).map_err(|_| E::ReadChannel(i))?;
            let old = channel.replace(new);
            if old.is_some() {
              return Err(E::DuplicateChannel);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("v") {
            if channel.is_none() {
              return Err(E::MissingChannel);
            }
            let new = get_text(node).map_err(|_| E::ReadVersion(i))?;
            let old = version.replace(new);
            if old.is_some() {
              return Err(E::DuplicateVersion);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("st") {
            if version.is_none() {
              return Err(E::MissingVersion);
            }
            let new = get_text(node).map_err(|_| E::ReadStatus(i))?;
            let old = status.replace(new);
            if old.is_some() {
              return Err(E::DuplicateStatus);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("l") {
            if status.is_none() {
              return Err(E::MissingStatus);
            }
            let new = get_text(node).map_err(|_| E::ReadLicense(i))?;
            let old = license.replace(new);
            if old.is_some() {
              return Err(E::DuplicateLicense);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("m") {
            if license.is_none() {
              return Err(E::MissingLicense);
            }
            let new = get_text(node).map_err(|_| E::ReadMaintainer(i))?;
            let old = maintainer.replace(new);
            if old.is_some() {
              return Err(E::DuplicateMaintainer);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("s") {
            if maintainer.is_none() {
              return Err(E::MissingMaintainer);
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
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("da") {
            if description.is_none() {
              return Err(E::MissingDescription);
            }
            let new = get_text(node).map_err(|_| E::ReadDate(i))?;
            let old = date.replace(new);
            if old.is_some() {
              return Err(E::DuplicateDate);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("n") {
            if date.is_none() {
              return Err(E::MissingDate);
            }
            let new = get_text(node).map_err(|_| E::ReadReleaseNotes(i))?;
            let old = release_notes.replace(new);
            if old.is_some() {
              return Err(E::DuplicateReleaseNotes);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("f") {
            if release_notes.is_none() {
              return Err(E::MissingReleaseNotes);
            }
            let new = get_text(node).map_err(|_| E::ReadArchiveSize(i))?;
            let old = archive_size.replace(new);
            if old.is_some() {
              return Err(E::DuplicateArchiveSize);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("g") {
            if archive_size.is_none() {
              return Err(E::MissingArchiveSize);
            }
            let new = get_text(node).map_err(|_| E::ReadArchiveLink(i))?;
            let old = archive_link.replace(new);
            if old.is_some() {
              return Err(E::DuplicateArchiveLink);
            }
          } else if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case("x") {
            let attrs = &*attrs.borrow();
            if archive_link.is_none() {
              return Err(E::MissingArchiveLink);
            }
            let new = get_link_attr(attrs).map_err(|_| E::ReadExtracted(i))?.ok_or(E::InvalidExtracted)?;
            let old = extracted.replace(CompactString::new(new.value.as_ref()));
            if old.is_some() {
              return Err(E::DuplicateExtracted);
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
      package: package.ok_or(E::MissingPackage)?,
      channel: channel.ok_or(E::MissingChannel)?,
      version: version.ok_or(E::MissingVersion)?,
      status: status.ok_or(E::MissingStatus)?,
      license: license.ok_or(E::MissingLicense)?,
      maintainer: maintainer.ok_or(E::MissingMaintainer)?,
      summary: summary.ok_or(E::MissingSummary)?,
      description: description.ok_or(E::MissingDescription)?,
      time: date.ok_or(E::MissingDate)?,
      release_notes: release_notes.ok_or(E::MissingReleaseNotes)?,
      archive: ReleaseArchive {
        size: archive_size.ok_or(E::MissingArchiveSize)?.parse::<u64>().map_err(|_| E::InvalidArchiveSize)?,
        link: archive_link.ok_or(E::MissingArchiveLink)?,
      },
      extracted_link: extracted.ok_or(E::MissingExtracted)?,
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn test_release_listing_from_xml() {
    let input = include_bytes!("../../test-resources/get_release_list/pecl_protobuf/input.xml");
    let actual = ReleaseListing::from_xml(input);
    assert_eq!(actual.channel.as_str(), "pecl.php.net");
    assert_eq!(actual.items.len(), 141);
  }

  #[test]
  pub fn test_release_from_xml() {
    let input = include_bytes!("../../test-resources/get_release/pecl_protobuf_4.27.0/input.xml");
    let actual = Release::from_xml(input);
    assert_eq!(actual.channel.as_str(), "pecl.php.net");
  }
}
