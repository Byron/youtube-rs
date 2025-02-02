#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

// ########
// HUB ###
// ######

/// Central instance to access all Discovery related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_discovery1 as discovery1;
/// use discovery1::{Result, Error};
/// # async fn dox() {
/// use discovery1::{Discovery, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Discovery::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.apis().get_rest("api", "version")
///              .doit().await;
///
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Discovery<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for Discovery<C> {}

impl<'a, C> Discovery<C> {
    pub fn new<A: 'static + common::GetToken>(client: common::Client<C>, auth: A) -> Discovery<C> {
        Discovery {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://www.googleapis.com/discovery/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn apis(&'a self) -> ApiMethods<'a, C> {
        ApiMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/discovery/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list apis](ApiListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DirectoryList {
    /// Indicate the version of the Discovery API used to generate this doc.
    #[serde(rename = "discoveryVersion")]
    pub discovery_version: Option<String>,
    /// The individual directory entries. One entry per api/version pair.
    pub items: Option<Vec<DirectoryListItems>>,
    /// The kind for this response.
    pub kind: Option<String>,
}

impl common::ResponseResult for DirectoryList {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonSchema {
    /// A reference to another schema. The value of this property is the "id" of another schema.
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    /// If this is a schema for an object, this property is the schema for any additional properties with dynamic keys on this object.
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Option<Box<JsonSchema>>>,
    /// Additional information about this property.
    pub annotations: Option<JsonSchemaAnnotations>,
    /// The default value of this property (if one exists).
    pub default: Option<String>,
    /// Whether the parameter is deprecated.
    pub deprecated: Option<bool>,
    /// A description of this object.
    pub description: Option<String>,
    /// Values this parameter may take (if it is an enum).
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    /// The deprecation status for the enums. Each position maps to the corresponding value in the "enum" array.
    #[serde(rename = "enumDeprecated")]
    pub enum_deprecated: Option<Vec<bool>>,
    /// The descriptions for the enums. Each position maps to the corresponding value in the "enum" array.
    #[serde(rename = "enumDescriptions")]
    pub enum_descriptions: Option<Vec<String>>,
    /// An additional regular expression or key that helps constrain the value. For more details see: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.23
    pub format: Option<String>,
    /// Unique identifier for this schema.
    pub id: Option<String>,
    /// If this is a schema for an array, this property is the schema for each element in the array.
    pub items: Option<Option<Box<JsonSchema>>>,
    /// Whether this parameter goes in the query or the path for REST requests.
    pub location: Option<String>,
    /// The maximum value of this parameter.
    pub maximum: Option<String>,
    /// The minimum value of this parameter.
    pub minimum: Option<String>,
    /// The regular expression this parameter must conform to. Uses Java 6 regex format: http://docs.oracle.com/javase/6/docs/api/java/util/regex/Pattern.html
    pub pattern: Option<String>,
    /// If this is a schema for an object, list the schema for each property of this object.
    pub properties: Option<HashMap<String, JsonSchema>>,
    /// The value is read-only, generated by the service. The value cannot be modified by the client. If the value is included in a POST, PUT, or PATCH request, it is ignored by the service.
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    /// Whether this parameter may appear multiple times.
    pub repeated: Option<bool>,
    /// Whether the parameter is required.
    pub required: Option<bool>,
    /// The value type for this schema. A list of values can be found here: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.1
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names.
    pub variant: Option<JsonSchemaVariant>,
}

impl common::Part for JsonSchema {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get rest apis](ApiGetRestCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestDescription {
    /// Authentication information.
    pub auth: Option<RestDescriptionAuth>,
    /// [DEPRECATED] The base path for REST requests.
    #[serde(rename = "basePath")]
    pub base_path: Option<String>,
    /// [DEPRECATED] The base URL for REST requests.
    #[serde(rename = "baseUrl")]
    pub base_url: Option<String>,
    /// The path for REST batch requests.
    #[serde(rename = "batchPath")]
    pub batch_path: Option<String>,
    /// Indicates how the API name should be capitalized and split into various parts. Useful for generating pretty class names.
    #[serde(rename = "canonicalName")]
    pub canonical_name: Option<String>,
    /// The description of this API.
    pub description: Option<String>,
    /// Indicate the version of the Discovery API used to generate this doc.
    #[serde(rename = "discoveryVersion")]
    pub discovery_version: Option<String>,
    /// A link to human readable documentation for the API.
    #[serde(rename = "documentationLink")]
    pub documentation_link: Option<String>,
    /// A list of location-based endpoint objects for this API. Each object contains the endpoint URL, location, description and deprecation status.
    pub endpoints: Option<Vec<RestDescriptionEndpoints>>,
    /// The ETag for this response.
    pub etag: Option<String>,
    /// Enable exponential backoff for suitable methods in the generated clients.
    #[serde(rename = "exponentialBackoffDefault")]
    pub exponential_backoff_default: Option<bool>,
    /// A list of supported features for this API.
    pub features: Option<Vec<String>>,
    /// Links to 16x16 and 32x32 icons representing the API.
    pub icons: Option<RestDescriptionIcons>,
    /// The ID of this API.
    pub id: Option<String>,
    /// The kind for this response.
    pub kind: Option<String>,
    /// Labels for the status of this API, such as labs or deprecated.
    pub labels: Option<Vec<String>>,
    /// API-level methods for this API.
    pub methods: Option<HashMap<String, RestMethod>>,
    /// The name of this API.
    pub name: Option<String>,
    /// The domain of the owner of this API. Together with the ownerName and a packagePath values, this can be used to generate a library for this API which would have a unique fully qualified name.
    #[serde(rename = "ownerDomain")]
    pub owner_domain: Option<String>,
    /// The name of the owner of this API. See ownerDomain.
    #[serde(rename = "ownerName")]
    pub owner_name: Option<String>,
    /// The package of the owner of this API. See ownerDomain.
    #[serde(rename = "packagePath")]
    pub package_path: Option<String>,
    /// Common parameters that apply across all apis.
    pub parameters: Option<HashMap<String, JsonSchema>>,
    /// The protocol described by this document.
    pub protocol: Option<String>,
    /// The resources in this API.
    pub resources: Option<HashMap<String, RestResource>>,
    /// The version of this API.
    pub revision: Option<String>,
    /// The root URL under which all API services live.
    #[serde(rename = "rootUrl")]
    pub root_url: Option<String>,
    /// The schemas for this API.
    pub schemas: Option<HashMap<String, JsonSchema>>,
    /// The base path for all REST requests.
    #[serde(rename = "servicePath")]
    pub service_path: Option<String>,
    /// The title of this API.
    pub title: Option<String>,
    /// The version of this API.
    pub version: Option<String>,
    /// no description provided
    pub version_module: Option<bool>,
}

impl common::ResponseResult for RestDescription {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestMethod {
    /// The API Version of this method, as passed in via the `X-Goog-Api-Version` header or `$apiVersion` query parameter.
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Whether this method is deprecated.
    pub deprecated: Option<bool>,
    /// Description of this method.
    pub description: Option<String>,
    /// Whether this method requires an ETag to be specified. The ETag is sent as an HTTP If-Match or If-None-Match header.
    #[serde(rename = "etagRequired")]
    pub etag_required: Option<bool>,
    /// The URI path of this REST method in (RFC 6570) format without level 2 features ({+var}). Supplementary to the path property.
    #[serde(rename = "flatPath")]
    pub flat_path: Option<String>,
    /// HTTP method used by this method.
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    /// A unique ID for this method. This property can be used to match methods between different versions of Discovery.
    pub id: Option<String>,
    /// Media upload parameters.
    #[serde(rename = "mediaUpload")]
    pub media_upload: Option<RestMethodMediaUpload>,
    /// Ordered list of required parameters, serves as a hint to clients on how to structure their method signatures. The array is ordered such that the "most-significant" parameter appears first.
    #[serde(rename = "parameterOrder")]
    pub parameter_order: Option<Vec<String>>,
    /// Details for all parameters in this method.
    pub parameters: Option<HashMap<String, JsonSchema>>,
    /// The URI path of this REST method. Should be used in conjunction with the basePath property at the api-level.
    pub path: Option<String>,
    /// The schema for the request.
    pub request: Option<RestMethodRequest>,
    /// The schema for the response.
    pub response: Option<RestMethodResponse>,
    /// OAuth 2.0 scopes applicable to this method.
    pub scopes: Option<Vec<String>>,
    /// Whether this method supports media downloads.
    #[serde(rename = "supportsMediaDownload")]
    pub supports_media_download: Option<bool>,
    /// Whether this method supports media uploads.
    #[serde(rename = "supportsMediaUpload")]
    pub supports_media_upload: Option<bool>,
    /// Whether this method supports subscriptions.
    #[serde(rename = "supportsSubscription")]
    pub supports_subscription: Option<bool>,
    /// Indicates that downloads from this method should use the download service URL (i.e. "/download"). Only applies if the method supports media download.
    #[serde(rename = "useMediaDownloadService")]
    pub use_media_download_service: Option<bool>,
}

impl common::Part for RestMethod {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestResource {
    /// Whether this resource is deprecated.
    pub deprecated: Option<bool>,
    /// Methods on this resource.
    pub methods: Option<HashMap<String, RestMethod>>,
    /// Sub-resources on this resource.
    pub resources: Option<HashMap<String, RestResource>>,
}

impl common::Part for RestResource {}

/// The individual directory entries. One entry per api/version pair.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DirectoryListItems {
    /// The description of this API.
    pub description: Option<String>,
    /// A link to the discovery document.
    #[serde(rename = "discoveryLink")]
    pub discovery_link: Option<String>,
    /// The URL for the discovery REST document.
    #[serde(rename = "discoveryRestUrl")]
    pub discovery_rest_url: Option<String>,
    /// A link to human readable documentation for the API.
    #[serde(rename = "documentationLink")]
    pub documentation_link: Option<String>,
    /// Links to 16x16 and 32x32 icons representing the API.
    pub icons: Option<DirectoryListItemsIcons>,
    /// The id of this API.
    pub id: Option<String>,
    /// The kind for this response.
    pub kind: Option<String>,
    /// Labels for the status of this API, such as labs or deprecated.
    pub labels: Option<Vec<String>>,
    /// The name of the API.
    pub name: Option<String>,
    /// True if this version is the preferred version to use.
    pub preferred: Option<bool>,
    /// The title of this API.
    pub title: Option<String>,
    /// The version of the API.
    pub version: Option<String>,
}

impl common::NestedType for DirectoryListItems {}
impl common::Part for DirectoryListItems {}

/// Links to 16x16 and 32x32 icons representing the API.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DirectoryListItemsIcons {
    /// The URL of the 16x16 icon.
    pub x16: Option<String>,
    /// The URL of the 32x32 icon.
    pub x32: Option<String>,
}

impl common::NestedType for DirectoryListItemsIcons {}
impl common::Part for DirectoryListItemsIcons {}

/// Additional information about this property.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonSchemaAnnotations {
    /// A list of methods for which this property is required on requests.
    pub required: Option<Vec<String>>,
}

impl common::NestedType for JsonSchemaAnnotations {}
impl common::Part for JsonSchemaAnnotations {}

/// In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonSchemaVariant {
    /// The name of the type discriminant property.
    pub discriminant: Option<String>,
    /// The map of discriminant value to schema to use for parsing..
    pub map: Option<Vec<JsonSchemaVariantMap>>,
}

impl common::NestedType for JsonSchemaVariant {}
impl common::Part for JsonSchemaVariant {}

/// The map of discriminant value to schema to use for parsing..
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonSchemaVariantMap {
    /// no description provided
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    /// no description provided
    pub type_value: Option<String>,
}

impl common::NestedType for JsonSchemaVariantMap {}
impl common::Part for JsonSchemaVariantMap {}

/// Authentication information.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestDescriptionAuth {
    /// OAuth 2.0 authentication information.
    pub oauth2: Option<RestDescriptionAuthOauth2>,
}

