---
document_id: '6965379541105164293'
directory_id: '6907567266537324545'
title: chooseLocation
full_path: /uYjL24iN/uUDN1EjL1QTNx4SN0UTM
breadcrumb:
- Client API
- Web app/Gadget API
- Location
- chooseLocation
document_type: GuideDocumentType
updated_at: 2022-02-07T13:37:52Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDN1EjL1QTNx4SN0UTM
---

# `chooseLocation`

打开地图选择位置。

::: note
调用前需要用户授权 scope.userLocation，**请开发者兼容用户拒绝授权的场景**。
该页面假设你已经阅读过了[小程序 API 权限](/document/uYjL24iN/uITMuITMuITM)。如果你对小程序 API 权限还不太了解，推荐你先阅读它。
:::

::: note
该 API 还需要系统授权方可调用，位置精度和调用耗时会因设备而异。
:::

::: note
**PC端暂不支持该API**
:::

## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，**无扩展属性**


## 输出

`success`回调对象参数的扩展属性：

名称 | 数据类型 | 描述
--|--|--|--|--
`name` | `string` | 位置名称
`address` | `string` | 详细地址
`latitude` | `string` | 纬度，浮点数，范围为-90~90，负数表示南纬。返回坐标系为`wgs84`坐标系。
`longitude` | `string` | 经度，浮点数，范围为-180~180，负数表示西经。返回坐标系为`wgs84`坐标系。





## 代码示例

```js
tt.chooseLocation({
    success (res) {
        console.log(res);
    },
    fail (res) {
        console.log(`chooseLocation 调用失败`);
    }
});
```
