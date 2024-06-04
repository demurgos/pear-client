use std::cell::RefCell;
use compact_str::CompactString;
use markup5ever_rcdom::{Handle, Node, NodeData};
use xml5ever::{Attribute, local_name, ns, QualName, namespace_url, namespace_prefix};
use xml5ever::tendril::StrTendril;

/// Get the root element node out of the document node.
pub(crate) fn find_root(doc: &Node, expected_name: &str) -> Result<Handle, ()> {
  assert!(matches!(doc.data, NodeData::Document));
  let children = doc.children.borrow();
  let mut root: Option<&Handle> = None;
  for handle in children.iter() {
    let node: &Node = handle;
    if let NodeData::Element { name, ..} = &node.data {
      if name.prefix.is_none() && name.local.eq_str_ignore_ascii_case(expected_name) {
        let old = root.replace(handle);
        if old.is_some() {
          return Err(()); // duplicate
        }
      }
    }
  }
  root.cloned().ok_or(())
}

/// Read the text of a Node with only text content.
/// If the node is empty, the empty string is returned.
/// Otherwise, If the node does not contain a single child text node, an error is returned
pub(crate) fn get_text(node: &Node) -> Result<CompactString, ()> {
  let children = node.children.borrow();
  let mut text: Option<&RefCell<StrTendril>> = None;
  for handle in children.iter() {
    let node: &Node = handle;
    if let NodeData::Text { contents } = &node.data {
      let old = text.replace(contents);
      if old.is_some() {
        return Err(()); // duplicate
      }
    }
  }
  Ok(match text {
    None => CompactString::default(),
    Some(text) => {
      let text: &StrTendril = &text.borrow();
      CompactString::new(text)
    }
  })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
#[error("duplicate attribute")]
pub struct DuplicateAttribute;

pub(crate) fn get_attr<'a>(attrs: &'a [Attribute], name: &QualName) -> Result<Option<&'a Attribute>, ()> {
  let mut result: Option<&Attribute> = None;
  for attr in attrs {
    if attr.name != *name {
      continue;
    }
    let old = result.replace(attr);
    if old.is_some() {
      return Err(());
    }
  }
  Ok(result)
}

pub const XLINK_HREF: QualName = QualName {
  ns: ns!(xlink),
  prefix: Some(namespace_prefix!("xlink")),
  local: local_name!("href"),
};

pub(crate) fn get_link_attr(attrs: &[Attribute]) -> Result<Option<&'_ Attribute>, ()> {
  get_attr(attrs, &XLINK_HREF)
}
