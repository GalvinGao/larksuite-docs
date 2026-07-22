---
document_id: '6965379541104459781'
directory_id: '6907567269107630082'
title: getSetting
full_path: /uYjL24iN/uQzMx4CNzEjL0MTM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Setting
- getSetting
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:26Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQzMx4CNzEjL0MTM
---

# getSetting(Object object)

获取用户设置（包括授予和拒绝）过的权限

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/authorized/authorized" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性

## 输出
`success`返回对象的扩展属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| authSetting | object | 展示用户设置（包括授予和拒绝）过的权限 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[scope.userInfo](/document/uYjL24iN/uYzMx4iNzEjL2MTM)<br></md-text> | boolean | 是否授予了用户权限 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[scope.userLocation](/document/uYjL24iN/uYzMx4iNzEjL2MTM)<br></md-text> | boolean | 是否授予了地理位置权限 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[scope.record](/document/uYjL24iN/uYzMx4iNzEjL2MTM)<br></md-text> | boolean | 是否授予了麦克风权限 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[scope.clipboard](/document/uYjL24iN/uYzMx4iNzEjL2MTM)<br></md-text> | boolean | 是否授予了剪贴板权限 |
| &emsp;<br><span style="color: #8F959E"><br>∟<br></span><br>&nbsp;<br><md-text type="field-name"><br>[scope.writePhotosAlbum](/document/uYjL24iN/uYzMx4iNzEjL2MTM)<br></md-text> | boolean | 是否授予了保存到相册权限 |



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
tt.getSetting({ 
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
    "errMsg": "getSetting:ok"
}
```
