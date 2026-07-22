---
document_id: '7073693024735887365'
directory_id: '7073450228347322373'
title: onLeftNavigationBarClick
full_path: /uYjL24iN/uYjMy4iNyIjL2IjM/onleftnavigationbarclick
breadcrumb:
- Client API
- Web app/Gadget API
- Interface
- Navigation
- setNavigationBar-Web App
- onLeftNavigationBarClick
document_type: GuideDocumentType
updated_at: 2022-03-11T04:14:53Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYjMy4iNyIjL2IjM/onleftnavigationbarclick
---

# onLeftNavigationBarClick(function callback)
监听导航栏左侧点击事件
:::html
<md-alert type="tip">
注意事项：
- 必须使用 [setNavigationBar](/document/uYjL24iN/uYjMy4iNyIjL2IjM/setnavigationbar) 设置导航栏左侧按钮后，才有效。

</md-alert>
:::


## 支持说明

| 应用能力 | Android | iOS | PC | 预览效果 |
| --- | --- | --- | --- | --- |
| 小程序 | **X** | **X** | **X** | / |
| 网页应用 | <md-version>V5.3.0+</md-version> | <md-version>V5.3.0+</md-version> | <md-version>V5.3.0+</md-version> | <md-preview-app type="gadget" disable="true" fontSize="14">预览				 </md-preview-app> |




## 输入

| 名称 | 数据类型 | 必填 | 默认值 | 描述 |
| --- | --- | --- | --- | --- |
| callback | function | 是 |  | 该事件的回调函数 |


## 输出
回调函数返回对象的属性：

| 名称 | 数据类型 | 描述 |
| --- | --- | --- |
| id | string | 对应自定义导航栏 item 的 id |



## 示例代码


```js
tt.setNavigationBar({
    left:{
        items: [
            // 显示文本
            {id: "left_one", text: "left_xxx"}
        ]            
    } ,
    success(res) {
        console.log("success");
    },
    fail(res) {
        console.log(`${JSON.stringify(res)}`);
    }                 
    
});

tt.onLeftNavigationBarClick(ev => {
    alert(`你点击了左上角id 为 ${ev.id} 的按钮`)
})
```

回调函数返回对象示例：
```json
{
    "id": "left_one",
}
```