impl common::NestedType for RestDescriptionAuth {}
impl common::Part for RestDescriptionAuth {}

/// OAuth 2.0 authentication information.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestDescriptionAuthOauth2 {
    /// Available OAuth 2.0 scopes.
    pub scopes: Option<HashMap<String, RestDescriptionAuthOauth2Scopes>>,
}

impl common::NestedType for RestDescriptionAuthOauth2 {}
impl common::Part for RestDescriptionAuthOauth2 {}

/// The scope value.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestDescriptionAuthOauth2Scopes {
    /// Description of scope.
    pub description: Option<String>,
}

impl common::NestedType for RestDescriptionAuthOauth2Scopes {}
impl common::Part for RestDescriptionAuthOauth2Scopes {}

/// A single endpoint object
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestDescriptionEndpoints {
    /// Whether this endpoint is deprecated
    pub deprecated: Option<bool>,
    /// A string describing the host designated by the URL
    pub description: Option<String>,
    /// The URL of the endpoint target host
    #[serde(rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    /// The location of the endpoint
    pub location: Option<String>,
}

impl common::NestedType for RestDescriptionEndpoints {}
impl common::Part for RestDescriptionEndpoints {}

/// Links to 16x16 and 32x32 icons representing the API.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestDescriptionIcons {
    /// The URL of the 16x16 icon.
    pub x16: Option<String>,
    /// The URL of the 32x32 icon.
    pub x32: Option<String>,
}

