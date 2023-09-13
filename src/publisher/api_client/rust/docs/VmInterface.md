# VmInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> |  | [optional][readonly]
**virtual_machine** | Option<[**crate::models::NestedVirtualMachine**](NestedVirtualMachine.md)> |  | 
**name** | **String** |  | 
**enabled** | Option<**bool**> |  | [optional]
**parent** | Option<[**crate::models::NestedVmInterface**](NestedVMInterface.md)> |  | [optional]
**bridge** | Option<[**crate::models::NestedVmInterface**](NestedVMInterface.md)> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**mode** | Option<[**crate::models::Mode**](Mode.md)> |  | [optional]
**untagged_vlan** | Option<[**crate::models::NestedVlan**](NestedVLAN.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::NestedVlan>**](NestedVLAN.md)> |  | [optional]
**vrf** | Option<[**crate::models::NestedVrf**](NestedVRF.md)> |  | [optional]
**l2vpn_termination** | Option<[**crate::models::NestedL2VpnTermination**](NestedL2VPNTermination.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::NestedTag>**](NestedTag.md)> |  | [optional]
**custom_fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**count_ipaddresses** | Option<**i32**> |  | [optional][readonly]
**count_fhrp_groups** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


