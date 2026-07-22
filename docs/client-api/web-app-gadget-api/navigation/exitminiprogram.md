---
document_id: '6965379543684644870'
directory_id: '6907567266537734145'
title: exitMiniProgram
full_path: /uYjL24iN/uATN4IjLwUDOy4CM1gjM
breadcrumb:
- Client API
- Web app/Gadget API
- Navigation
- exitMiniProgram
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:54Z
source_url: https://open.larksuite.com/document/uYjL24iN/uATN4IjLwUDOy4CM1gjM
---

# exitMiniProgram(Object object)


退出当前小程序。



## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/navigator/navigator" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，无扩展属性


## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/navigator/navigator" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
  </div>
</div> 
:::

```js
tt.exitMiniProgram({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`exitMiniProgram fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "exitMiniProgram:ok"
}
```
