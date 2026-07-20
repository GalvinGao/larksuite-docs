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
:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">应用能力</md-th>
      <md-th style="width: 20%;">Android</md-th>
       <md-th style="width: 20%;">iOS</md-th>
      <md-th style="width: 20%;">PC</md-th>
      <md-th style="width: 20%;">预览效果</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>小程序</md-td>
      <md-td>**✓**</md-td>
      <md-td>**✓**</md-td>
      <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-location/get-location" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 20%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th style="width: 10%;">
                必填
            </md-th>
            <md-th style="width: 10%;">
                默认值
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                type
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                wgs84
            </md-td>
            <md-td>
                坐标系类型

**可选值**：
- `wgs84`：wgs84 坐标系
- `gcj02`：gcj02 坐标系
<md-alert type="tip" icon="none">
Lark [V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 以下版本时，如果需将返回值使用在 [openLocation](/document/uYjL24iN/uQTOz4CN5MjL0kzM) 中，建议指定坐标系为`gcj02`, 否则地图显示可能不准确
</md-alert>

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                timeout
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                5
            </md-td>
            <md-td>
                定位超时时间，单位秒。若传入允许范围之外的数值，高精度模式下会使用 10s，最高精度模式使用 3s

**最小值**：`3`

**最大值**：`180`
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                cacheTimeout
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>0</md-td>
            <md-td>
                定位缓存超时时间，单位秒；每次定位缓存当前定位数据，并记下时间戳，当下次调用在 cacheTimeout 之内时，返回缓存数据。如果 cacheTimeout 小于 0 或大于 60s，则不使用缓存

**最小值**：`0`

**最大值**：`60`
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                accuracy
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                high
            </md-td>
            <md-td>
                指定期望精度，支持 high，best。当指定 high 时，期望精度值为100m，当指定 best 时期望精度值为20m。当定位得到的精度不符合条件时，在timeout之前会继续定位，尝试拿到符合要求的定位结果。

**可选值**：
- `high`：期望精度值为100m
- `best`：期望精度值为20m
            </md-td>
        </md-tr>

    </md-tbody>
</md-table>
:::


## 输出

`success`返回对象的扩展属性：
:::html
<md-table>
    <md-thead>
        <md-tr>
            <md-th style="width: 30%;">
                名称
            </md-th>
            <md-th style="width: 18%;">
                数据类型
            </md-th>
            <md-th>
                描述
            </md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>
                latitude
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                纬度，范围为-90~90，正数表示北，负数表示南
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                longitude
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                经度，范围为-180~180，正数表示东，负数表示西
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                accuracy
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                位置的精确度
<md-alert type="tip" icon="none">
Android/iOS 均返回水平精度
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                verticalAccuracy
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                垂直精度，单位 m
<md-alert type="tip" icon="none">
Android 无法获取，返回 0
</md-alert>

            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                horizontalAccuracy
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                水平精度，单位 m
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                authorizationAccuracy
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                指示应用程序有权使用的位置准确性级别。

**可选值**：
- `reduced`：非精确位置授权
- `full`：精确位置授权
              
<md-alert type="tip" icon="none">
只有 iOS14 且Lark [V3.36.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持
</md-alert>
            </md-td>
        </md-tr>
           <md-tr>
            <md-td>
                timestamp
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                定位数据的时间戳，单位 ms
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


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

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">错误码</md-th>
      <md-th style="width: 40%;">描述</md-th>
      <md-th style="width: 40%;">排查建议</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>1000001</md-td>
      <md-td>租户后台GPS开关是关闭状态</md-td>
      <md-td>请联系租户管理员解决</md-td>
    </md-tr>
 </md-tbody>
</md-table>
:::


