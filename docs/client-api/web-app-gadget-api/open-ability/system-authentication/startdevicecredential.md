---
document_id: '6965379543683514374'
directory_id: '6907567266536800257'
title: startDeviceCredential
full_path: /uYjL24iN/uIDN14iM0UjLyQTN
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- System Authentication
- startDeviceCredential
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:43Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIDN14iM0UjLyQTN
---

# startDeviceCredential(Object object)

打开系统解锁界面


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
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/device-authentication/device-authentication" fontSize="14">预览</md-preview-app>
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
            <md-th style="width: 20%;">名称</md-th>
            <md-th style="width: 18%;">数据类型</md-th>
            <md-th style="width: 10%;">必填</md-th>
            <md-th style="width: 10%;">默认值</md-th>
            <md-th>描述</md-th>
        </md-tr>
    </md-thead>
    <md-tbody>
        <md-tr>
            <md-td>authContent</md-td>
            <md-td>string</md-td>
            <md-td>是</md-td>
            <md-td></md-td>
            <md-td>
                验证描述，即识别过程中显示在界面上的对话框提示内容

**示例值**：解锁界面
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/device-authentication/device-authentication" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.startDeviceCredential({
    authContent: "解锁界面",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`startDeviceCredential fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "startDeviceCredential:ok"
}
```



## 错误码
`fail`返回对象中会包含[errCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码，具体错误码列表参见：
错误码 | 错误信息
--|--|--|--|--
`40000` | 用户未设置锁屏密码
`40002` | 解锁失败
`40003` | authContent 字段不能为空
