---
document_id: '6965379543684218886'
directory_id: '6907567269107630082'
title: openSetting
full_path: /uYjL24iN/uUzMx4SNzEjL1MTM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Setting
- openSetting
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:23Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUzMx4SNzEjL1MTM
---

# openSetting(Object object)

打开设置页面，展示用户设置（包括授予和拒绝）过的[API 权限](/document/uYjL24iN/uITMuITMuITM)，并返回用户设置过的授权结果。

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
     <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/authorized/authorized" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

        <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app></md-td>
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
                authSetting
            </md-td>
            <md-td>
                object
            </md-td>
            <md-td>
                展示用户设置（包括授予和拒绝）过的权限
<md-alert type="tip" icon="none">
PC 端：暂不支持
</md-alert>
            </md-td>
        </md-tr>
      <md-tr>
      <md-td>&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >[scope.userInfo](/document/uYjL24iN/uYzMx4iNzEjL2MTM) </md-text></md-td>
          <md-td>
                boolean
            </md-td>
         <md-td>
                是否授予了获取用户信息权限
            </md-td>
      </md-tr>
       <md-tr>
      <md-td>&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >[scope.userLocation](/document/uYjL24iN/uYzMx4iNzEjL2MTM) </md-text></md-td>
          <md-td>
                boolean
            </md-td>
          <md-td>
                是否授予了地理位置权限
            </md-td>
      </md-tr>
        <md-tr>
      <md-td>&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >[scope.record](/document/uYjL24iN/uYzMx4iNzEjL2MTM) </md-text></md-td>
          <md-td>
                boolean
            </md-td>
         <md-td>
               是否授予了麦克风权限
            </md-td>
      </md-tr>
      <md-tr>
      <md-td>&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >[scope.clipboard](/document/uYjL24iN/uYzMx4iNzEjL2MTM) </md-text></md-td>
          <md-td>
                boolean
            </md-td>
         <md-td>
               是否授予了剪贴板权限
            </md-td>
      </md-tr>
        <md-tr>
      <md-td>&emsp;<span style="color: #8F959E">∟</span>&nbsp;<md-text type="field-name" >[scope.writePhotosAlbum](/document/uYjL24iN/uYzMx4iNzEjL2MTM) </md-text></md-td>
          <md-td>
                boolean
            </md-td>
          <md-td>
               是否授予了保存到相册权限
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
         <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/authorized/authorized" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.openSetting({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`openSetting fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "authSetting": {
        "scope.clipboard": true,
        "scope.userInfo": true
    },
    "errMsg": "openSetting:ok"
}
```
