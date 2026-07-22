---
document_id: '6965379543684448262'
directory_id: '6907567269107810306'
title: openDocument
full_path: /uYjL24iN/ukTN24SO1YjL5UjN
breadcrumb:
- Client API
- Web app/Gadget API
- File
- openDocument
document_type: GuideDocumentType
updated_at: 2022-03-11T04:17:51Z
source_url: https://open.larksuite.com/document/uYjL24iN/ukTN24SO1YjL5UjN
---

# openDocument(Object object)

在新页面打开文档

:::html
<md-alert type="tip">
仅当文件名有 fileType 中的合法后缀时，文件可正常打开
</md-alert>
:::

## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | <md-version>V2.6.0+</md-version> | <md-version>V2.6.0+</md-version> | <md-version>V2.6.0+</md-version> | <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app> |
| 网页应用 | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-version>V3.44.0+</md-version> | <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app> |


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| filePath | string | 是 |  | 文件路径<br>**最小长度**：`1` 字符 |
| fileType | string | 否 |  | 文件类型，指定打开文档的文件类型。如果 `filePath` 为云空间文档链接，则需要声明为 `cloudFile`<br>**可选值**：<br>- `doc`：doc 格式<br>- `docx`：docx 格式<br>- `xls`：xls 格式<br>- `xlsx`：xlsx 格式<br>- `ppt`：ppt 格式<br>- `pptx`：pptx 格式<br>- `pdf`：pdf 格式<br>- `zip`: zip格式<br>- Android:Lark [V5.2.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- iOS:Lark [V5.1.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- `cloudFile`：云空间文档。支持打开云空间内的所有文档类型，包括文件夹<br>- 小程序：Lark[V3.12.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- 网页应用：Lark[V3.44.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持 |
| showMenu | boolean | 否 | true | 是否显示右上角菜单以及底部使用外部应用打开按钮<br><md-alert type="tip" icon="none"><br>- Android/iOS 端<br>- 小程序：Lark[V3.35.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- 网页应用：Lark[V3.44.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持<br>- PC 端：暂不支持<br></md-alert> |


## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

:::html

<div style="display: flex; justify-content: space-between">
  <md-download-code href="/document/uYjL24iN/uYDM04iNwQjL2ADN" mobileDisplay="none">下载示例代码</md-download-code>

  <div style="display: flex">
          <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
          <md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104"  fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.filePicker({
  success(res) {
    const filePath = res.list[0].path;
    tt.openDocument({
      filePath,
      showMenu: true,
      success(res) {
        console.log(JSON.stringify(res));
      },
      fail(res) {
        console.log(`openDocument fail: ${JSON.stringify(res)}`);
      },
    });
  },
});
```

`success`返回对象示例：

```json
{ "errMsg": "openDocument:ok" }
```
