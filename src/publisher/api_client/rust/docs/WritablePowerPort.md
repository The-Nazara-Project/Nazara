# WritablePowerPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**device** | **i32** |  | 
**module** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | Option<**String**> | Physical port type | [optional]
**maximum_draw** | Option<**i32**> | Maximum power draw (watts) | [optional]
**allocated_draw** | Option<**i32**> | Allocated power draw (watts) | [optional]
**description** | Option<**String**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**cable_end** | Option<**String**> |  | [optional][readonly]
**link_peers** | Option<**Vec<String>**> |  Return the appropriate serializer for the link termination model.  | [optional][readonly]
**link_peers_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoints** | Option<**Vec<String>**> |  Return the appropriate serializer for the type of connected object.  | [optional][readonly]
**connected_endpoints_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoints_reachable** | Option<**bool**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**_occupied** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


