# WritableDeviceWithConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | 
**device_type** | **i32** |  | 
**device_role** | **i32** |  | 
**tenant** | Option<**i32**> |  | 
**platform** | Option<**i32**> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**site** | **i32** |  | 
**location** | Option<**i32**> |  | [optional]
**rack** | Option<**i32**> |  | 
**position** | Option<**f32**> |  | [optional]
**face** | **String** |  | 
**parent_device** | Option<[**crate::models::NestedDevice**](NestedDevice.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**airflow** | Option<**String**> |  | [optional]
**primary_ip** | Option<**String**> |  | [optional][readonly]
**primary_ip4** | Option<**i32**> |  | [optional]
**primary_ip6** | Option<**i32**> |  | [optional]
**cluster** | Option<**i32**> |  | [optional]
**virtual_chassis** | Option<**i32**> |  | 
**vc_position** | Option<**i32**> |  | [optional]
**vc_priority** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**config_context** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


