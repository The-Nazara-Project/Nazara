# \UsersApi

All URIs are relative to *https://netbox.suse.de/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_config_list**](UsersApi.md#users_config_list) | **GET** /users/config/ | 
[**users_groups_bulk_delete**](UsersApi.md#users_groups_bulk_delete) | **DELETE** /users/groups/ | 
[**users_groups_bulk_partial_update**](UsersApi.md#users_groups_bulk_partial_update) | **PATCH** /users/groups/ | 
[**users_groups_bulk_update**](UsersApi.md#users_groups_bulk_update) | **PUT** /users/groups/ | 
[**users_groups_create**](UsersApi.md#users_groups_create) | **POST** /users/groups/ | 
[**users_groups_delete**](UsersApi.md#users_groups_delete) | **DELETE** /users/groups/{id}/ | 
[**users_groups_list**](UsersApi.md#users_groups_list) | **GET** /users/groups/ | 
[**users_groups_partial_update**](UsersApi.md#users_groups_partial_update) | **PATCH** /users/groups/{id}/ | 
[**users_groups_read**](UsersApi.md#users_groups_read) | **GET** /users/groups/{id}/ | 
[**users_groups_update**](UsersApi.md#users_groups_update) | **PUT** /users/groups/{id}/ | 
[**users_permissions_bulk_delete**](UsersApi.md#users_permissions_bulk_delete) | **DELETE** /users/permissions/ | 
[**users_permissions_bulk_partial_update**](UsersApi.md#users_permissions_bulk_partial_update) | **PATCH** /users/permissions/ | 
[**users_permissions_bulk_update**](UsersApi.md#users_permissions_bulk_update) | **PUT** /users/permissions/ | 
[**users_permissions_create**](UsersApi.md#users_permissions_create) | **POST** /users/permissions/ | 
[**users_permissions_delete**](UsersApi.md#users_permissions_delete) | **DELETE** /users/permissions/{id}/ | 
[**users_permissions_list**](UsersApi.md#users_permissions_list) | **GET** /users/permissions/ | 
[**users_permissions_partial_update**](UsersApi.md#users_permissions_partial_update) | **PATCH** /users/permissions/{id}/ | 
[**users_permissions_read**](UsersApi.md#users_permissions_read) | **GET** /users/permissions/{id}/ | 
[**users_permissions_update**](UsersApi.md#users_permissions_update) | **PUT** /users/permissions/{id}/ | 
[**users_tokens_bulk_delete**](UsersApi.md#users_tokens_bulk_delete) | **DELETE** /users/tokens/ | 
[**users_tokens_bulk_partial_update**](UsersApi.md#users_tokens_bulk_partial_update) | **PATCH** /users/tokens/ | 
[**users_tokens_bulk_update**](UsersApi.md#users_tokens_bulk_update) | **PUT** /users/tokens/ | 
[**users_tokens_create**](UsersApi.md#users_tokens_create) | **POST** /users/tokens/ | 
[**users_tokens_delete**](UsersApi.md#users_tokens_delete) | **DELETE** /users/tokens/{id}/ | 
[**users_tokens_list**](UsersApi.md#users_tokens_list) | **GET** /users/tokens/ | 
[**users_tokens_partial_update**](UsersApi.md#users_tokens_partial_update) | **PATCH** /users/tokens/{id}/ | 
[**users_tokens_provision_create**](UsersApi.md#users_tokens_provision_create) | **POST** /users/tokens/provision/ | 
[**users_tokens_read**](UsersApi.md#users_tokens_read) | **GET** /users/tokens/{id}/ | 
[**users_tokens_update**](UsersApi.md#users_tokens_update) | **PUT** /users/tokens/{id}/ | 
[**users_users_bulk_delete**](UsersApi.md#users_users_bulk_delete) | **DELETE** /users/users/ | 
[**users_users_bulk_partial_update**](UsersApi.md#users_users_bulk_partial_update) | **PATCH** /users/users/ | 
[**users_users_bulk_update**](UsersApi.md#users_users_bulk_update) | **PUT** /users/users/ | 
[**users_users_create**](UsersApi.md#users_users_create) | **POST** /users/users/ | 
[**users_users_delete**](UsersApi.md#users_users_delete) | **DELETE** /users/users/{id}/ | 
[**users_users_list**](UsersApi.md#users_users_list) | **GET** /users/users/ | 
[**users_users_partial_update**](UsersApi.md#users_users_partial_update) | **PATCH** /users/users/{id}/ | 
[**users_users_read**](UsersApi.md#users_users_read) | **GET** /users/users/{id}/ | 
[**users_users_update**](UsersApi.md#users_users_update) | **PUT** /users/users/{id}/ | 



## users_config_list

> users_config_list()


Return the UserConfig for the currently authenticated User.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_delete

> users_groups_bulk_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_partial_update

> crate::models::Group users_groups_bulk_partial_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Group**](Group.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_update

> crate::models::Group users_groups_bulk_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Group**](Group.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_create

> crate::models::Group users_groups_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**Group**](Group.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_delete

> users_groups_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_list

> crate::models::UsersGroupsList200Response users_groups_list(id, name, q, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, name__empty, ordering, limit, offset)


Overrides ListModelMixin to allow processing ExportTemplates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**name__n** | Option<**String**> |  |  |
**name__ic** | Option<**String**> |  |  |
**name__nic** | Option<**String**> |  |  |
**name__iew** | Option<**String**> |  |  |
**name__niew** | Option<**String**> |  |  |
**name__isw** | Option<**String**> |  |  |
**name__nisw** | Option<**String**> |  |  |
**name__ie** | Option<**String**> |  |  |
**name__nie** | Option<**String**> |  |  |
**name__empty** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::UsersGroupsList200Response**](users_groups_list_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_partial_update

> crate::models::Group users_groups_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**data** | [**Group**](Group.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_read

> crate::models::Group users_groups_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_update

> crate::models::Group users_groups_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**data** | [**Group**](Group.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_delete

> users_permissions_bulk_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_partial_update

> crate::models::ObjectPermission users_permissions_bulk_partial_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableObjectPermission**](WritableObjectPermission.md) |  | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_update

> crate::models::ObjectPermission users_permissions_bulk_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableObjectPermission**](WritableObjectPermission.md) |  | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_create

> crate::models::ObjectPermission users_permissions_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableObjectPermission**](WritableObjectPermission.md) |  | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_delete

> users_permissions_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_list

> crate::models::UsersPermissionsList200Response users_permissions_list(id, name, enabled, object_types, description, q, user_id, user, group_id, group, id__n, id__lte, id__lt, id__gte, id__gt, name__n, name__ic, name__nic, name__iew, name__niew, name__isw, name__nisw, name__ie, name__nie, name__empty, object_types__n, description__n, description__ic, description__nic, description__iew, description__niew, description__isw, description__nisw, description__ie, description__nie, description__empty, user_id__n, user__n, group_id__n, group__n, ordering, limit, offset)


Overrides ListModelMixin to allow processing ExportTemplates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**enabled** | Option<**String**> |  |  |
**object_types** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**user_id** | Option<**String**> |  |  |
**user** | Option<**String**> |  |  |
**group_id** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**name__n** | Option<**String**> |  |  |
**name__ic** | Option<**String**> |  |  |
**name__nic** | Option<**String**> |  |  |
**name__iew** | Option<**String**> |  |  |
**name__niew** | Option<**String**> |  |  |
**name__isw** | Option<**String**> |  |  |
**name__nisw** | Option<**String**> |  |  |
**name__ie** | Option<**String**> |  |  |
**name__nie** | Option<**String**> |  |  |
**name__empty** | Option<**String**> |  |  |
**object_types__n** | Option<**String**> |  |  |
**description__n** | Option<**String**> |  |  |
**description__ic** | Option<**String**> |  |  |
**description__nic** | Option<**String**> |  |  |
**description__iew** | Option<**String**> |  |  |
**description__niew** | Option<**String**> |  |  |
**description__isw** | Option<**String**> |  |  |
**description__nisw** | Option<**String**> |  |  |
**description__ie** | Option<**String**> |  |  |
**description__nie** | Option<**String**> |  |  |
**description__empty** | Option<**String**> |  |  |
**user_id__n** | Option<**String**> |  |  |
**user__n** | Option<**String**> |  |  |
**group_id__n** | Option<**String**> |  |  |
**group__n** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::UsersPermissionsList200Response**](users_permissions_list_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_partial_update

> crate::models::ObjectPermission users_permissions_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |
**data** | [**WritableObjectPermission**](WritableObjectPermission.md) |  | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_read

> crate::models::ObjectPermission users_permissions_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_update

> crate::models::ObjectPermission users_permissions_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |
**data** | [**WritableObjectPermission**](WritableObjectPermission.md) |  | [required] |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_delete

> users_tokens_bulk_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_partial_update

> crate::models::Token users_tokens_bulk_partial_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableToken**](WritableToken.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_update

> crate::models::Token users_tokens_bulk_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableToken**](WritableToken.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_create

> crate::models::Token users_tokens_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableToken**](WritableToken.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_delete

> users_tokens_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_list

> crate::models::UsersTokensList200Response users_tokens_list(id, key, write_enabled, description, q, user_id, user, created, created__gte, created__lte, expires, expires__gte, expires__lte, id__n, id__lte, id__lt, id__gte, id__gt, key__n, key__ic, key__nic, key__iew, key__niew, key__isw, key__nisw, key__ie, key__nie, key__empty, description__n, description__ic, description__nic, description__iew, description__niew, description__isw, description__nisw, description__ie, description__nie, description__empty, user_id__n, user__n, ordering, limit, offset)


Overrides ListModelMixin to allow processing ExportTemplates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**key** | Option<**String**> |  |  |
**write_enabled** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**user_id** | Option<**String**> |  |  |
**user** | Option<**String**> |  |  |
**created** | Option<**String**> |  |  |
**created__gte** | Option<**String**> |  |  |
**created__lte** | Option<**String**> |  |  |
**expires** | Option<**String**> |  |  |
**expires__gte** | Option<**String**> |  |  |
**expires__lte** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**key__n** | Option<**String**> |  |  |
**key__ic** | Option<**String**> |  |  |
**key__nic** | Option<**String**> |  |  |
**key__iew** | Option<**String**> |  |  |
**key__niew** | Option<**String**> |  |  |
**key__isw** | Option<**String**> |  |  |
**key__nisw** | Option<**String**> |  |  |
**key__ie** | Option<**String**> |  |  |
**key__nie** | Option<**String**> |  |  |
**key__empty** | Option<**String**> |  |  |
**description__n** | Option<**String**> |  |  |
**description__ic** | Option<**String**> |  |  |
**description__nic** | Option<**String**> |  |  |
**description__iew** | Option<**String**> |  |  |
**description__niew** | Option<**String**> |  |  |
**description__isw** | Option<**String**> |  |  |
**description__nisw** | Option<**String**> |  |  |
**description__ie** | Option<**String**> |  |  |
**description__nie** | Option<**String**> |  |  |
**description__empty** | Option<**String**> |  |  |
**user_id__n** | Option<**String**> |  |  |
**user__n** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::UsersTokensList200Response**](users_tokens_list_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_partial_update

> crate::models::Token users_tokens_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |
**data** | [**WritableToken**](WritableToken.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_provision_create

> users_tokens_provision_create()


Non-authenticated REST API endpoint via which a user may create a Token.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_read

> crate::models::Token users_tokens_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_update

> crate::models::Token users_tokens_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this token. | [required] |
**data** | [**WritableToken**](WritableToken.md) |  | [required] |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_delete

> users_users_bulk_delete()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_partial_update

> crate::models::User users_users_bulk_partial_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableUser**](WritableUser.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_update

> crate::models::User users_users_bulk_update(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableUser**](WritableUser.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_create

> crate::models::User users_users_create(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**WritableUser**](WritableUser.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_delete

> users_users_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_list

> crate::models::UsersUsersList200Response users_users_list(id, username, first_name, last_name, email, is_staff, is_active, q, group_id, group, id__n, id__lte, id__lt, id__gte, id__gt, username__n, username__ic, username__nic, username__iew, username__niew, username__isw, username__nisw, username__ie, username__nie, username__empty, first_name__n, first_name__ic, first_name__nic, first_name__iew, first_name__niew, first_name__isw, first_name__nisw, first_name__ie, first_name__nie, first_name__empty, last_name__n, last_name__ic, last_name__nic, last_name__iew, last_name__niew, last_name__isw, last_name__nisw, last_name__ie, last_name__nie, last_name__empty, email__n, email__ic, email__nic, email__iew, email__niew, email__isw, email__nisw, email__ie, email__nie, email__empty, group_id__n, group__n, ordering, limit, offset)


Overrides ListModelMixin to allow processing ExportTemplates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> |  |  |
**username** | Option<**String**> |  |  |
**first_name** | Option<**String**> |  |  |
**last_name** | Option<**String**> |  |  |
**email** | Option<**String**> |  |  |
**is_staff** | Option<**String**> |  |  |
**is_active** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**group_id** | Option<**String**> |  |  |
**group** | Option<**String**> |  |  |
**id__n** | Option<**String**> |  |  |
**id__lte** | Option<**String**> |  |  |
**id__lt** | Option<**String**> |  |  |
**id__gte** | Option<**String**> |  |  |
**id__gt** | Option<**String**> |  |  |
**username__n** | Option<**String**> |  |  |
**username__ic** | Option<**String**> |  |  |
**username__nic** | Option<**String**> |  |  |
**username__iew** | Option<**String**> |  |  |
**username__niew** | Option<**String**> |  |  |
**username__isw** | Option<**String**> |  |  |
**username__nisw** | Option<**String**> |  |  |
**username__ie** | Option<**String**> |  |  |
**username__nie** | Option<**String**> |  |  |
**username__empty** | Option<**String**> |  |  |
**first_name__n** | Option<**String**> |  |  |
**first_name__ic** | Option<**String**> |  |  |
**first_name__nic** | Option<**String**> |  |  |
**first_name__iew** | Option<**String**> |  |  |
**first_name__niew** | Option<**String**> |  |  |
**first_name__isw** | Option<**String**> |  |  |
**first_name__nisw** | Option<**String**> |  |  |
**first_name__ie** | Option<**String**> |  |  |
**first_name__nie** | Option<**String**> |  |  |
**first_name__empty** | Option<**String**> |  |  |
**last_name__n** | Option<**String**> |  |  |
**last_name__ic** | Option<**String**> |  |  |
**last_name__nic** | Option<**String**> |  |  |
**last_name__iew** | Option<**String**> |  |  |
**last_name__niew** | Option<**String**> |  |  |
**last_name__isw** | Option<**String**> |  |  |
**last_name__nisw** | Option<**String**> |  |  |
**last_name__ie** | Option<**String**> |  |  |
**last_name__nie** | Option<**String**> |  |  |
**last_name__empty** | Option<**String**> |  |  |
**email__n** | Option<**String**> |  |  |
**email__ic** | Option<**String**> |  |  |
**email__nic** | Option<**String**> |  |  |
**email__iew** | Option<**String**> |  |  |
**email__niew** | Option<**String**> |  |  |
**email__isw** | Option<**String**> |  |  |
**email__nisw** | Option<**String**> |  |  |
**email__ie** | Option<**String**> |  |  |
**email__nie** | Option<**String**> |  |  |
**email__empty** | Option<**String**> |  |  |
**group_id__n** | Option<**String**> |  |  |
**group__n** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |

### Return type

[**crate::models::UsersUsersList200Response**](users_users_list_200_response.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_partial_update

> crate::models::User users_users_partial_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |
**data** | [**WritableUser**](WritableUser.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_read

> crate::models::User users_users_read(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_update

> crate::models::User users_users_update(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this user. | [required] |
**data** | [**WritableUser**](WritableUser.md) |  | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

