---
document_id: '6965379543683940358'
directory_id: '6907567269107810306'
title: saveFile
full_path: /uYjL24iN/ugDOz4CO4MjL4gzM
breadcrumb:
- Client API
- Web app/Gadget API
- File
- saveFile
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:34Z
source_url: https://open.larksuite.com/document/uYjL24iN/ugDOz4CO4MjL4gzM
---

# saveFile(Object object)

:::html
<md-alert type="warn">
该接口已停止维护，推荐使用 [FileSystemManager.saveFile(Object object)](/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_save_file)
</md-alert>
:::
保存临时文件到本地永久目录

:::html
<md-alert type="tip">
该 API 会把临时文件移动到永久目录（目录最大200M），所以在调用成功后原文件路径将访问失败
</md-alert>
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
<md-td>**✓**</md-td>
<md-td>**✓**</md-td>
<md-td>**✓**</md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.44.0+</md-version></md-td>
<md-td><md-version>V3.47.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
</md-tbody>
</md-table>
:::

## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

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
<md-th style="width: 10%;">
必填
</md-th>
<md-th style="width: 10%;">
默认值
</md-th>
<md-th>
描述
</md-th>
</md-tr>
</md-thead>
<md-tbody>
<md-tr>
<md-td>
tempFilePath
</md-td>
<md-td>
string
</md-td>
<md-td>
是
</md-td>
<md-td></md-td>
<md-td>
文件临时路径

**示例值**：ttfile://temp/5c5fdd01-03bd-42cf-9938-31fb8b769a19-2863810ed1844e79f1b9bb880acb38d0.png
</md-td>
</md-tr>
<md-tr>
<md-td>
filePath
</md-td>
<md-td>
string
</md-td>
<md-td>
否
</md-td>
<md-td></md-td>
<md-td>
文件路径。格式为：ttfile://user/feishu.png, 其中 ttfile://user/为固定格式，feishu.png 为文件名。如果不填则给一个随机路径。**不支持网络地址**

**示例值**：ttfile://user/feishu.png

<md-alert type="tip" icon="none">
- Android/iOS 端
  - 网页应用：Lark[V3.44.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility)及以上版本支持
- PC 端：暂不支持
</md-alert>
</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

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
savedFilePath
</md-td>
<md-td>
string
</md-td>
<md-td>
保存后的文件路径
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
    <md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="16" style="margin-right: 24px">预览小程序</md-preview-app>
    <md-preview-app type="webApp" disable="true" fontSize="16">预览网页应用</md-preview-app>
  </div>
</div> 
:::

```js
tt.chooseImage({
  success(res) {
    const tempFilePath = res.tempFilePaths[0];
    tt.saveFile({
      tempFilePath,
      filePath: "ttfile://user/feishu.png",
      success(res) {
        console.log(`${JSON.stringify(res)}`);
      },
      fail(res) {
        console.log(`saveFile fail: ${JSON.stringify(res)}`);
      },
    });
  },
});
```

`success`返回对象示例：

```json
{
    "savedFilePath": "ttfile://user/feishu.png",
    "errMsg": "saveFile:ok"
}
```

## 已知问题

- iOS 和 Android 本地文件存储的大小限制为 200M。
- PC 端在 3.10.0 版本以前会弹出对话框供用户选择存储路径，3.10.0 及以上版本将这一特性去除，如果需要弹出对话框，请参考 [saveFileAs](/document/uYjL24iN/uQjN3UjL0YzN14CN2cTN)