impl common::NestedType for RestDescriptionIcons {}
impl common::Part for RestDescriptionIcons {}

/// Media upload parameters.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestMethodMediaUpload {
    /// MIME Media Ranges for acceptable media uploads to this method.
    pub accept: Option<Vec<String>>,
    /// Maximum size of a media upload, such as "1MB", "2GB" or "3TB".
    #[serde(rename = "maxSize")]
    pub max_size: Option<String>,
    /// Supported upload protocols.
    pub protocols: Option<RestMethodMediaUploadProtocols>,
}

impl common::NestedType for RestMethodMediaUpload {}
impl common::Part for RestMethodMediaUpload {}

/// Supported upload protocols.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestMethodMediaUploadProtocols {
    /// Supports the Resumable Media Upload protocol.
    pub resumable: Option<RestMethodMediaUploadProtocolsResumable>,
    /// Supports uploading as a single HTTP request.
    pub simple: Option<RestMethodMediaUploadProtocolsSimple>,
}

impl common::NestedType for RestMethodMediaUploadProtocols {}
impl common::Part for RestMethodMediaUploadProtocols {}

/// Supports the Resumable Media Upload protocol.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestMethodMediaUploadProtocolsResumable {
    /// True if this endpoint supports uploading multipart media.
    pub multipart: Option<bool>,
    /// The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level.
    pub path: Option<String>,
}

