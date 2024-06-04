use url::Url;

pub fn url_join<I>(url: &Url, segments: I) -> Url
where
  I: IntoIterator,
  I::Item: AsRef<str>,
{
  let mut res: Url = url.clone();
  {
    let mut p = res.path_segments_mut().expect("GitLab URL has path segments");
    p.extend(["rest"]);
    p.extend(segments);
  }
  res
}

pub trait UrlExt {
  fn url_join<I>(&self, segments: I) -> Self
  where
    I: IntoIterator,
    I::Item: AsRef<str>;
}

impl UrlExt for Url {
  fn url_join<I>(&self, segments: I) -> Self
  where
    I: IntoIterator,
    I::Item: AsRef<str>,
  {
    url_join(self, segments)
  }
}
