---
document_id: '6965379541104803845'
directory_id: '6907567266541961218'
title: mailto
full_path: /uYjL24iN/uAjNwEjLwYDMx4CM2ATM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Mail
- mailto
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:49Z
source_url: https://open.larksuite.com/document/uYjL24iN/uAjNwEjLwYDMx4CM2ATM
---

# mailto(Object object)

调用系统发送邮件，调用邮件程序后会立即返回结果。后续登录账户、切换账户、发送、编辑、取消、失败等流程不会有回调。


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.2.0+</md-version> | <md-version>V2.2.0+</md-version> | **X** | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/mailto/mailto" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44+</md-version> | <md-version>V3.44+</md-version> | **X** | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="14">预览</md-preview-app> |



## 输入
继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| to | string[] | 否 |  | 收件人邮箱列表<br>**示例值**：["test@gmail.com"] |
| cc | string[] | 否 |  | 抄送邮箱列表<br>**示例值**：["test.cc@gmail.com", "test.cc2@gmail.com"] |
| bcc | string[] | 否 |  | 密送邮箱列表<br>**示例值**：["test.bcc@gmail.com"] |
| subject | string | 否 |  | 主题<br>**示例值**：测试 |
| body | string | 否 |  | 邮件内容<br>**示例值**：测试 |



## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性


## 示例代码
:::html
<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="/page/API/pages/mailto/mailto" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.mailto({
    to: [
        "test@gmail.com"
    ],
    cc: [
        "test.cc@gmail.com",
        "test.cc2@gmail.com"
    ],
    bcc: [
        "test.bcc@gmail.com"
    ],
    subject: "测试",
    body: "测试",
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`mailto fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "mailto:ok"
}
```