impl common::NestedType for RestMethodMediaUploadProtocolsResumable {}
impl common::Part for RestMethodMediaUploadProtocolsResumable {}

/// Supports uploading as a single HTTP request.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestMethodMediaUploadProtocolsSimple {
    /// True if this endpoint supports upload multipart media.
    pub multipart: Option<bool>,
    /// The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level.
    pub path: Option<String>,
}

impl common::NestedType for RestMethodMediaUploadProtocolsSimple {}
impl common::Part for RestMethodMediaUploadProtocolsSimple {}

/// The schema for the request.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestMethodRequest {
    /// Schema ID for the request schema.
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    /// parameter name.
    #[serde(rename = "parameterName")]
    pub parameter_name: Option<String>,
}

impl common::NestedType for RestMethodRequest {}
impl common::Part for RestMethodRequest {}

/// The schema for the response.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RestMethodResponse {
    /// Schema ID for the response schema.
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
}

impl common::NestedType for RestMethodResponse {}
impl common::Part for RestMethodResponse {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *api* resources.
/// It is not used directly, but through the [`Discovery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_discovery1 as discovery1;
///
/// # async fn dox() {
/// use discovery1::{Discovery, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Discovery::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_rest(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.apis();
/// # }
/// ```
pub struct ApiMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Discovery<C>,
}

