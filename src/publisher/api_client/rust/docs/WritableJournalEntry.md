# WritableJournalEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**assigned_object_type** | **String** |  | 
**assigned_object_id** | **i32** |  | 
**assigned_object** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**created_by** | Option<**i32**> |  | [optional]
**kind** | Option<**String**> |  | [optional]
**comments** | **String** |  | 
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


