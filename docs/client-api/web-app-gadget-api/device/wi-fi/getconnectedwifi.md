---
document_id: '6965379543683170310'
directory_id: '6907567266537832449'
title: getConnectedWifi
full_path: /uYjL24iN/ugjNx4CO2EjL4YTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Wi-Fi
- getConnectedWifi
document_type: GuideDocumentType
updated_at: 2022-09-28T07:09:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugjNx4CO2EjL4YTM
---

# getConnectedWifi(Object object)

获取设备当前所连的 Wifi。

:::html
<md-alert type="tip">
调用前需要用户授权 `scope.userLocation`。了解如何授权，可查看[API 权限](/document/uYjL24iN/uITMuITMuITM)。
</md-alert>
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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/get-connected-wifi/get-connected-wifi" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app></md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性。



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
                SSID
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                Wi-Fi 的 SSID
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                BSSID
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                Wi-Fi 的 BSSID
<md-alert type="tip" icon="none">PC 端：暂不支持
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                secure
            </md-td>
            <md-td>
                boolean
            </md-td>
            <md-td>
                Wi-Fi 是否安全
<md-alert type="tip" icon="none">
  - iOS 端：暂不支持
 - Android 10及以上版本不支持
</md-alert>
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                signalStrength
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                Wi-Fi 信号强度
<md-alert type="tip" icon="none">
iOS 端：暂不支持
</md-alert>
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/get-connected-wifi/get-connected-wifi" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getConnectedWifi({
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getConnectedWifi fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "SSID": "Future Inc",
    "errMsg": "getConnectedWifi:ok",
    "BSSID": "b8:4b:5a:d8:28:d2"
}
```
