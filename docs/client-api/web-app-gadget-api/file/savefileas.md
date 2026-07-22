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

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **X** | **X** | <md-version>V3.9.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app> |
| 网页应用 | **X** | **X** | <md-version>V5.16.0+</md-version> | / |


## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| filePath | string | 是 |  | 文件路径。**不支持网络地址**<br>**示例值**：ttfile://user/feishu.png |


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
