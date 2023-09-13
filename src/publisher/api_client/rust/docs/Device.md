# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | 
**device_type** | [**crate::models::NestedDeviceType**](NestedDeviceType.md) |  | 
**device_role** | [**crate::models::NestedDeviceRole**](NestedDeviceRole.md) |  | 
**tenant** | Option<[**crate::models::NestedTenant**](NestedTenant.md)> |  | [optional]
**platform** | Option<[**crate::models::NestedPlatform**](NestedPlatform.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**site** | Option<[**crate::models::NestedSite**](NestedSite.md)> |  | 
**location** | Option<[**crate::models::NestedLocation**](NestedLocation.md)> |  | [optional]
**rack** | Option<[**crate::models::NestedRack**](NestedRack.md)> |  | [optional]
**position** | Option<**f32**> |  | [optional]
**face** | Option<[**crate::models::Face**](Face.md)> |  | [optional]
**parent_device** | Option<[**crate::models::NestedDevice**](NestedDevice.md)> |  | [optional]
**status** | Option<[**crate::models::Status2**](Status_2.md)> |  | [optional]
**airflow** | Option<[**crate::models::Airflow**](Airflow.md)> |  | [optional]
**primary_ip** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::NestedIpAddress**](NestedIPAddress.md)> |  | [optional]
**cluster** | Option<[**crate::models::NestedCluster**](NestedCluster.md)> |  | [optional]
**virtual_chassis** | Option<[**crate::models::NestedVirtualChassis**](NestedVirtualChassis.md)> |  | [optional]
**vc_position** | Option<**i32**> |  | [optional]
**vc_priority** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