impl<'a, C> common::MethodsBuilder for ApiMethods<'a, C> {}

impl<'a, C> ApiMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the description of a particular version of an api.
    ///
    /// # Arguments
    ///
    /// * `api` - The name of the API.
    /// * `version` - The version of the API.
    pub fn get_rest(&self, api: &str, version: &str) -> ApiGetRestCall<'a, C> {
        ApiGetRestCall {
            hub: self.hub,
            _api: api.to_string(),
            _version: version.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the list of APIs supported at this endpoint.
    pub fn list(&self) -> ApiListCall<'a, C> {
        ApiListCall {
            hub: self.hub,
            _preferred: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Retrieve the description of a particular version of an api.
///
/// A builder for the *getRest* method supported by a *api* resource.
/// It is not used directly, but through a [`ApiMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_discovery1 as discovery1;
/// # async fn dox() {
/// # use discovery1::{Discovery, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Discovery::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.apis().get_rest("api", "version")
///              .doit().await;
/// # }
/// ```
pub struct ApiGetRestCall<'a, C>
where
    C: 'a,
{
    hub: &'a Discovery<C>,
    _api: String,
    _version: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ApiGetRestCall<'a, C> {}

impl<'a, C> ApiGetRestCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, RestDescription)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "discovery.apis.getRest",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "api", "version"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("api", self._api);
        params.push("version", self._version);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "apis/{api}/{version}/rest";

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{api}", "api"), ("{version}", "version")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["version", "api"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The name of the API.
    ///
    /// Sets the *api* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn api(mut self, new_value: &str) -> ApiGetRestCall<'a, C> {
        self._api = new_value.to_string();
        self
    }
    /// The version of the API.
    ///
    /// Sets the *version* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn version(mut self, new_value: &str) -> ApiGetRestCall<'a, C> {
        self._version = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ApiGetRestCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ApiGetRestCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Retrieve the list of APIs supported at this endpoint.
///
/// A builder for the *list* method supported by a *api* resource.
/// It is not used directly, but through a [`ApiMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_discovery1 as discovery1;
/// # async fn dox() {
/// # use discovery1::{Discovery, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Discovery::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.apis().list()
///              .preferred(true)
///              .name("amet.")
///              .doit().await;
/// # }
/// ```
pub struct ApiListCall<'a, C>
where
    C: 'a,
{
    hub: &'a Discovery<C>,
    _preferred: Option<bool>,
    _name: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for ApiListCall<'a, C> {}

impl<'a, C> ApiListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, DirectoryList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "discovery.apis.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "preferred", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._preferred.as_ref() {
            params.push("preferred", value.to_string());
        }
        if let Some(value) = self._name.as_ref() {
            params.push("name", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "apis";

        let url = params.parse_with_url(&url);

        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Return only the preferred version of an API.
    ///
    /// Sets the *preferred* query property to the given value.
    pub fn preferred(mut self, new_value: bool) -> ApiListCall<'a, C> {
        self._preferred = Some(new_value);
        self
    }
    /// Only include APIs with the given name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> ApiListCall<'a, C> {
        self._name = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ApiListCall<'a, C> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ApiListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}
