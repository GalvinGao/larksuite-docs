---
document_id: '6965379541104361477'
directory_id: '6907567266537324545'
title: getLocation
full_path: /uYjL24iN/uUTOz4SN5MjL1kzM
breadcrumb:
- Client API
- Web app/Gadget API
- Location
- getLocation
document_type: GuideDocumentType
updated_at: 2022-06-20T07:25:19Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTOz4SN5MjL1kzM
---

# getLocation(Object object)

获取设备当前的地理位置。

:::html
<md-alert type="tip">
注意事项：
- 调用前需要用户授权 `scope.userLocation`。了解如何授权，可查看[API 权限](/document/uYjL24iN/uITMuITMuITM)。
- 该 API 还需要用户在手机系统中给Lark客户端授予地理位置权限，位置精度和调用耗时会因设备而异。
</md-alert>
:::

::: warnning
该 API 有一定性能消耗，请注意不要频繁调用以防设备过热和耗电过快，小程序框架也会做相应的节流处理。
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-location/get-location" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| type | string | 否 | wgs84 | 坐标系类型<br>**可选值**：<br>- `wgs84`：wgs84 坐标系<br>- `gcj02`：gcj02 坐标系<br><md-alert type="tip" icon="none"><br>Lark [V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 以下版本时，如果需将返回值使用在 [openLocation](/document/uYjL24iN/uQTOz4CN5MjL0kzM) 中，建议指定坐标系为`gcj02`, 否则地图显示可能不准确<br></md-alert> |
| timeout | number | 否 | 5 | 定位超时时间，单位秒。若传入允许范围之外的数值，高精度模式下会使用 10s，最高精度模式使用 3s<br>**最小值**：`3`<br>**最大值**：`180` |
| cacheTimeout | number | 否 | 0 | 定位缓存超时时间，单位秒；每次定位缓存当前定位数据，并记下时间戳，当下次调用在 cacheTimeout 之内时，返回缓存数据。如果 cacheTimeout 小于 0 或大于 60s，则不使用缓存<br>**最小值**：`0`<br>**最大值**：`60` |
| accuracy | string | 否 | high | 指定期望精度，支持 high，best。当指定 high 时，期望精度值为100m，当指定 best 时期望精度值为20m。当定位得到的精度不符合条件时，在timeout之前会继续定位，尝试拿到符合要求的定位结果。<br>**可选值**：<br>- `high`：期望精度值为100m<br>- `best`：期望精度值为20m |



## 输出

`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| latitude | number | 纬度，范围为-90~90，正数表示北，负数表示南 |
| longitude | number | 经度，范围为-180~180，正数表示东，负数表示西 |
| accuracy | number | 位置的精确度<br><md-alert type="tip" icon="none"><br>Android/iOS 均返回水平精度<br></md-alert> |
| verticalAccuracy | number | 垂直精度，单位 m<br><md-alert type="tip" icon="none"><br>Android 无法获取，返回 0<br></md-alert> |
| horizontalAccuracy | number | 水平精度，单位 m |
| authorizationAccuracy | string | 指示应用程序有权使用的位置准确性级别。<br>**可选值**：<br>- `reduced`：非精确位置授权<br>- `full`：精确位置授权<br><md-alert type="tip" icon="none"><br>只有 iOS14 且Lark [V3.36.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持<br></md-alert> |
| timestamp | number | 定位数据的时间戳，单位 ms |



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-location/get-location" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getLocation({
    "type": "gcj02",
    "timeout": 5,
    "cacheTimeout": 30,
    "accuracy": "best",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getLocation fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "authorizationAccuracy": "full",
    "latitude": 30.48318898654514,
    "errMsg": "getLocation:ok",
    "longitude": 120.03518184678819,
    "accuracy": 148,
    "horizontalAccuracy": 148,
    "verticalAccuracy": 13.574329376220703,
    "timestamp": 1637490791204
}
``` 
## 错误码
`fail`返回对象中会包含[errCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码。具体错误码列表参见：

| 错误码 | 描述 | 排查建议 |
| --- | --- | --- |
| 1000001 | 租户后台GPS开关是关闭状态 | 请联系租户管理员解决 |



