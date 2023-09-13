# RackReservation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**rack** | Option<[**crate::models::NestedRack**](NestedRack.md)> |  | 
**units** | **Vec<i32>** |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**user** | [**crate::models::NestedUser**](NestedUser.md) |  | 
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**description** | **String** |  | 
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


