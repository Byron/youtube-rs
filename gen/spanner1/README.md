<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/api/README.md.mako'
DO NOT EDIT !
-->
The `google-spanner1` library allows access to all features of the *Google Spanner* service.

This documentation was generated from *Spanner* crate version *6.0.0+20240618*, where *20240618* is the exact revision of the *spanner:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v6.0.0*.

Everything else about the *Spanner* *v1* API can be found at the
[official documentation site](https://cloud.google.com/spanner/).
# Features

Handle the following *Resources* with ease from the central [hub](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/Spanner) ...

* projects
 * [*instance config operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigOperationListCall), [*instance configs create*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigCreateCall), [*instance configs delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigDeleteCall), [*instance configs get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigGetCall), [*instance configs list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigListCall), [*instance configs operations cancel*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigOperationCancelCall), [*instance configs operations delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigOperationDeleteCall), [*instance configs operations get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigOperationGetCall), [*instance configs operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigOperationListCall1), [*instance configs patch*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigPatchCall), [*instance configs ssd caches operations cancel*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigSsdCachOperationCancelCall), [*instance configs ssd caches operations delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigSsdCachOperationDeleteCall), [*instance configs ssd caches operations get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigSsdCachOperationGetCall), [*instance configs ssd caches operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceConfigSsdCachOperationListCall), [*instances backup operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupOperationListCall), [*instances backups copy*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupCopyCall), [*instances backups create*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupCreateCall), [*instances backups delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupDeleteCall), [*instances backups get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupGetCall), [*instances backups get iam policy*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupGetIamPolicyCall), [*instances backups list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupListCall), [*instances backups operations cancel*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupOperationCancelCall), [*instances backups operations delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupOperationDeleteCall), [*instances backups operations get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupOperationGetCall), [*instances backups operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupOperationListCall1), [*instances backups patch*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupPatchCall), [*instances backups set iam policy*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupSetIamPolicyCall), [*instances backups test iam permissions*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceBackupTestIamPermissionCall), [*instances create*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceCreateCall), [*instances database operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseOperationListCall), [*instances databases changequorum*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseChangequorumCall), [*instances databases create*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseCreateCall), [*instances databases database roles list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseDatabaseRoleListCall), [*instances databases database roles test iam permissions*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseDatabaseRoleTestIamPermissionCall), [*instances databases drop database*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseDropDatabaseCall), [*instances databases get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseGetCall), [*instances databases get ddl*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseGetDdlCall), [*instances databases get iam policy*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseGetIamPolicyCall), [*instances databases get scans*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseGetScanCall), [*instances databases list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseListCall), [*instances databases operations cancel*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseOperationCancelCall), [*instances databases operations delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseOperationDeleteCall), [*instances databases operations get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseOperationGetCall), [*instances databases operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseOperationListCall1), [*instances databases patch*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabasePatchCall), [*instances databases restore*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseRestoreCall), [*instances databases sessions batch create*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionBatchCreateCall), [*instances databases sessions batch write*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionBatchWriteCall), [*instances databases sessions begin transaction*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionBeginTransactionCall), [*instances databases sessions commit*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionCommitCall), [*instances databases sessions create*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionCreateCall), [*instances databases sessions delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionDeleteCall), [*instances databases sessions execute batch dml*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionExecuteBatchDmlCall), [*instances databases sessions execute sql*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionExecuteSqlCall), [*instances databases sessions execute streaming sql*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionExecuteStreamingSqlCall), [*instances databases sessions get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionGetCall), [*instances databases sessions list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionListCall), [*instances databases sessions partition query*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionPartitionQueryCall), [*instances databases sessions partition read*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionPartitionReadCall), [*instances databases sessions read*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionReadCall), [*instances databases sessions rollback*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionRollbackCall), [*instances databases sessions streaming read*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSessionStreamingReadCall), [*instances databases set iam policy*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseSetIamPolicyCall), [*instances databases test iam permissions*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseTestIamPermissionCall), [*instances databases update ddl*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDatabaseUpdateDdlCall), [*instances delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceDeleteCall), [*instances get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceGetCall), [*instances get iam policy*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceGetIamPolicyCall), [*instances instance partition operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionOperationListCall), [*instances instance partitions create*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionCreateCall), [*instances instance partitions delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionDeleteCall), [*instances instance partitions get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionGetCall), [*instances instance partitions list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionListCall), [*instances instance partitions operations cancel*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionOperationCancelCall), [*instances instance partitions operations delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionOperationDeleteCall), [*instances instance partitions operations get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionOperationGetCall), [*instances instance partitions operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionOperationListCall1), [*instances instance partitions patch*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceInstancePartitionPatchCall), [*instances list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceListCall), [*instances move*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceMoveCall), [*instances operations cancel*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceOperationCancelCall), [*instances operations delete*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceOperationDeleteCall), [*instances operations get*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceOperationGetCall), [*instances operations list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceOperationListCall), [*instances patch*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstancePatchCall), [*instances set iam policy*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceSetIamPolicyCall) and [*instances test iam permissions*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ProjectInstanceTestIamPermissionCall)
* [scans](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::Scan)
 * [*list*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/api::ScanListCall)




# Structure of this Library

The API is structured into the following primary items:

* **[Hub](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/Spanner)**
    * a central object to maintain state and allow accessing all *Activities*
    * creates [*Method Builders*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::MethodsBuilder) which in turn
      allow access to individual [*Call Builders*](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::CallBuilder)
* **[Resources](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Resource)**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **[Parts](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Part)**
        * a collection of properties
        * never directly used in *Activities*
* **[Activities](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::CallBuilder)**
    * operations to apply to *Resources*

All *structures* are marked with applicable traits to further categorize them and ease browsing.

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).doit().await
```

Or specifically ...

```ignore
let r = hub.projects().instance_configs_operations_get(...).doit().await
let r = hub.projects().instance_configs_ssd_caches_operations_get(...).doit().await
let r = hub.projects().instance_configs_create(...).doit().await
let r = hub.projects().instance_configs_patch(...).doit().await
let r = hub.projects().instances_backups_operations_get(...).doit().await
let r = hub.projects().instances_backups_copy(...).doit().await
let r = hub.projects().instances_backups_create(...).doit().await
let r = hub.projects().instances_databases_operations_get(...).doit().await
let r = hub.projects().instances_databases_changequorum(...).doit().await
let r = hub.projects().instances_databases_create(...).doit().await
let r = hub.projects().instances_databases_patch(...).doit().await
let r = hub.projects().instances_databases_restore(...).doit().await
let r = hub.projects().instances_databases_update_ddl(...).doit().await
let r = hub.projects().instances_instance_partitions_operations_get(...).doit().await
let r = hub.projects().instances_instance_partitions_create(...).doit().await
let r = hub.projects().instances_instance_partitions_patch(...).doit().await
let r = hub.projects().instances_operations_get(...).doit().await
let r = hub.projects().instances_create(...).doit().await
let r = hub.projects().instances_move(...).doit().await
let r = hub.projects().instances_patch(...).doit().await
```

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities`
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `doit()` method performs the actual communication with the server and returns the respective result.

