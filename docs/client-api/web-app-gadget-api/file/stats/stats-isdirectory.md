---
document_id: '7073693024736002053'
directory_id: '7073451436033851397'
title: Stats.isDirectory
full_path: /uYjL24iN/uETOuETOuETO/stat/stats_is_directory
breadcrumb:
- Client API
- Web app/Gadget API
- File
- Stats
- Stats.isDirectory
document_type: GuideDocumentType
updated_at: 2022-11-07T08:13:13Z
source_url: https://open.larksuite.com/document/uYjL24iN/uETOuETOuETO/stat/stats_is_directory
---

# Stats.isDirectory()

判断当前文件是否一个目录。


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
      <md-td>**X**</md-td>
           <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td>
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td><md-version>V4.11.0+</md-version></md-td>
      <md-td>**X**</md-td>
<md-td><md-preview-app type="webApp" disable="true" fontSize="14">预览</md-preview-app> </md-td>
    </md-tr>
    
    
</md-tbody>
</md-table>
:::



## 输入
无

## 输出

返回值：boolean


## 示例代码


```js
const fileSystemManager = tt.getFileSystemManager();
fileSystemManager.stat({
    path: "ttfile://temp",
    success(res) {
      console.log(res.stat.isDirectory());
    },
    fail(res) {
      console.log(`stat fail: ${JSON.stringify(res)}`);
    }
});
```
返回值示例：

```json
true
``` 
