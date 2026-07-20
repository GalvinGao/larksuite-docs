---
document_id: '7073691561007841285'
directory_id: '7073451436034048005'
title: FileSystemManager.writeFileSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_write_file_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.writeFileSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:13:07Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_write_file_sync
---

# FileSystemManager.writeFileSync

写入本地文件。


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
      <md-td><md-version>V5.23.0+</md-version></md-td>
      <md-td><md-version>V5.23.0+</md-version></md-td>
      <md-td>**X**</md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104"
path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
    </md-tr>
    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.23.0+</md-version></md-td>
      <md-td><md-version>V5.23.0+</md-version></md-td>
      <md-td>**X**</md-td>
      <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app></md-td>
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
      <md-th style="width: 20%;">名称</md-th>
      <md-th style="width: 18%;">数据类型</md-th>
      <md-th style="width: 10%;">必填</md-th>
      <md-th style="width: 10%;">默认值</md-th>
      <md-th>描述</md-th>
    </md-tr>
  </md-thead>
  <md-tbody>
    <md-tr>
      <md-td>filePath</md-td>
      <md-td>string</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        本地文件路径
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>data</md-td>
      <md-td>string/ArrayBuffer</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>
        要写入的文本或二进制数据
      </md-td>
    </md-tr>
    <md-tr>
      <md-td>encoding</md-td>
      <md-td>string</md-td>
      <md-td>否</md-td>
      <md-td>utf8</md-td>
      <md-td>
        指定写入文件的字符编码，默认值仅在data为string类型时生效
      </md-td>
    </md-tr>
  </md-tbody>
</md-table>
:::

### encoding 的合法值

| 值                          | 说明         |Android|iOS|
| --------------------------- | ------------ |------|---|
| ascii                       |              |支持|支持|
| base64                      |              |支持|支持|
| binary                      |              |支持|支持|
| hex                         |              |支持|支持|
| ucs2/ucs-2/utf16le/utf-16le | 以小端序读取   |支持|支持|
| utf-8/utf8                  |              |支持|支持|
| latin1                      |              |支持|支持|

## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，无扩展属性

## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

const filePath = `ttfile://user/example.txt`;

try {
  fileSystemManager.writeFileSync(filePath, "example content", "utf8");
  console.log("调用成功");
  const data = fileSystemManager.readFileSync(filePath);
  console.log("写入的内容为:", data);
} catch (err) {
  console.log("调用失败", err);
}
```


## 已知问题

- `encoding`参数与`data`的实际编码需要保持一致，否则可能会无法写入成功
