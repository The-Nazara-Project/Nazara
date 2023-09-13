# WritableInterface

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
**r#type** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<**i32**> |  | [optional]
**bridge** | Option<**i32**> |  | [optional]
**lag** | Option<**i32**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**speed** | Option<**i32**> |  | [optional]
**duplex** | Option<**String**> |  | [optional]
**wwn** | Option<**String**> | 64-bit World Wide Name | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<**String**> |  | [optional]
**rf_role** | Option<**String**> |  | [optional]
**rf_channel** | Option<**String**> |  | [optional]
**poe_mode** | Option<**String**> |  | [optional]
**poe_type** | Option<**String**> |  | [optional]
**rf_channel_frequency** | Option<**f32**> |  | [optional]
**rf_channel_width** | Option<**f32**> |  | [optional]
**tx_power** | Option<**i32**> |  | [optional]
**untagged_vlan** | Option<**i32**> |  | [optional]
**tagged_vlans** | Option<**Vec<i32>**> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**cable_end** | Option<**String**> |  | [optional][readonly]
**wireless_link** | Option<**i32**> |  | [optional]
**link_peers** | Option<**Vec<String>**> |  Return the appropriate serializer for the link termination model.  | [optional][readonly]
**link_peers_type** | Option<**String**> |  | [optional][readonly]
**wireless_lans** | Option<**Vec<i32>**> |  | [optional]
**vrf** | Option<**i32**> |  | [optional]
**l2vpn_termination** | Option<**String**> |  | [optional][readonly]
**connected_endpoints** | Option<**Vec<String>**> |  Return the appropriate serializer for the type of connected object.  | [optional][readonly]
**connected_endpoints_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoints_reachable** | Option<**bool**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**count_ipaddresses** | Option<**i32**> |  | [optional][readonly]
**count_fhrp_groups** | Option<**i32**> |  | [optional][readonly]
**_occupied** | Option<**bool**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


