---
document_id: '6967331158355525638'
directory_id: '6907567266536685569'
title: 样式
full_path: /uYjL24iN/uYTN04iN1QjL2UDN
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Gadget Design Specification
- Visual Specifications
- Style
document_type: GuideDocumentType
updated_at: 2022-07-04T07:16:00Z
source_url: https://open.larksuite.com/document/uYjL24iN/uYTN04iN1QjL2UDN
---

# 样式
## 字阶与行高
字阶在无形中区分了内容的主次，决定了内容的节奏与秩序之美。行高决定了文字的留白空间，配合字阶展示字体的最佳视觉效果，如无特殊情况请严格按照以下字号行高规范使用。

### 桌面端文字
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/747fac56a30e94b1be8975d65520d278_b11D4Z7knu.png?lazyload=true&width=3840&height=2240)

### 移动端文字
![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b3136c7248ad7fcac65085214d16b5d2_ShPQtpcXWC.png?lazyload=true&width=3840&height=2364)


## 间距
间距是在布局和组件中放置元素的方法。在定义间距时，需要注意到元素之间的亲密关系，
在设计上，以  4px 为元素单位进行扩展。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/69ac500a713b85a83f0f8001ae6a896f_0VNBf6oM98.png?lazyload=true&width=1920&height=680)

## 颜色
颜色在小程序应用程序中扮演着重要角色。它传达内容的重要性和关联，并为用户提供指导，帮助他们完成任务。我们将小程序组件库提供的色板分为：功能色、中性色，其中中性色针对文字、图标、背景、描边、分割线以及交互状态提供颜色规范，可以根据实际场景选择合适的颜色，确保呈现给用户的信息具有足够的视觉对比度。

### 功能色
功能色用来标识界面中的信息状态，在选取时需要遵循用户的认知习惯。在相同产品体系下需要保持一致性，在选取时需要遵循用户的认知习惯，例如: 语义红色代表错误、橙色代表警示、绿色代表成功、蓝色代表强调/可点击文字链等。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/34d211593ef91b5eece6a7811ed195e1_C8pokGUvxA.png?lazyload=true&width=1920&height=616)

### 中性色
中性色主要应用于文字与通用元素部分，如背景、描边、分割线等。选择中性色时需要考虑深、浅背景下的明度对比差异性。合理地选择中性色能够令页面信息具备良好的主次关系，保障界面元素的可识别性。Lark 的灰阶色板一共包含了从白到黑的14个颜色，其中黑、白为无彩色，灰色中融入一定量蓝色，能为界面提供更好的阅读体验。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/edb824c4f7c56dab9692cbe9ab425980_1T2ioq0Ikb.png?lazyload=true&width=3680&height=1120)

#### 文字颜色
同一页面文字颜色不应超过三种，如遇到特殊场景，最多增加到四种。以下为不同类型文本配色的使用规范：
- 一级标题、正文使用 N900 
- 二级标题、二级正文使用 N600
-  输入框引导语、提示文字、辅助信息使用 N500
- 在灰/白色背景下的文字链 Disable 状态使用 N400

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/25f3dce6ee33202f7e633e291cf18b9e_GBTLV80gbX.png?lazyload=true&width=1920&height=616)
#### 图标颜色
图标颜色需要有明确的区分度，根据场景与功能类型的不同，使用4个色阶进行区分，使用规则如下：
- 一级图标使用 N800
- 二级图标使用 N600
- 三级图标、表意图标、 tab 未选中图标使用 N500
- Disable 状态使用 N400 

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/a855bdc2221c91700e7739f0a828f6b3_8j85sINXEw.png?lazyload=true&width=1920&height=616)

#### 背景颜色
背景色用于协调界面层级关系，常规模式下的背景色值建议限定为 N50、N100、N200 三种。

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/2c249c5f4d7b0c5ea1bc17e822468e53_nGujnWNKoK.png?lazyload=true&width=1920&height=616)

#### 描边颜色
- 可交互控件（如输入框，选择器等）描边颜色N400
- 卡片描边颜色N300

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b93b00958be4bcedf0232588eccf89c0_U2vKEZKO7R.png?lazyload=true&width=1920&height=616)

#### 分割线和交互状态色
- 分割线：N900，15%透明度
- Hover态：N900，10%透明度
- Press按压态：N900，15%透明度

![image.png](//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/d216f4663317055cabf31539f15014eb_pRjLLpZsMe.png?lazyload=true&width=2360&height=616)

