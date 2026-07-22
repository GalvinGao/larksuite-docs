---
document_id: '7224019389065969670'
directory_id: '7073448001569914886'
title: shareWebContent
full_path: /uYjL24iN/uQjMuQjMuQjM/share/sharewebcontent
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Share
- shareWebContent
document_type: GuideDocumentType
updated_at: 2023-04-23T02:53:15Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQjMuQjMuQjM/share/sharewebcontent
---

# shareWebContent(Object object)

分享应用内网页内容

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/share/share" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-version>V5.23.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| title | string | 否 |  | 分享组件的标题 |
| url | string | 是 |  | 需要分享的网页链接 |



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性
## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>
  <div style="display: flex">
    <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/share/share" fontSize="16">预览小程序</md-preview-app>
    <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.shareWebContent({
  title: '我是分享标题'，
  url: 'https://www.larksuite.com/',
  success(res) {
    console.log(JSON.stringify(res));
  },
  fail(res) {
    console.log(`share fail: ${JSON.stringify(res)}`);
  }
})
```


