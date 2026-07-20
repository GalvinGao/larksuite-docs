---
document_id: '6965379541104820229'
directory_id: '6907567266536980481'
title: startPasswordVerify
full_path: /uYjL24iN/ugTO3IjL4kzNy4CO5cjM
breadcrumb:
- Client API
- Web app/Gadget API
- Open Ability
- Security Code Verify
- startPasswordVerify
document_type: GuideDocumentType
updated_at: 2022-03-11T04:13:41Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugTO3IjL4kzNy4CO5cjM
---

# startPasswordVerify(Object object)

调起二次验证Lark安全密码的输入界面

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
<md-td><md-version>V3.1.0+</md-version></md-td>
<md-td><md-version>V3.1.0+</md-version></md-td>
<md-td><md-version>V3.1.0+</md-version></md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/password-verify/password-verify" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.47.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" disable="true" fontSize="14">预览</md-preview-app></md-td>
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
<md-th style="width: 20%;">
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
token
</md-td>
<md-td>
string
</md-td>
<md-td>
认证 token 信息
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
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/password-verify/password-verify" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.startPasswordVerify({ 
    success(res) {
      console.log(JSON.stringify(res));
    },
    fail(res) {
      console.log(`startPasswordVerify fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：
```json
{
    "errMsg": "startPasswordVerify:ok",
    "token": "436147ae-****-****-****-d71142f6a69f"
}
```

## 错误码
`fail`返回对象中会包含[errCode属性](/document/uYjL24iN/ukzNy4SO3IjL5cjM#a825f4c8)，代表错误码，具体错误码列表参见：

:::html
<md-table>
  <md-thead>
    <md-tr>
      <md-th style="width: 20%;">错误码</md-th>
      <md-th style="width: 40%;">描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>40101</md-td>
      <md-td>用户取消，验证失败</md-td>
    </md-tr>
    <md-tr>
      <md-td>40102</md-td>
      <md-td>密码错误，验证失败</md-td>
    </md-tr>
    <md-tr>
      <md-td>40103</md-td>
      <md-td>密码输入次数超限制，验证失败</md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::
