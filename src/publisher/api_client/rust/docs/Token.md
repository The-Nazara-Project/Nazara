# Token

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**user** | [**crate::models::NestedUser**](NestedUser.md) |  | 
**created** | Option<**String**> |  | [optional][readonly]
**expires** | Option<**String**> |  | [optional]
**last_used** | Option<**String**> |  | [optional]
**key** | Option<**String**> |  | [optional]
**write_enabled** | Option<**bool**> | Permit create/update/delete operations using this key | [optional]
**description** | Option<**String**> |  | [optional]
**allowed_ips** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


