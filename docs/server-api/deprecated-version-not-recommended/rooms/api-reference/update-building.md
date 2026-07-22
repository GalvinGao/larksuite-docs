---
document_id: '6967251478324396038'
directory_id: '6907567269107843074'
title: 更新建筑物
full_path: /ukTMukTMukTM/uETNwYjLxUDM24SM1AjN
breadcrumb:
- Server API
- Deprecated Version (Not Recommended)
- Rooms
- API Reference
- Update building
document_type: GuideDocumentType
updated_at: 2023-06-16T07:35:39Z
source_url: https://open.larksuite.com/document/ukTMukTMukTM/uETNwYjLxUDM24SM1AjN
---

# 更新建筑物

该接口用于编辑建筑信息，添加楼层，删除楼层，编辑楼层信息。

## 请求

| 基本 |  |
| --- | --- |
| HTTP URL | https://open.larksuite.com/open-apis/meeting_room/building/update |
| HTTP Method | POST |
| 支持的应用类型 | <md-app-support types="custom"></md-app-support> |
| 权限要求<br><md-tooltip type="info">调用该 API 所需的权限。开启其中任意一项权限即可调用</md-tooltip> | <md-perm name="calendar:room" desc="管理会议室信息" support_app_types="custom" tags="">管理会议室信息</md-perm> |

### 请求头

| 名称 | 类型 | 必填 | 描述 |
| --- | --- | --- | --- |
| Authorization | string | 是 | <md-tag mode="inline" type="token-tenant">tenant_access_token</md-tag><br>**值格式**："Bearer `access_token`"<br>**示例值**："Bearer t-7f1bcd13fc57d46bac21793a18e560"<br>[了解更多：如何选择与获取 access token](/document/uAjLw4CM/ugTN1YjL4UTN24CO1UjN/trouble-shooting/how-to-choose-which-type-of-token-to-use) |
| Content-Type | string | 是 | **固定值**："application/json; charset=utf-8" |


### 请求体

| 参数       | 参数类型 | 必须 | 说明                                                         |
| ---------- | -------- | ---- | ------------------------------------------------------------ |
| building_id  | string      | 是   | 要更新的建筑ID |
| name  | string      | 否   | 建筑名称 |
| floors | string[]   | 否   | 楼层列表 |
| country_id   | string   | 否   | 国家/地区ID |
| district_id     | string   | 否   | 城市ID |
| custom_building_id     | string   | 否   | 租户自定义建筑ID |

### 请求体示例

```json
{
	"building_id":"omb_8ec170b937536a5d87c23b418b83f9bb",
    "name":"测试建筑",
    "floors":[
        "F1",
        "F2",
        "F3",
        "F4"
    ],
    "country_id":"1814991",
    "district_id":"2034437",
    "custom_building_id":"test_building_01"
}
```

## 响应

### 响应体

| 参数         | 说明                                                 |
| ------------ | ---------------------------------------------------- |
| code         | 返回码，非 0 表示失败                                |
| msg          | 返回码的描述，"success" 表示成功，其他为错误提示信息 |

### 响应体示例

```json
{
    "code":0,
    "msg":"success",
}
```

### 错误码

具体可参考：[服务端错误码说明](/document/ukTMukTMukTM/ugjM14COyUjL4ITN)
