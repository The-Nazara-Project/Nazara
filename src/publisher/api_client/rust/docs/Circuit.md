# Circuit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**cid** | **String** |  | 
**provider** | [**crate::models::NestedProvider**](NestedProvider.md) |  | 
**r#type** | [**crate::models::NestedCircuitType**](NestedCircuitType.md) |  | 
**status** | Option<[**crate::models::Status**](Status.md)> |  | [optional]
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**install_date** | Option<[**String**](string.md)> |  | [optional]
**termination_date** | Option<[**String**](string.md)> |  | [optional]
**commit_rate** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**termination_a** | Option<[**crate::models::CircuitCircuitTermination**](CircuitCircuitTermination.md)> |  | [optional]
**termination_z** | Option<[**crate::models::CircuitCircuitTermination**](CircuitCircuitTermination.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


