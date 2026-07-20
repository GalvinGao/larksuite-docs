---
document_id: '7073691561008496645'
directory_id: '7073451436034048005'
title: FileSystemManager.readFileSync
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file_sync
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.readFileSync
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:25Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file_sync
---

# FileSystemManager.readFileSync

读取本地文件内容。

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
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
<md-tr>
<md-td>网页应用</md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-version>V5.23.0+</md-version></md-td>
<md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app></md-td>
</md-tr>
</md-tbody>
</md-table>
:::
## 输入

继承[标准对象输入](/document/uYjL24iN/ukzNy4SO3IjL5cjM)，扩展属性描述：

名称 | 数据类型 | 属性 | 默认值 | 描述
--|--|--|--|--
`filePath` | `string` | required |  | 要读取的本地文件路径
`encoding` | `string` | optional  |  | 指定读取文件的字符编码，如果不传 encoding，则以 ArrayBuffer 格式读取文件的二进制内容
`position` | `number` | optional  | 0 | 从文件指定位置开始读，如果不指定，则从文件头开始读。读取的范围应该是左闭右开区间 [position, position+length)。有效范围：[0, fileLength - 1]。单位：byte。
`length` | `number` | optional  | fileLength - position | 指定读取的长度，如果不指定，则读到文件末尾，如果可读长度小于指定长度，则返回可读长度。有效范围：[0, fileLength]。单位：byte。


### encoding 的合法值

| 值                          | 说明         |Android|iOS|PC|
| --------------------------- | ------------ |------|---|--|
| ascii                       |              |支持|支持|不支持|
| base64                      |              |支持|支持|支持|
| binary                      |              |支持|支持|不支持|
| hex                         |              |支持|支持|支持|
| ucs2/ucs-2/utf16le/utf-16le | 以小端序读取   |支持|支持|不支持|
| utf-8/utf8                  |              |支持|支持|支持|
| latin1                      |              |支持|支持|不支持|



## 输出
继承[标准对象输出](/document/uYjL24iN/ukzNy4SO3IjL5cjM#8c92acb8)，`success`返回对象的扩展属性：
名称 | 数据类型 | 描述
--|--|--|--|--
`data` |  `string` \| `ArrayBuffer` | 数据

## 代码示例

```js
const fileSystemManager = tt.getFileSystemManager();

tt.chooseImage({
  success(res) {
    try {
      const data = fileSystemManager.readFileSync(res.tempFilePaths[0]);
      console.log("调用成功", data);
    } catch (err) {
      console.log("调用失败", err);
    }
  },
});
```
## 已知问题

- `encoding`参数与文件内容的实际编码需要保持一致，否则可能会无法读取成功
- `length`参数应当合理设置，过大可能会导致性能问题
