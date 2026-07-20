---
document_id: '7180269945548324869'
directory_id: '7179507279661842438'
title: white-space
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/white-space
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- View Layer
- TTSS
- Attributes
- text
- white-space
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:41Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/view-layer/ttss/attributes/text/white-space
---

# white-space

## 介绍

设置如何处理元素中的空白

## 语法

```css
white-space: normal;

white-space: nowrap;
```

### 取值

-   `normal`

连续的空白符会被合并，换行符会被当作空白符来处理。

-   `nowrap`

连续的空白符会被合并，但文本内的换行无效。

## 标准化语法

```css
white-space: normal | nowrap
```

## 例子

```html
<view style="width: 100%; height: 100%;flex-direction: column;">

  <text class="title">white-space: normal</text>

  <text class="item normal">
    But ere she from the church-door stepped She smiled and told us why: 'It was a wicked woman's curse,' Quoth she, 'and what care I?' She
    smiled, and smiled, and passed it off Ere from the door she stept—</text>
  <text class="title">white-space: nowrap</text>
  <text class="item nowrap">
    But ere she from the church-door stepped She smiled and told us why: 'It was a wicked woman's curse,' Quoth she, 'and what care I?' She
    smiled, and smiled, and passed it off Ere from the door she stept—</text>
</view>
```

```css
.title {
  width: 100%;
  height: 20px;
  margin-top: 15px;
  color: #2d8cf0;
}

.item {
  width: 100%;
  height: 100px;
  border-color: red;
  border-width: 1px;
  border-style: solid;
}

.normal {
  white-space: normal;
}

.nowrap {
  white-space: nowrap;
}
```
