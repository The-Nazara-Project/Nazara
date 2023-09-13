# WritableVirtualMachineWithConfigContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**status** | Option<**String**> |  | [optional]
**site** | Option<**i32**> |  | [optional]
**cluster** | Option<**i32**> |  | [optional]
**device** | Option<**i32**> |  | [optional]
**role** | Option<**i32**> |  | [optional]
**tenant** | Option<**i32**> |  | [optional]
**platform** | Option<**i32**> |  | [optional]
**primary_ip** | Option<**String**> |  | [optional][readonly]
**primary_ip4** | Option<**i32**> |  | [optional]
**primary_ip6** | Option<**i32**> |  | [optional]
**vcpus** | Option<**f32**> |  | [optional]
**memory** | Option<**i32**> |  | [optional]
**disk** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_context_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**config_context** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


