---
document_id: '6965379543683547142'
directory_id: '6907567266536931329'
title: editor
full_path: /uYjL24iN/uMTM4QjLzEDO04yMxgDN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Form
- editor
document_type: GuideDocumentType
updated_at: 2022-07-06T07:38:46Z
source_url: https://open.larksuite.com/document/uYjL24iN/uMTM4QjLzEDO04yMxgDN
---

# editor

富文本编辑器，可以对图片，文字进行编辑。<br>
编辑器导出内容支持带标签的 html，纯文本的 text 以及自定义的 json 格式。<br>

## 功能兼容版本列表

> 版本默认为Lark版本

|功能|移动端|PC端|备注|
|---|-----|---|----|
|Editor 组件|≥ 3.4|≥ 3.12||
|@ 能力|≥ 3.7| ≥ 3.12 |@ 功能中的搜索能力需要调用 tt.login 之后才可以使用|
|右键快捷操作| - |≥ 3.47 |快捷操作包含加粗、倾斜、删除线|

## 属性


| 属性名         | 类型           | 默认值     | 说明         | 最低版本 |
| --------- | --------------- | -------   | ----------- | --- |
|contents | `JSON` | 无 | 输入框内容，支持text，html，json，优先html展示 | 移动端3.4，PC端3.12 |
|placeholder| `String` | 无 |当编辑器内容为空的时候的提示信息| 移动端3.4，PC端3.12 |
|read-only| `Boolean` |  false | 是否是只读模式，只读模式下，编辑器组件将无法编辑| 移动端3.4，PC端3.12 |
|plugins|`Array[string]`| [] | 可动态配置工具栏功能 | 移动端3.7，PC端3.12 |
|placeholderStyle|`Object`| 无 | placeholder 样式 | 移动端3.7，PC端不支持 |
|auto-height|`Boolean`| false | 高度自适应 | 3.34 |
|supportMarkdown|`Boolean`| true | 输入时支持 markdown 语法 | 5.5 |
|supportAttribution|`Boolean`| true | 输入、粘贴时支持富文本格式，**值为flase时，顶部栏中文本格式编辑的功能会隐藏** | 5.12 |
|supportInsertImage|`Boolean`| true | 支持图片上传、粘贴，**值为flase时，顶部栏中的插入图片功能会隐藏**| 5.12 |
|bindready| `EventHandler` | 无 | 编辑器初始化完成时的触发 | 移动端3.7，PC端3.12 |
|bindinput| `EventHandler` | 无 | 编辑器内容变化时触发| 移动端3.7，PC端3.12 |
|bindinsertimage|`EventHandler`| 无 |插入图片时，小程序需要实现的上传图图片回调 | 移动端3.4，PC端3.15 |
|bindmentionselect|`EventHandler`| 无 |默认 at 面板选择联系人时触发 | 移动端 3.8，PC端3.12 |
|bindmentionclick|`EventHandler`| 无 |点击@xxx标签时触发，**PC端悬浮触发**| 移动端 3.8，PC端3.12|
|bindgetfileinfo| `EventHandler` | 无 |用于文档链接的识别，输入或粘贴链接时，小程序需要实现的获取文档标题回调| 3.34 |
|bindeditorclick| `EventHandler` | 无  |点击编辑器时的回调| 3.34 |
|bindatfinder| `EventHandler` | 无  |点击 at 人或者输入 at 的回调，用于业务方自定义 at 面板（有这个事件默认 at 面板不再弹出）| 移动端 3.4， PC 5.2 |

:::note
注意事项：
- contents 属性内 json 不支持传入 {}，传入会报错 
- 组件样式默认存在 min-height，若要调整高度需要在 style 内传入 min-height 以覆写
- 在 PC 端，如需使用 “@能力”，推荐设置为自适应高度（auto-height=true）；当 editor 选择固定高度且高度不足，@ 面板将无法完整显示；该问题可通过 `bindatfinder `自定义 at 面板解决。
:::
plugins元素描述：
| 属性名         | 类型            | 说明         | 支持版本 | 备注 |
| --------- | -----------   | ----------- |  --- | -- |
|attribution|String|字体配置| 移动端3.4，PC端3.12 | |
|insertImage|String|插入图片|移动端3.4，PC端3.12| |
|mention|String|@功能|移动端3.7，PC端3.12| |
|undo|String|撤回键入|3.34| |
|redo|String|恢复键入|3.34| |
|indentRight|String|向右缩进|3.34| |
|indentLeft|String|向左缩进|3.34| |

