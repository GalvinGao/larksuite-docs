---
document_id: '6965379543683301382'
directory_id: '6907567266540830722'
title: Component 构造器
full_path: /uYjL24iN/uADMx4CMwEjLwATM
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Components
- Custom Components
- Component Builder
document_type: GuideDocumentType
updated_at: 2022-03-11T04:12:02Z
source_url: https://open.larksuite.com/document/uYjL24iN/uADMx4CMwEjLwATM
---

# Component 构造器

Component 构造器可用于定义组件，调用 Component 构造器时可以指定组件的属性、数据、方法等。

## 定义参数

|选项名|类型|是否必填|说明|
|-----|---|-------|---|
|properties|Object |否|组件的对外属性，是属性名到属性设置的映射表，<br>属性设置中可包含三个字段:<br>`type` 表示属性类型、<br>`value` 表示属性初始值、<br>`observer` 表示属性值被更改时的响应函数。|
|data|Object|否|组件的内部数据，和 `properties` 一同用于组件的模版渲染。|
|methods|Object|否|组件的方法，包括事件响应函数和任意的自定义方法，<br>关于事件响应函数的使用，参见[组件间通信与事件](/document/uYjL24iN/uEDMx4SMwEjLxATM)。|
|behaviors|String/Array|否|类似于 mixins|
|created|Function|否|组件生命周期函数，在组件实例进入页面节点树时执行，<br>注意此时不能调用 `setData`。|
|attached|Function|否|组件生命周期函数，在组件实例进入页面节点树时执行。|
|ready|Function|否|组件生命周期函数，在组件布局完成后执行。|
|moved|Function|否|组件生命周期函数，在组件实例被移动到节点树另一个位置时执行。|
|detached|Function|否|组件生命周期函数，在组件实例被从页面节点树移除时执行。|
|options|Object|否|其他选项|

生成的组件实例可以在组件的方法、生命周期函数和属性 `observer` 中通过 `this` 访问。组件包含一些通用属性和方法。

**通用属性列表**

|属性名|类型|说明|
|-----|---|---|
|is|String|组件的文件路径|
|id|String|节点 id|
|dataset|String|节点 dataset|
|data|Object|组件数据|
|properties|Object|组件数据|

**方法列表**

|方法名|参数及其类型|说明|
|-----|---|---|
|setData|Object `data`|设置 data|
|hasBehavior|Object `behavior`|检查组件是否具有 `behavior`|
|triggerEvent|String `eventName`, Object `detail`, Object `options`|触发事件|
|selectComponent|String `selector`， Function `callback`|使用选择器选择组件实例节点，返回匹配到的第一个组件实例对象，此方法为异步方法，返回值会在请求完成后传入 callback 。|
|selectAllComponents|String `selector`， Function `callback`|使用选择器选择组件实例节点，返回匹配到的全部组件实例对象组成的数组，此方法为异步方法，返回值会在请求完成后传入 callback 。|

**代码示例**

```js
Component({
  behaviors: [],

  properties: {
    myProperty: {
      // 类型（必填），目前接受的类型包括：String, Number, Boolean, Object, Array, null（表示任意类型, 暂不支持 function）
      type: String,

      // 属性初始值（可选），如果未指定则会根据类型选择一个
      value: '',

      // 属性被改变时执行的函数（可选），也可以写成在 methods 段中定义的方法名字符串, 如：'_propertyChange'
      observer: function(newVal, oldVal, changedPath) {
         // 通常 newVal 就是新设置的数据， oldVal 是旧数据
      }
    },
    myPropertyB: String // 简化的定义方式
  },

  // 私有数据，可用于模版渲染
  data: {},

  // 生命周期函数，可以为函数，或一个在 methods 段中定义的方法名
   attached: function () { 
    this.selectComponent('.class-name', function(res) {
      console.log(res)
    })
  },
  ready: function() { },

  // 组件自定义方法
  methods: {
    onMyButtonTap: function(){
      this.setData({ ... })
    },

    _myPrivateMethod: function(){
      // 这里将 data.A[0].B 设为 'test'
      this.setData({
        'A[0].B': 'test'
      })
    },

    _propertyChange: function(newVal, oldVal) {

    }
  }
})
```

::: note
- 在 `properties` 定义中，属性名采用驼峰写法（`propertyName`）；
- 在 `ttml` 中，指定属性值时则对应使用连字符写法（`my-component property-name="value"`），应用于数据绑定时采用驼峰写法（`attr="{{propertyName}}"`）。
:::

## 使用 Component 构造器构造页面

小程序的页面也可以视为自定义组件。因而，页面也可以使用 Component 构造器构造，拥有与普通组件一样的定义与实例方法。但此时要求对应 `json` 文件中包含 `usingComponents` 定义段。

此时，组件的属性可以用于接收页面的参数，如访问页面 `/pages/index/index?paramA=123&paramB=xyz` ，如果声明有属性 `paramA` 或 `paramB` ，则它们会被赋值为 `123` 或 `xyz` 。

示例代码：

```js
{
  "usingComponents": {
  	"my-component" : "/pages/index/index?paramA=123&paramB=xyz"
  }
}
```

在组件代码中：

```js
Component({
  properties: {
    paramA: Number,
    paramB: String,
  },

  methods: {
    onLoad() {
      this.data.paramA // 页面参数 paramA 的值
      this.data.paramB // 页面参数 paramB 的值
    }
  }
})
```

::: note
- 此功能正在逐步支持中，可能会存在不符合预期的问题；
- setData 支持 path string 作为 key 值, 如‘data.A[0].B’
:::
