---
document_id: '6965379541105000453'
directory_id: '6907567266537324545'
title: openLocation
full_path: /uYjL24iN/uQTOz4CN5MjL0kzM
breadcrumb:
- Client API
- Web app/Gadget API
- Location
- openLocation
document_type: GuideDocumentType
updated_at: 2022-02-07T13:37:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQTOz4CN5MjL0kzM
---

# `openLocation`

使用客户端内置地图查看位置。

::: note
该 API 需要用户授权方可调用，详细信息可参考[小程序API权限](/document/uYjL24iN/uITMuITMuITM)
:::

::: warnning
该 API 有一定性能消耗，请注意不要频繁调用以防设备过热和耗电过快。小程序框架也会做相应的节流处理。
:::

::: note
**PC端暂不支持该API**
:::

## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

名称 | 数据类型 | 属性 | 默认值 | 描述
--|--|--|--|--
`latitude` | `number` | required | N/A | 纬度，范围为`-90`~`90`，正数表示北，负数表示南。输入坐标系需为`wgs84`坐标系。
`longitude` | `number` | required | N/A | 经度，范围为`-180`~`180`，正数表示东，负数表示西。输入坐标系需为`wgs84`坐标系。
`scale` | `number` | optional | `18` | 缩放比例，范围`5`~`18`
`name` | `string` | optional | `` | 位置名
`address` | `string` | optional | `` |地址的详细说明


## 输出

**各 callback 参数均无额外属性**



## 代码示例

```js
tt.getLocation({
    success: function(res) {
        var latitude = res.latitude;
        var longitude = res.longitude;
        tt.openLocation({
            latitude: latitude,
            longitude: longitude,
            scale: 18
        });
    }
})
```
