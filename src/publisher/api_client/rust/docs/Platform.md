# Platform

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**slug** | **String** |  | 
**manufacturer** | Option<[**crate::models::NestedManufacturer**](NestedManufacturer.md)> |  | [optional]
**napalm_driver** | Option<**String**> | The name of the NAPALM driver to use when interacting with devices | [optional]
**napalm_args** | Option<[**serde_json::Value**](.md)> | Additional arguments to pass when initiating the NAPALM driver (JSON format) | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**virtualmachine_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


