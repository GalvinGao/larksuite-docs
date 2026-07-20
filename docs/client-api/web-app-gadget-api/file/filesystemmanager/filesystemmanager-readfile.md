---
document_id: '7073692582770245638'
directory_id: '7073451436034048005'
title: FileSystemManager.readFile
full_path: /uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file
breadcrumb:
- Client API
- Web app/Gadget API
- File
- FileSystemManager
- FileSystemManager.readFile
document_type: GuideDocumentType
updated_at: 2022-11-07T08:12:22Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/file_system_manager/file_system_manager_read_file
---

# FileSystemManager.readFile(Object object)


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
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V5.23.0+</md-version></md-td>
	  <md-td><md-preview-app type="gadget" appId="cli_9dff7f6ae02ad104" path="page/API/pages/file/file" fontSize="14">预览</md-preview-app>
      </md-td>
    </md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V5.23.0+</md-version></md-td>
	 <md-td><md-preview-app type="webApp" appId="cli_9dff7f6ae02ad104" fontSize="14">预览</md-preview-app>
      </md-td>
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
                要读取的本地文件路径

**示例值**：ttfile://temp/test.jpg
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                encoding
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td></md-td>
            <md-td>
                指定读取文件的字符编码，如果不传 encoding，则以 ArrayBuffer 格式读取文件的二进制内容


            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                position
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                0
            </md-td>
            <md-td>
                从文件指定位置开始读，如果不指定，则从文件头开始读。读取的范围应该是左闭右开区间 [position, position+length)。有效范围：[0, fileLength - 1]。单位：byte。

**示例值**：5
            </md-td>
        </md-tr>
        <md-tr>
            <md-td>
                length
            </md-td>
            <md-td>
                number
            </md-td>
            <md-td>
                否
            </md-td>
            <md-td>
                
            </md-td>
            <md-td>
                指定读取的长度，如果不指定，则读到文件末尾，如果可读长度小于指定长度，则返回可读长度。有效范围：[0, fileLength]。单位：byte。

**示例值**：10
              
**单次读取最大**：10 * 1024 *1024
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

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

## 示例代码


```js
const fileSystemManager = tt.getFileSystemManager();
fileSystemManager.readFile({
    filePath: "ttfile://temp/test.jpg",
    encoding: "base64",
    position: 5,
    length: 10,
    success(res) {
      console.log(res);
    },
    fail(res) {
      console.log(`readFile fail: ${JSON.stringify(res)}`);
    }
});
```

`success`返回对象示例：

- data 为 string 类型
```json
{
  data: 'EEpGSUYAAQEAAA==',
  errMsg: 'readFile:ok'
}
```
- data 为 arraybuffer 类型
```json
{
  errMsg: 'readFile:ok',
  data: ArrayBuffer(10)
}
```
## 分片读取示例
```js
function readFileBySlice(path, position, total, slice, uint8Buffer, callback) {
    if (position >= total) {
        callback(uint8Buffer)
        return
    }
    if (slice > 1024 * 1024) {
        slice = 1024 * 1024 // 建议每一片不要超过1M，否则可能引起性能问题
    }
    tt.getFileSystemManager().readFile({
        filePath: path,
        position: position,
        length: slice < total ? slice : total,
        success(res) {
            // 输出读取的文件内容
            const srcUint8 = new Uint8Array(res.data)
            uint8Buffer.set(srcUint8, position)
            const nextPosition = position + srcUint8.buffer.byteLength;
            readFileBySlice(path, nextPosition, total, slice, uint8Buffer, callback)
        },
        fail(res) {
            console.log("调用失败", res.errMsg);
        }
    });
}

const slice = 1024 * 1024 // 1M
const filePath = "your file path";
const fileSize = "your file size";
const uint8Buffer = new Uint8Array(new ArrayBuffer(fileSize))
readFileBySlice(filePath, 0, fileSize, slice, uint8Buffer, (uint8Buffer) => {
  //todo
})
```
:::note
`JSON.stringify(arraybuffer)` 会返回 `'{}'`
:::
## 已知问题

- `encoding`参数与文件内容的实际编码需要保持一致，否则可能会无法读取成功
- `length`参数应当合理设置，过大可能会导致性能问题






