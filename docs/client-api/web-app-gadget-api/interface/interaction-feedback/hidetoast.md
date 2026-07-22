---
document_id: '6965379541104197637'
directory_id: '6907567266541240322'
title: hideToast
full_path: /uYjL24iN/ukzMy4SOzIjL5MjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Interaction Feedback
- hideToast
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:20Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukzMy4SOzIjL5MjM
---

# hideToast(Object object)


隐藏灰色背景的消息提示框。



:::html
<md-alert type="tip">
注意事项：
loading 的实现基于 toast，所以hideToast也会将 loading 隐藏
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/toast/toast" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.47.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/toast/toast" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.hideToast({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`hideToast fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
  errMsg: "hideToast:ok"
}
``` 
