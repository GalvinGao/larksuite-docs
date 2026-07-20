---
document_id: '7073693024735444997'
directory_id: '7073450228347240453'
title: onThemeChange
full_path: /uYjL24iN/uUTOuUTOuUTO/darkmode/onthemechange
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- DarkMode
- onThemeChange
document_type: GuideDocumentType
updated_at: 2022-03-11T04:15:41Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUTOuUTOuUTO/darkmode/onthemechange
---

# onThemeChange(function callback)

监听系统主题变化的事件

:::html
<md-alert type="tip">
使用同一回调函数多次调用，会注册多次该事件，回调会被执行多次
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

## 输出
回调函数返回对象的属性：
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
              theme
            </md-td>
            <md-td>
                string
            </md-td>
            <md-td>
                当前系统主题，在[小程序支持 DarkMode](/document/uYjL24iN/uQTM5UjL0ETO14CNxkTN/darkmode) 的时候才会返回

**示例值**：light

**可选值**：
- `light`：浅色主题
- `dark`：深色主题
<md-alert type="tip" icon="none">
- Lark [V5.3.0](/document/uYjL24iN/uAjMuAjMuAjM/version-compatibility) 及以上版本支持
- 网页应用：不支持
</md-alert>
            </md-td>
        </md-tr>
    </md-tbody>
</md-table>
:::

## 示例代码


```js
tt.onThemeChange(({ theme }) => {
  console.log('onThemeChange', theme);
});
```

回调函数返回对象示例：
```json
{
  "theme": "dark"
}
```

## 已知问题

- 由于 Android 系统主题的刷新机制会导致整个页面重新创建，所以在 Android 设备上并不能通过此接口实时监听到系统主题变化
