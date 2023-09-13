# Cluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**r#type** | [**crate::models::NestedClusterType**](NestedClusterType.md) |  | 
**group** | Option<[**crate::models::NestedClusterGroup**](NestedClusterGroup.md)> |  | [optional]
**status** | Option<[**crate::models::Status10**](Status_10.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**virtualmachine_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


