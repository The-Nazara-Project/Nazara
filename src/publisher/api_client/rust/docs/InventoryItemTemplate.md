# InventoryItemTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**device_type** | [**crate::models::NestedDeviceType**](NestedDeviceType.md) |  | 
**parent** | Option<**i32**> |  | [optional]
**name** | **String** |  {module} is accepted as a substitution for the module bay position when attached to a module type.  | 
**label** | Option<**String**> | Physical label | [optional]
**role** | Option<[**crate::models::NestedInventoryItemRole**](NestedInventoryItemRole.md)> |  | [optional]
**manufacturer** | Option<[**crate::models::NestedManufacturer**](NestedManufacturer.md)> |  | [optional]
**part_id** | Option<**String**> | Manufacturer-assigned part identifier | [optional]
**description** | Option<**String**> |  | [optional]
**component_type** | Option<**String**> |  | [optional]
**component_id** | Option<**i32**> |  | [optional]
**component** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**_depth** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


