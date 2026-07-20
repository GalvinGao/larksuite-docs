---
document_id: '6965379541104787461'
directory_id: '6907567266540535810'
title: getScreenBrightness
full_path: /uYjL24iN/uIjNx4iM2EjLyYTM/get-screen-brightness
breadcrumb:
- Client API
- Web app/Gadget API
- Device
- Screen Brightness
- getScreenBrightness
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:01Z
source_url: https://open.larksuite.com/document/uYjL24iN/uIjNx4iM2EjLyYTM/get-screen-brightness
---

# getScreenBrightness(Object object)
获取屏幕亮度。

::: note
iOS获取系统屏幕亮度。Android获取当前小程序屏幕亮度。
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
	  <md-td><md-version>V3.42.0+</md-version></md-td>
	  <md-td><md-version>V3.42.0+</md-version></md-td>
	  <md-td>**X**</md-td>
      <md-td> <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/screen-brightness/screen-brightness" fontSize="14">预览</md-preview-app>
</md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
	  <md-td><md-version>V3.44.0+</md-version></md-td>
	  <md-td><md-version>V3.44.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" disable="true" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览</md-preview-app></md-td>
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
                value
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                屏幕亮度值，范围 0 ~ 1。0 最暗，1 最亮
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/screen-brightness/screen-brightness" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.getScreenBrightness({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`getScreenBrightness fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "getScreenBrightness:ok",
    "value": 0.8206081986427307
}
```

## 已知问题
Android 端未设置屏幕亮度前获取的为系统默认 value，可能会大于 1。
