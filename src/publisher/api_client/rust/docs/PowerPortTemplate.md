# PowerPortTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**device_type** | Option<[**crate::models::NestedDeviceType**](NestedDeviceType.md)> |  | [optional]
**module_type** | Option<[**crate::models::NestedModuleType**](NestedModuleType.md)> |  | [optional]
**name** | **String** |  {module} is accepted as a substitution for the module bay position when attached to a module type.  | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | Option<[**crate::models::Type5**](Type_5.md)> |  | [optional]
**maximum_draw** | Option<**i32**> | Maximum power draw (watts) | [optional]
**allocated_draw** | Option<**i32**> | Allocated power draw (watts) | [optional]
**description** | Option<**String**> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


