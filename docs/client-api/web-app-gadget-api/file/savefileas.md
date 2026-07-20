---
document_id: '6965379543684136966'
directory_id: '6907567269107810306'
title: saveFileAs
full_path: /uYjL24iN/uQjN3UjL0YzN14CN2cTN
breadcrumb:
- Client API
- Web app/Gadget API
- File
- saveFileAs
document_type: GuideDocumentType
updated_at: 2022-11-07T08:11:37Z
source_url: https://open.larksuite.com/document/uYjL24iN/uQjN3UjL0YzN14CN2cTN
---

# saveFileAs(Object object)

保存文件到本地指定目录

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
<md-td>**X**</md-td>
<md-td>**X**</md-td>
<md-td><md-version>V3.9.0+</md-version></md-td>
<md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td>**X**</md-td>
<md-td>**X**</md-td>
<md-td><md-version>V5.16.0+</md-version></md-td>
<md-td>/</md-td>
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
filePath
</md-td>
<md-td>
string
</md-td>
<md-td>
是
</md-td>
<md-td></md-td>
<md-td>
文件路径。**不支持网络地址**

**示例值**：ttfile://user/feishu.png
</md-td>
</md-tr>
</md-tbody>
</md-table>
:::

## 输出

继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 示例代码

```js
tt.chooseImage({
  success(res) {
    const tempFilePath = res.tempFilePaths[0];
    const filePath = "ttfile://user/feishu.png";
    tt.saveFile({
      tempFilePath,
      filePath,
      success(res) {
        tt.saveFileAs({
          filePath,
          success(res) {
            console.log(`${JSON.stringify(res)}`);
          },
          fail(res) {
            console.log(`saveFileAs fail: ${JSON.stringify(res)}`);
          },
        });
      },
    });
  },
});
```

`success`返回对象示例：

```json
{
  "errMsg": "saveFileAs:ok"
}
```
