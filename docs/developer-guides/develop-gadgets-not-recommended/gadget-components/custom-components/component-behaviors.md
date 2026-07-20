---
document_id: '6965379541104918533'
directory_id: '6907567266540830722'
title: 组件行为 behaviors
full_path: /uYjL24iN/uMDMx4yMwEjLzATM
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Components
- Custom Components
- Component Behaviors
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMDMx4yMwEjLzATM
---

# 组件行为 behaviors

## 定义和使用 behaviors

`behaviors` 是用于组件间代码共享的特性，类似于一些编程语言中的 `mixins`。

每个 `behavior` 可以包含一组属性、数据、生命周期函数和方法，组件引用它时，它的属性、数据和方法会被合并到组件中，生命周期函数也会在对应时机被调用。每个组件可以引用多个 `behavior` 。 `behavior` 也可以引用其他 `behavior` 。

`behavior` 需要使用 `Behavior()` 构造器定义。

示例代码：

```js
module.exports = Behavior({
  behaviors: [],
  properties: {
    myBehaviorProperty: {
      type: String
    }
  },
  data: {
    myBehaviorData: {}
  },
  attached: function(){},
  methods: {
    myBehaviorMethod: function(){}
  }
})
```
组件引用时，在 `behaviors` 定义段中将它们逐个列出即可。

示例代码：

```js
var myBehavior = require('my-behavior')
Component({
  behaviors: [myBehavior],
  properties: {
    myProperty: {
      type: String
    }
  },
  data: {
    myData: {}
  },
  attached: function(){},
  methods: {
    myMethod: function(){}
  }
})
```

在上例中， `my-component` 组件定义中加入了 `my-behavior` ，而 `my-behavior` 中包含有 `myBehaviorProperty` 属性、 `myBehaviorData` 数据字段、 `myBehaviorMethod` 方法和一个 `attached` 生命周期函数。

这将使得 `my-component` 中最终包含 `myBehaviorProperty` 、 `myProperty` 两个属性， `myBehaviorData` 、 `myData` 两个数据字段，和 `myBehaviorMethod` 、 `myMethod` 两个方法。

当组件触发 `attached` 生命周期时，会依次触发 `my-behavior` 中的 `attached` 生命周期函数和 `my-component` 中的 `attached` 生命周期函数。

## 字段的覆盖和组合规则

组件和它引用的 `behavior` 中可以包含同名的字段，对这些字段的处理方法如下：

- 如果有同名的属性或方法，组件本身的属性或方法会覆盖 `behavior` 中的属性或方法，如果引用了多个 `behavior` ，在定义段中靠后 `behavior` 中的属性或方法会覆盖靠前的属性或方法；
- 如果有同名的数据字段，如果数据是对象类型，会进行对象合并，如果是非对象类型则会进行相互覆盖；
- 生命周期函数不会相互覆盖，而是在对应触发时机被逐个调用。如果同一个 `behavior` 被一个组件多次引用，它定义的生命周期函数只会被执行一次。

