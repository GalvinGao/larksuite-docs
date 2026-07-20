---
document_id: '7073691561008250885'
directory_id: '7073450228347240453'
title: offThemeChange
full_path: /uYjL24iN/uUTOuUTOuUTO/darkmode/offthemechange
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- DarkMode
- offThemeChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:41Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTOuUTOuUTO/darkmode/offthemechange
---

# offThemeChange(function callback)

取消监听系统主题变化的事件
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
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览</md-preview-app></md-td> 
</md-tr>

    <md-tr>
      <md-td>网页应用</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
</md-tr>
    
    
    
</md-tbody>
</md-table>
:::
## 输入
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
      <md-td>callback</md-td>
      <md-td>function</md-td>
      <md-td>是</md-td>
      <md-td></md-td>
      <md-td>该事件的回调函数</md-td>

   </md-tr>  
    

    
</md-tbody>
</md-table>
:::

::: note
如果不传递，不会取消所有 themeChange 事件
:::

## 输出
无


## 代码示例

```js
const fn = ({ theme }) => {
  console.log('onThemeChange', theme);
};
tt.onThemeChange(fn);
// 在某个时机取消监听
tt.offThemeChange(fn);
```
 
