# Module

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**device** | [**crate::models::NestedDevice**](NestedDevice.md) |  | 
**module_bay** | [**crate::models::NestedModuleBay**](NestedModuleBay.md) |  | 
**module_type** | Option<[**crate::models::NestedModuleType**](NestedModuleType.md)> |  | 
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


