use std::ops::Deref;
use url::Url;

/// A very restricted version of frunk hlist to hold the context for PEAR client requests.
pub struct Context<TyPearUrl> {
  /// PEAR channel URL.
  pear_url: TyPearUrl,
}

impl<TyPearUrl> Context<TyPearUrl> {
  pub fn set_pear_url<NewPearUrl>(self, pear_url: NewPearUrl) -> Context<NewPearUrl> {
    Context { pear_url }
  }
}

pub type EmptyContext = Context<()>;

impl EmptyContext {
  pub const fn new() -> Self {
    Self { pear_url: () }
  }
}

impl Default for EmptyContext {
    fn default() -> Self {
        Self::new()
    }
}

pub trait GetRef<T: ?Sized> {
  fn get_ref(&self) -> &T;
}

pub struct PearUrl(pub Url);

impl Deref for PearUrl {
  type Target = Url;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl GetRef<PearUrl> for Context<PearUrl> {
  fn get_ref(&self) -> &PearUrl {
    &self.pear_url
  }
}
