---
document_id: '6965379541105049605'
directory_id: '6907567266540830722'
title: 组件扩展
full_path: /uYjL24iN/uUDMx4SNwEjL1ATM
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Components
- Custom Components
- Component Extension
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:59Z
source_url: https://open.larksuite.com/document/uYjL24iN/uUDMx4SNwEjL1ATM
---

# 组件扩展

::: note
可用版本: 1.19.0+
:::

为了更好的理解扩展后的效果，先举一个例子。

在 `behavior.js` 中：

```js
module.exports = Behavior({
  definitionFilter(defFields) {
    defFields.data.from = 'behavior'
  },
})
```

在 `component.js` 中：
```
Component({
  data: {
    from: 'component'
  },
  behaviors: [require('behavior.js')],
  ready() {
    // 此处会发现输出 behavior 而不是 component
    console.log(this.data.from)
  }
})
```

通过例子可以发现，自定义组件的扩展其实就是提供了修改自定义组件定义段的能力，上述例子就是修改了自定义组件中的 `data` 定义段里的内容。