placeholderStyle元素描述：
| 属性名         | 类型           | 默认值     | 说明         | 
| --------- | --------------- | -------   | ----------- | 
|color|String|#C0C0C0|颜色|
|fontFamily|String|sans-serif|字体|
|fontSize|String|15px|字号|
|lineHeight|Number|1.5|高度|
|opacity|Number|0.5|不透明度|


## 代码示例
### 整体示例
:::html

<md-code page="page/component/pages/editor/editor">

<md-code-item type="TTML">

```html
<view class="container">
    <editor
        style="height:500px"
        id="editor"
        placeholder="{{placeholder}}"
        contents="{{contents}}"
        read-only="{{readOnly}}"
        plugins="{{plugins}}"
        placeholderStyle="{{placeholderStyle}}"
        bindready="onEditorReady"
        bindmentionselect = "onMentionSelect"
        bindmentionclick = "onMentionClick"
        bindinput="onEditorInputValueChange"
        bindinsertimage="onInsertImages">
    </editor>
</view>
```

</md-code-item>

<md-code-item type="JS">

```js
Page({
    data: {
        placeholder: 'Hello editor!',
        readOnly: false,
        contents: {
            html: `<div id="magicdomid-1_19" class="ace-line align-center heading-h1  locate lineguid-B2nSe8" dir="auto"><span class=" ">Lark</span></div><div id="magicdomid-1_26" class="ace-line blockquote blockquote  locate lineguid-17mqsc" dir="auto"><span class=" ">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Lark是字节跳动旗下办公平台，整合即时沟通、日历、音视频会议、云文档、云盘、工作台等功能于一体，成就组织和个人，更高效、更愉悦。</span></div><div id="magicdomid-1_27" class="ace-line locate lineguid-6UxLlZ" dir="auto"><span class=" ">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 2020年2月24日，字节跳动旗下办公套件Lark宣布，向全国所有企业和组织免费开放，不限规模，不限使用时长，所有用户均可使用Lark全部套件功能。2020年11月18日，Lark在北京举办“2020Lark未来无限大会”。会上，Lark推出全新版本“π”，发布独立App“Lark文档”，并在视频会议、即时沟通等功能上宣布了重大更新。</span></div><div id="magicdomid-1_37" class="ace-line locate lineguid-AWIi9E" dir="auto"><span class=" ">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span><span class=" backgroundcolor " style="background-color: #FFF362; ">公众评价：小米选用了Lark，目前已经有一段时间了，Lark在信息创建、分享，以及协同办公方面，非常简洁、高效，的确越用越顺手。 ​​​（小米公司创始人、董事长兼CEO 评价）</span></div><div id="magicdomid-1_38" class="ace-line" dir="auto"><br></div><div id="magicdomid-1_34" class="ace-line" dir="auto"><br></div><div id="magicdomid-1_35" class="ace-line image-upload single-line" dir="auto"><span><div contenteditable="false" class="image-container single-elem-cls"><span class="image-wrapper"><span class="point tl n-icon-dragable"></span><span class="point tr n-icon-dragable"></span><span class="point br n-icon-dragable"></span><span class="point bl n-icon-dragable"></span><img src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/37338d385a39166169f6bfeaaee5b32f_B6WlUG5Z3o.png" data-faketext=" " data-uuid="LxW0dlVX" class="editor_image" style="width: 88px;height: 88px"></span></div></span></div><div id="magicdomid-1_36" class="ace-line" dir="auto"><br></div>`
        },
        plugins: ['attribution', 'mention', 'undo', 'redo','indentRight','indentLeft'],
        placeholderStyle: {
            color: '#FFFD00',
            fontSize:"25px"
        }
    },
    onLoad: function () {

    },
    onShow: function () {

    },
    onHide: function () {
    },

    onEditorReady: function (res) {
        console.log('onEditorReady '  + JSON.stringify(res))
    },
    onEditorInputValueChange: function(res) {
        console.log('onEditorInputValueChange '  + JSON.stringify(res))
    },

    onMentionSelect: function(res) {
        console.log('onMentionSelect ' + JSON.stringify(res))
    },

    onMentionClick: function(res) {
        console.log('onMentionClick ' + JSON.stringify(res))
    },

    onInsertImages: function (res) {
        console.log('onInsertImages '  + JSON.stringify(res));
        const images = res.detail.images.map(item => ({
            ...item,
            src: item.filePath,
        }))
        res.insertImagesCallback({ images });
    }
})

