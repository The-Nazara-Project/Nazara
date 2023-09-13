# ConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**weight** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**regions** | Option<[**Vec<crate::models::NestedRegion>**](NestedRegion.md)> |  | [optional]
**site_groups** | Option<[**Vec<crate::models::NestedSiteGroup>**](NestedSiteGroup.md)> |  | [optional]
**sites** | Option<[**Vec<crate::models::NestedSite>**](NestedSite.md)> |  | [optional]
**locations** | Option<[**Vec<crate::models::NestedLocation>**](NestedLocation.md)> |  | [optional]
**device_types** | Option<[**Vec<crate::models::NestedDeviceType>**](NestedDeviceType.md)> |  | [optional]
**roles** | Option<[**Vec<crate::models::NestedDeviceRole>**](NestedDeviceRole.md)> |  | [optional]
**platforms** | Option<[**Vec<crate::models::NestedPlatform>**](NestedPlatform.md)> |  | [optional]
**cluster_types** | Option<[**Vec<crate::models::NestedClusterType>**](NestedClusterType.md)> |  | [optional]
**cluster_groups** | Option<[**Vec<crate::models::NestedClusterGroup>**](NestedClusterGroup.md)> |  | [optional]
**clusters** | Option<[**Vec<crate::models::NestedCluster>**](NestedCluster.md)> |  | [optional]
**tenant_groups** | Option<[**Vec<crate::models::NestedTenantGroup>**](NestedTenantGroup.md)> |  | [optional]
**tenants** | Option<[**Vec<crate::models::NestedTenant>**](NestedTenant.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**data** | [**serde_json::Value**](.md) |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


