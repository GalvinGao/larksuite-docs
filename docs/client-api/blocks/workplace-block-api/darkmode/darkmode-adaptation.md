---
document_id: '7180269945547718661'
directory_id: '7180165099248205829'
title: DarkMode 适配
full_path: /uAjLw4CM/uYjL24iN/block/guide/darkmode
breadcrumb:
- Client API
- Blocks
- Workplace Block API
- DarkMode
- DarkMode adaptation
document_type: GuideDocumentType
updated_at: 2022-12-27T10:19:28Z
source_url: https://open.larksuite.com/document/uAjLw4CM/uYjL24iN/block/guide/darkmode
---

# DarkMode适配
## 开启 DarkMode
在 Block 的 `index.json` 中添加如下配置，将 `darkmode` 字段设置为 `true`，表示当前 Block 已支持 DarkMode。

```json
{
  "blockTypeID": "{此处填写blockTypeID}",
  "darkmode": true
}
```

## 主题相关事件


- [tt.onThemeChange](/document/uAjLw4CM/uYjL24iN/block/api/darkmode/onthemechange)：监听主题变化。
- [tt.offThemeChange](/document/uAjLw4CM/uYjL24iN/block/api/darkmode/offthemechange)：取消监听。
- [tt.getSystemInfo](/document/uAjLw4CM/uYjL24iN/block/api/device/getsysteminfo)：获取当前主题。

## DarkMode 样式适配
### 示例代码
#### TTML
```html
<view class="your-block">
  <view class="block-title">
    <text class="text-{{theme}}">Hello, Block ~! 主题色:{{theme}}</text>
  </view>
  <!--省略部分无关代码-->
</view>
```
#### TTSS
```css
.text-light{
  color: #1f1f1f;
}

.bg-light {
  background-color: #f5f6f7;
}

.text-dark {
  color: #F0F0F0;
}

.bg-dark {
  background-color: #101010;
}
```

#### JS
```js
Block({
  data: {
    theme: 'light'
  },
  onLoad(options) {
    //初次加载时获取当前系统主题模式
    tt.getSystemInfo({
      success: ({ theme }) => {
        //更新当前的主题模式变量
        this.setData({
          theme
        });
      }
    });

    //运行时监听当前系统主题模式变更
    tt.onThemeChange(({ theme }) => {
      this.setData({
        theme
      });
    });
  } 
  //省略部分无关代码
});
```
### 适配效果
:::html
<img width=50% src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/39a7706265d1e57df15a4c98c12d8337_1ZOO4K0AZw.png?lazyload=true&width=1640&height=903"/>


<img width=50% src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/77bd65e4d23a5e11d8283b5581df5385_bOQtxiBDzs.png?lazyload=true&width=1640&height=903"/>

:::

## 优化图片展示效果
为了防止用户生的成图片（如头像、新闻图片等）在暗色模式下的呈现视觉过曝，建议在暗色模式下，使用半透明暗色遮罩对图片亮度进行压暗处理。

**处理方法：** 在图片对应的展示组件（`<image>` 或 CSS `backgroud-image`属性）在暗色模式下增加一层半透明 `<view>` 遮罩。可参考下列示例代码。

### 示例代码
#### TTML
```html
<view class="your-block">
  <view class="image-wrap">
    <image class="info-image" src="https://example.com/image.jpg" />
    <view class="image-mask" tt:if="{{theme === 'dark'}}"/>
  </view>
  <!--省略部分无关代码-->
</view>
```
#### TTSS
```css
.image-wrap{
  padding: 15px;
  flex-direction: column;
  display: flex;
}

.info-image{
  height: 100px;
  border-radius: 4px;
}

.image-mask{
  height: 100px;
  display: relative;
  margin-top: -100px;
  background-color: rgba(0, 0, 0, 0.12);
  border-radius: 4px;
}
```

#### JS
```js
Block({
  data: {
    theme: 'light'
  },
  onLoad(options) {
    //初次加载时获取当前系统主题模式
    tt.getSystemInfo({
      success: ({ theme }) => {
        //更新当前的主题模式变量
        this.setData({
          theme
        });
      }
    });

    //运行时监听当前系统主题模式变更
    tt.onThemeChange(({ theme }) => {
      this.setData({
        theme
      });
    });
  } 

  //省略部分无关代码
});
```
### 优化效果

右侧为优化后图片亮度压暗的效果。

:::html
<img width=50% src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/fcb3182189c14f1f357ab1c7f4128778_sPJLyA74Jz.png?lazyload=true&width=1640&height=752"/>

<img width=50% src="//sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/35a8b2184bd36774eb95b924822bcb80_LHzy9jbAmF.png?lazyload=true&width=1640&height=752"/>
:::

## 兼容处理

如果开发者未对 Block 进行深色模式适配（即配置的 json 文件中未配置 `"darkmode": true`），为了保证小组件在深色模式下的可用性，小组件内容区域默认背景色会保持白色背景以确保组件内容整体可识读，具体表现如下图所示。为了保证最好的体验，请各位开发者及时适配 DarkMode 样式。
:::html
<img width="50%" src="https://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/b85579c1ae208cf107d551b45cedf0db_qJDBRY58L9.png?lazyload=true&width=1640&height=903?lazyload=true&width=1089&height=1080"/>
:::