# Usage

## Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
google-spanner1 = "*"
serde = "1"
serde_json = "1"
```

## A complete example

```Rust
extern crate hyper;
extern crate hyper_rustls;
extern crate google_spanner1 as spanner1;
use spanner1::api::Backup;
use spanner1::{Result, Error};
use spanner1::{Spanner, FieldMask, hyper_rustls, hyper_util, yup_oauth2};

// Get an ApplicationSecret instance by some means. It contains the `client_id` and
// `client_secret`, among other things.
let secret: yup_oauth2::ApplicationSecret = Default::default();
// Instantiate the authenticator. It will choose a suitable authentication flow for you,
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
    secret,
    yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
).build().await.unwrap();

let client = hyper_util::client::legacy::Client::builder(
    hyper_util::rt::TokioExecutor::new()
)
.build(
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_or_http()
        .enable_http1()
        .build()
);
let mut hub = Spanner::new(client, auth);
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// Values shown here are possibly random and not representative !
let mut req = Backup::default();

// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `doit()`.
// Values shown here are possibly random and not representative !
let result = hub.projects().instances_backups_create(req, "parent")
             .add_encryption_config_kms_key_names("magna")
             .encryption_config_kms_key_name("no")
             .encryption_config_encryption_type("ipsum")
             .backup_id("voluptua.")
             .doit().await;

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}

```
## Handling Errors

All errors produced by the system are provided either as [Result](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Result) enumeration as return value of
the doit() methods, or handed as possibly intermediate results to either the
[Hub Delegate](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Delegate), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This
makes the system potentially resilient to all kinds of errors.

## Uploads and Downloads
If a method supports downloads, the response body, which is part of the [Result](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Result), should be
read by you to obtain the media.
If such a method also supports a [Response Result](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::ResponseResult), it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `.param("alt", "media")`.

Methods supporting uploads can do so using up to 2 different protocols:
*simple* and *resumable*. The distinctiveness of each is represented by customized
`doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.

## Customization and Callbacks

You may alter the way an `doit()` method is called by providing a [delegate](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Delegate) to the
[Method Builder](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::CallBuilder) before making the final `doit()` call.
Respective methods will be called to provide progress information, as well as determine whether the system should
retry on failure.

The [delegate trait](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Delegate) is default-implemented, allowing you to customize it with minimal effort.

## Optional Parts in Server-Requests

All structures provided by this library are made to be [encodable](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::RequestValue) and
[decodable](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::ResponseResult) via *json*. Optionals are used to indicate that partial requests are responses
are valid.
Most optionals are are considered [Parts](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::Part) which are identifiable by name, which will be sent to
the server to indicate either the set parts of the request or the desired parts in the response.

## Builder Arguments

Using [method builders](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::CallBuilder), you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* [request values](https://docs.rs/google-spanner1/6.0.0+20240618/google_spanner1/common::RequestValue) are moved

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client

## Cargo Features

* `utoipa` - Add support for [utoipa](https://crates.io/crates/utoipa) and derive `utoipa::ToSchema` on all
the types. You'll have to import and register the required types in `#[openapi(schemas(...))]`, otherwise the
generated `openapi` spec would be invalid.


# License
The **spanner1** library was generated by Sebastian Thiel, and is placed
under the *MIT* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: https://github.com/Byron/google-apis-rsblob/main/LICENSE.md

