# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**content_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**type_create** | Option<**bool**> | Call this webhook when a matching object is created. | [optional][default to false]
**type_update** | Option<**bool**> | Call this webhook when a matching object is updated. | [optional][default to false]
**type_delete** | Option<**bool**> | Call this webhook when a matching object is deleted. | [optional][default to false]
**payload_url** | **String** | This URL will be called using the HTTP method defined when the webhook is called. Jinja2 template processing is supported with the same context as the request body. | 
**enabled** | Option<**bool**> |  | [optional]
**http_method** | Option<**String**> |  | [optional]
**http_content_type** | Option<**String**> | The complete list of official content types is available <a href=\"https://www.iana.org/assignments/media-types/media-types.xhtml\">here</a>. | [optional]
**additional_headers** | Option<**String**> | User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below). | [optional]
**body_template** | Option<**String**> | Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>. | [optional]
**secret** | Option<**String**> | When provided, the request will include a 'X-Hook-Signature' header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request. | [optional]
**conditions** | Option<[**serde_json::Value**](.md)> | A set of conditions which determine whether the webhook will be generated. | [optional]
**ssl_verification** | Option<**bool**> | Enable SSL certificate verification. Disable with caution! | [optional]
**ca_file_path** | Option<**String**> | The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults. | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


