---
document_id: '7180269945548292101'
directory_id: '7179507279661760518'
title: 注册 Creator
full_path: /uAjLw4CM/uYjL24iN/block/block-frame/logic-layer/register-creator
breadcrumb:
- Developer Guides
- Develop Workplace Blocks
- Block Framework
- Code components and structure
- Logic Layer
- Register Creator
document_type: GuideDocumentType
updated_at: 2022-12-27T10:18:42Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/block-frame/logic-layer/register-creator
---

# 注册 Creator

Creator 的主要功能是创建小组件应用的元信息（SourceMeta）和源数据（SourceData），并生成小组件应用的实体数据（BlockEntity）。
<br>
如下代码展示了如果使用 Creator 构造器注册一个 Creator。Creator 构造器的核心是基于 `tt.setBlockInfo` 接口创建小组件应用的元信息。

```javaScript
Creator({
  onLoad(options) {
    // option 1: no need user input, fetch meta by network directly
    tt.request({
      url: '',
      success(res) {
        tt.setBlockInfo({
          sourceMeta: res.meta,
        });
      },
    });
  },
  onShow() {
      // 视图可见时执行
  },
  onHide() {
      // 视图隐藏时执行
  },
  onReady() {
      // 视图渲染完成时执行
  },
  onDestroy() {
      // Creator销毁时执行
  },
  methods: {
    createBlock() {
      // option 2: user submit some input
      tt.setBlockInfo({
        sourceMeta: meta,
      });
    },
    cancelBlock() {
      tt.cancel();
    },
  },
});
```

