# Interface

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
**r#type** | [**crate::models::Type2**](Type_2.md) |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | [optional]
**bridge** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | [optional]
**lag** | Option<[**crate::models::NestedInterface**](NestedInterface.md)> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**speed** | Option<**i32**> |  | [optional]
**duplex** | Option<[**crate::models::Duplex**](Duplex.md)> |  | [optional]
**wwn** | Option<**String**> | 64-bit World Wide Name | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<[**crate::models::Mode**](Mode.md)> |  | [optional]
**rf_role** | Option<[**crate::models::RfRole**](Rf_role.md)> |  | [optional]
**rf_channel** | Option<[**crate::models::RfChannel**](Rf_channel.md)> |  | [optional]
**poe_mode** | Option<[**crate::models::PoeMode**](Poe_mode.md)> |  | [optional]
**poe_type** | Option<[**crate::models::PoeType**](Poe_type.md)> |  | [optional]
**rf_channel_frequency** | Option<**f32**> |  | [optional]
**rf_channel_width** | Option<**f32**> |  | [optional]
**tx_power** | Option<**i32**> |  | [optional]
**untagged_vlan** | Option<[**crate::models::NestedVlan**](NestedVLAN.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::NestedVlan>**](NestedVLAN.md)> |  | [optional]
**mark_connected** | Option<**bool**> | Treat as if a cable is connected | [optional]
**cable** | Option<[**crate::models::NestedCable**](NestedCable.md)> |  | [optional]
**cable_end** | Option<**String**> |  | [optional][readonly]
**wireless_link** | Option<[**crate::models::NestedWirelessLink**](NestedWirelessLink.md)> |  | [optional]
**link_peers** | Option<**Vec<String>**> |  Return the appropriate serializer for the link termination model.  | [optional][readonly]
**link_peers_type** | Option<**String**> |  | [optional][readonly]
**wireless_lans** | Option<[**Vec<crate::models::NestedWirelessLan>**](NestedWirelessLAN.md)> |  | [optional]
**vrf** | Option<[**crate::models::NestedVrf**](NestedVRF.md)> |  | [optional]
**l2vpn_termination** | Option<[**crate::models::NestedL2VpnTermination**](NestedL2VPNTermination.md)> |  | [optional]
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


