---
document_id: '7180270043522318342'
directory_id: '7179507279661842438'
title: font-family
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-family
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- text
- font-family
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/font-family
---

# font-family

## 介绍

通过给定一个有先后顺序的，由字体名或者字体族名组成的列表来为选定的元素设置字体。

## 语法

```css
.font {
  font-family: 'my-fontface';
}

@font-face {
  font-family: 'my-fontface';
  src: url(data:...;base64base64编码的字体...) local('Monosspace') url('https://www.runoob.com/try/demo_source/Sansation_Light.ttf');
}
```

### 取值

-   `<string>`

引用字体名称，目前不支持`serif`、`sans-serif`等通用字族。
