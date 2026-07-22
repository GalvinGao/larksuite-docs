---
document_id: '6965379567070543877'
directory_id: '6907567266541977602'
title: previewImage
full_path: /uYjL24iN/uMDOx4yM4EjLzgTM
breadcrumb:
- Client API
- Web app/Gadget API
- Media
- Image
- previewImage
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDOx4yM4EjLzgTM
---

# previewImage(Object object)


预览一组图片。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.47.0+</md-version> | <md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> |



## 输入


继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| urls | string[] | 是 |  | 图片地址列表，支持本地和网络url<br>**示例值**：<br>["https: //sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/05b68b58ca78f4d3de5aa4a881f3cf2b.png","https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png"] |
| header | object | 否 |  | 请求 Header，仅为网络url时有效。<br>**示例值**：<br>{"csrf-token": "1234"}<br><md-alert type="tip" icon="none"><br>- PC 端：Lark[V3.8.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- Android/iOS 端：Lark[V3.3.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br></md-alert> |
| current | string | 否 | urls[0]的内容 | 默认显示的图片的地址<br>**示例值**：<br>https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/05b68b58ca78f4d3de5aa4a881f3cf2b.png |


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性



## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/image/image" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.previewImage({
    urls: [
        "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/05b68b58ca78f4d3de5aa4a881f3cf2b.png",
        "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png"
    ],
    current: "https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/33e4ae2ff215314046c51ee1d3008d89_p1QpEy0jkK.png",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`previewImage fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "previewImage:ok"
}
```
