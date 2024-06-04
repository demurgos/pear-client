use pear_client::client::http::HttpPearClient;
use pear_client::context::{Context, PearUrl};
use pear_client::query::get_package_list::GetPackageListQuery;
use pear_client::tower_service::Service;
use pear_client::url::Url;
use hyper_tls::HttpsConnector;
use pear_client::common::package::PackageListing;
use pear_client::common::release::{Release, ReleaseListing};
use pear_client::compact_str::CompactString;
use pear_client::query::get_release::GetReleaseQuery;
use pear_client::query::get_release_list::GetReleaseListQuery;


#[tokio::main]
async fn main() {
  let connector = HttpsConnector::new();
  let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);
  let mut client = HttpPearClient::new(client);
  {
    let context = Context::new().set_pear_url(PearUrl(Url::parse("https://pecl.php.net/").unwrap()));
    let query = GetPackageListQuery::<_>::new().set_context(context);
    let res: PackageListing = client.call(&query).await.unwrap();
    dbg!(&res.items[..10]);
  }
  {
    let context = Context::new().set_pear_url(PearUrl(Url::parse("https://pecl.php.net/").unwrap()));
    let query = GetReleaseListQuery::<_>::new(CompactString::new("protobuf")).set_context(context);
    let res: ReleaseListing = client.call(&query).await.unwrap();
    dbg!(&res.items[..10]);
  }
  {
    let context = Context::new().set_pear_url(PearUrl(Url::parse("https://pecl.php.net/").unwrap()));
    let query = GetReleaseQuery::<_>::new(CompactString::new("protobuf"), CompactString::new("4.27.0")).set_context(context);
    let res: Release = client.call(&query).await.unwrap();
    dbg!(&res);
  }
}
