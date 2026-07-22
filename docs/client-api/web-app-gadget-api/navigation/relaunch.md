---
document_id: '6965379543684333574'
directory_id: '6907567266537734145'
title: reLaunch
full_path: /uYjL24iN/uEDM04SMwQjLxADN
breadcrumb:
- Client API
- Web app/Gadget API
- Navigation
- reLaunch
document_type: GuideDocumentType
updated_at: 2022-03-11T04:19:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEDM04SMwQjLxADN
---

# reLaunch(Object object)

关闭所有当前页面，打开指定页面。

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/navigator/navigator" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | **X** | / |



## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| url | string | 是 |  | 需要跳转的应用内页面路径 , 路径后可以带参数。参数与路径之间使用`?`分隔，参数键与参数值用`=`相连，不同参数用`&`分隔；如 `path?key=value&key2=value2`，如果跳转的页面路径是 tabBar 页面则不能带参数 |


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
tt.reLaunch({
    "url": "/pages/api/index",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`reLaunch fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "reLaunch:ok"
}
```