```

</md-code-item>

<md-code-item type="JSON">

```json
{
    "navigationBarTitleText": "editor"
}
```

</md-code-item>

</md-code>

:::

### 回调方法示例
#### bindinput
```html
<editor bindinput="onValueChange"  />
```
```js
// ...
onValueChange: function(res) {
	const { html, text, json } = res.detail;
    // html：编辑器数据的html
    // text：编辑器的文字内容
    // json：编辑器数据的底层数据结构
}
```
#### bindready
```html
<editor bindready="onReady"  />
```
```js
// ...
onReady: function() {
	// 编辑器初始化完成后的回调，建议数据在此方法回调后再进行注入
}
```
#### bindinsertimage
```html
<editor bindinsertimage="onInsertImage"  />
```
```js
// ...
onInsertImage: function(res) {
	const { detail, insertImagesCallback } = res;
	const { images } = detail;
 	  // images 为一个数组，每项为 { filePath, width, height }，PC端无width/height属性
    const uploadedImages =images.map(item => {
        // 业务方可以自行处理图片，返回一个 src 字段
        return {
        	src: item.filePath,
        }
    })
    // 调用res中的回调函数，将数据传回
    insertImagesCallback({ images: uploadedImages });
}
```
#### bindmentionselect
```html
<editor bindmentionselect="onMentionSelect"  />
```
```js
// ...
onMentionSelect: function(res) {
	const { name, token, ...others } = res;
    // 默认 at 面板 token 为 openid
}
```
#### bindmentionclick
```html
<editor bindmentionclick="onMentionClick"  />
```
```js
// ...
onMentionClick: function(res) {
	const { openId, rect, ...others } = res;
    // 默认 at 面板 openId 为 真实openId，业务自定义 at 面板则由业务传入的 token 决定
    // rect 用于 PC 端 at 标签的位置，便于业务方将 profile 渲染在该位置
}
```
#### bindgetfileinfo
```html
<editor bindgetfileinfo="onGetFileInfo"  />
```
```js
// ...
onGetFileInfo: function(res) {
	const { detail, getFileInfoCallback } = res;
	const { url } = detail;
	// 业务拿到 url 后，需要展示的名称在 title 字段传回
    getFileInfoCallback({ title });
}
```
#### bindeditorclick
```html
<editor bindeditorclick="onEditorClick"  />
```
```js
// ...
onEditorClick: function(res) {
	const { detail } = res;
	const { nodeName, nodeType, value } = detail;
    // PC 端 a 标签与 img 标签需要监听此方法自行
    // 移动端点击 a 标签会自动跳转（需要在开发者后台 - 安全设置 - 小程序 Schema 白名单内配置）
    // 移动端点击 img 标签会进行预览
}
```
#### bindatfinder
```html
<editor bindatfinder="onAtFinder"  />
```
```js
// ...
onAtFinder: function (res) {
    // 调起自定义的人员选择
    // 选完后将人员数据当做参数回调组件
    // 传入该回调后 mentionselect 便不会触发
    res.atFinderCallback(users);
    // users数据结构
    // { data: { result_list: [{ token: string; content: string; type: 0; ...others }] } }
    // 其中 content 应为 btoa(unescape(encodeURIComponent(需要显示的名称))) bota建议polyfill
    // token为传入的id，用于 mentionclick 回调数据
    // type目前必须固定为0，表示选择的人员
}
```





