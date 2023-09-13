# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**device** | Option<[**crate::models::NestedDevice**](NestedDevice.md)> |  | [optional]
**virtual_machine** | Option<[**crate::models::NestedVirtualMachine**](NestedVirtualMachine.md)> |  | [optional]
**name** | **String** |  | 
**ports** | **Vec<i32>** |  | 
**protocol** | Option<[**crate::models::Protocol**](Protocol.md)> |  | [optional]
**ipaddresses** | Option<[**Vec<crate::models::NestedIpAddress>**](NestedIPAddress.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


