# PowerOutlet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**device** | [**crate::models::NestedDevice**](NestedDevice.md) |  | 
**module** | Option<[**crate::models::ComponentNestedModule**](ComponentNestedModule.md)> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**r#type** | Option<[**crate::models::Type4**](Type_4.md)> |  | [optional]
**power_port** | Option<[**crate::models::NestedPowerPort**](NestedPowerPort.md)> |  | [optional]
**feed_leg** | Option<[**crate::models::FeedLeg**](Feed_leg.md)> |  | [optional]
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


