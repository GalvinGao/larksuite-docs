---
document_id: '7073692582769967110'
directory_id: '7073450228347322373'
title: onRightNavigationBarClick
full_path: /uYjL24iN/uYjMy4iNyIjL2IjM/onrightnavigationbarclick
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Navigation
- setNavigationBar-Web App
- onRightNavigationBarClick
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYjMy4iNyIjL2IjM/onrightnavigationbarclick
---

# onRightNavigationBarClick(function callback)
监听导航栏右侧点击事件

:::html
<md-alert type="tip">
注意事项：
- 必须使用 [setNavigationBar](/document/uYjL24iN/uYjMy4iNyIjL2IjM/setnavigationbar) 设置导航栏右侧按钮后，才有效。

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
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>**X**</md-td>
      <md-td>/</md-td>
    </md-tr>
    <md-tr>
      <md-td>网页应用</md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-version>V5.3.0+</md-version></md-td>
      <md-td><md-preview-app type="gadget" disable="true" fontSize="14">预览				 </md-preview-app>
	  </md-td>
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
<md-th style="width: 20%;">
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
id
</md-td>
<md-td>
string
</md-td>
<md-td>
对应自定义导航栏 item 的 id
</md-td>
</md-tr>
<md-tr>
</md-tbody>
</md-table>

:::


## 示例代码


```js
tt.setNavigationBar({
    right:{
        items: [
            // 显示文本
            {id: "right_one", text: "right_xxx"}
        ]            
    } ,
    success(res) {
        console.log("success");
    },
    fail(res) {
        console.log(`${JSON.stringify(res)}`);
    }                 
    
});

tt.onRightNavigationBarClick(ev => {
    alert(`你点击了右上角id 为 ${ev.id} 的按钮`)
})
```

回调函数返回对象示例：
```json
{
    "id": "right_one",
}
```



