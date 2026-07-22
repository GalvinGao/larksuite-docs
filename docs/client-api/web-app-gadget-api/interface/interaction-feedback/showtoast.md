---
document_id: '6965379541104394245'
directory_id: '6907567266541240322'
title: showToast
full_path: /uYjL24iN/ugzMy4COzIjL4MjM
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Interaction Feedback
- showToast
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:17Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugzMy4COzIjL4MjM
---

# showToast(Object object)


显示灰色背景的消息提示。

:::html
<md-alert type="tip">
注意事项：
- 多次弹出 toast/loading 时，后一个会立刻覆盖前一个。
</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **✓** | **✓** | **✓** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/toast/toast" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.47.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| title | string | 是 |  | 消息内容。<br>**示例值**：添加购物车成功<br><md-alert type="tip" icon="none"><br>从3.39版本开始，在平台为移动端且配置显示图标的情况下title将只能显示至多7个字符，其他情况下可显示最多2行<br>- 当显示图标时，title 最多能够展示7个字符<br>- 当不显示图标时，title 最多可显示两行<br></md-alert> |
| duration | number | 否 | 1500 | 提示框停留的时间，单位ms |
| icon | string | 否 | success | 图标的类型<br>**可选值**：<br>- `success`：成功<br>- `loading`：加载中<br>- `none`：不显示图标（PC暂不支持）<br>- `error`：错误（仅PC端有效）<br>- `info`：提示（仅PC端有效）<br>- `warning`：警告（仅PC端有效） |
| mask | boolean | 否 | false | 是否显示透明蒙层，防止触摸穿透<br><md-alert type="tip" icon="none"><br>- Android/iOS 端：Lark[V2.5.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：暂不支持<br></md-alert> |

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
tt.showToast({
    "title": "添加购物车成功",
    "duration": 3000,
    "icon": "success",
    "mask": false,
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`showToast fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

```json
{
  errMsg: "showToast:ok"
}
``` 

## 已知问题
- PC端暂不支持`mask`参数
- PC端暂不支持`icon`为`none`
