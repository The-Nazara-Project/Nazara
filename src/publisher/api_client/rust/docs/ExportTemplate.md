# ExportTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**content_type** | **String** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**template_code** | **String** | Jinja2 template code. The list of objects being exported is passed as a context variable named <code>queryset</code>. | 
**mime_type** | Option<**String**> | Defaults to <code>text/plain</code> | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**as_attachment** | Option<**bool**> | Download file as attachment | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


