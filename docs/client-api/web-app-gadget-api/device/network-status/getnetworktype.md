---
document_id: '6965379543683203078'
directory_id: '6907567266540748802'
title: getNetworkType
full_path: /uYjL24iN/uYjNx4iN2EjL2YTM
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Network Status
- getNetworkType
document_type: GuideDocumentType
updated_at: 2022-03-11T04:16:49Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYjNx4iN2EjL2YTM
---

# getNetworkType(Object object)


获取设备当前所处的网络类型



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
      <md-td>**✓**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-network-type/get-network-type" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.47.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


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
                networkType
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                网络类型
              
**可选值**：
- `none`：无网络
- `wifi`：wifi 网络
- `2g`：2G 网络
- `3g`：3G 网络
- `4g`：4G 网络
- `unknown`：如果设备无法确定上述网络类型，则会返回该值
- `connected`：已连接网络，仅 **PC端** 返回      
<md-alert type="tip" icon="none">
PC端仅支持 `connected` 和 `none` 两种类型
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/get-network-type/get-network-type" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
<md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getNetworkType({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getNetworkType fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
    "networkType": "wifi",
    "errMsg": "getNetworkType:ok"
}
```

