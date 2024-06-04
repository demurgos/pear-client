use crate::context::{GetRef, PearUrl};
use crate::url_util::UrlExt;
use bytes::Bytes;
use core::task::{Context, Poll};
use futures::future::BoxFuture;
use http::{Method, Request, Response,};
use http_body::Body;
use http_body_util::{BodyExt, Full};
use std::error::Error as StdError;
use compact_str::format_compact;
use tower_service::Service;
use crate::common::package::{PackageInfo, PackageListing};
use crate::common::release::{Release, ReleaseListing};
use crate::query::get_package_info::GetPackageInfoQuery;
use crate::query::get_package_list::GetPackageListQuery;
use crate::query::get_release::GetReleaseQuery;
use crate::query::get_release_list::GetReleaseListQuery;

pub struct HttpPearClient<TyInner> {
  inner: TyInner,
}

impl<TyInner> HttpPearClient<TyInner> {
  pub fn new(inner: TyInner) -> Self {
    Self { inner }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum HttpPearClientError {
  #[error("failed to poll ready status: {0}")]
  PollReady(String),
  #[error("failed to send request: {0}")]
  Send(String),
  #[error("failed to receive response: {0}")]
  Receive(String),
  #[error("failed to parse response: {0}")]
  ResponseFormat(String, Bytes),
  #[error("operation is forbidden for provided auth")]
  Forbidden,
  #[error("resource already exists")]
  Conflict,
  #[error("resource not found")]
  NotFound,
  #[error("unexpected error: {0}")]
  Other(String),
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetPackageListQuery<Cx>> for HttpPearClient<TyInner>
where
  Cx: GetRef<PearUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = PackageListing;
  type Error = HttpPearClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpPearClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetPackageListQuery<Cx>) -> Self::Future {
    let url = req.context.get_ref().url_join(["p", "packages.xml"]);

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .body(Full::new(Bytes::new()))
      .unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpPearClientError::Send(format!("{e:?}")))?;
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpPearClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();
      let result = PackageListing::from_xml(body.as_ref());
      Ok(result)
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetReleaseListQuery<Cx>> for HttpPearClient<TyInner>
where
  Cx: GetRef<PearUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = ReleaseListing;
  type Error = HttpPearClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpPearClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetReleaseListQuery<Cx>) -> Self::Future {
    let url = req.context.get_ref().url_join(["r", req.package.as_str(), "allreleases.xml"]);

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .body(Full::new(Bytes::new()))
      .unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpPearClientError::Send(format!("{e:?}")))?;
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpPearClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();
      let result = ReleaseListing::from_xml(body.as_ref());
      Ok(result)
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetPackageInfoQuery<Cx>> for HttpPearClient<TyInner>
where
  Cx: GetRef<PearUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = PackageInfo;
  type Error = HttpPearClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpPearClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetPackageInfoQuery<Cx>) -> Self::Future {
    let url = req.context.get_ref().url_join(["p", req.package.as_str(), "info.xml"]);

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .body(Full::new(Bytes::new()))
      .unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpPearClientError::Send(format!("{e:?}")))?;
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpPearClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();
      let result = PackageInfo::from_xml(body.as_ref());
      Ok(result)
    })
  }
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetReleaseQuery<Cx>> for HttpPearClient<TyInner>
where
  Cx: GetRef<PearUrl>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Release;
  type Error = HttpPearClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpPearClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetReleaseQuery<Cx>) -> Self::Future {
    let url = req.context.get_ref().url_join(["r", req.package.as_str(), &format_compact!("{}.xml", req.version.as_str())]);

    let req = Request::builder()
      .method(Method::GET)
      .uri(url.as_str())
      .body(Full::new(Bytes::new()))
      .unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpPearClientError::Send(format!("{e:?}")))?;
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpPearClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();
      let result = Release::from_xml(body.as_ref());
      Ok(result)
    })
  }
}
