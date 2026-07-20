---
document_id: '6965379543683137542'
directory_id: '6907567266536931329'
title: picker
full_path: /uYjL24iN/ucjNucjNucjN
breadcrumb:
- Client API
- Gadget Basic Components (Not Recommended)
- Form
- picker
document_type: GuideDocumentType
updated_at: 2021-05-23T07:10:01Z
source_url: https://open.larksuite.com/document/uYjL24iN/ucjNucjNucjN
---

# picker

从底部弹起的滚动选择器，现支持五种选择器，通过 mode 属性来设置，分别是普通选择器，多列选择器，时间选择器，日期选择器，省市区选择器，默认是普通选择器。

## mode = selector

普通选择器。

|属性名|类型|默认值|说明|
|----|----|-----|---|
|range|Array/Object Array|[ ]|mode 为 selector 或 multiSelector 时，range 有效|
|range-key|String||当 range 是一个 Object Array 时，<br>通过 range-key 来指定 Object 中 key 的值作为选择器显示内容|
|value|Number|0|value 的值表示选择了 range 中的第几个（下标从 0 开始）|
|bindchange|EventHandler||value 改变时触发 change 事件，event.detail = {value: value}|
|disabled|Boolean|false|是否禁用|
|bindcancel|EventHandler||取消选择收起 picker 时触发|

## mode = multiSelector

多列选择器。

|属性名|类型|默认值|说明|
|-----|---|-----|---|
|range|二维Array/Object Array|[ ]|mode 为 selector 或 multiSelector 时，range 有效。<br>二维数组，长度表示多少列，<br>数组的每项表示每列的数据，如 [["a","b"], ["c","d"]]|
|range-key|String||同普通选择器|
|value|Array|[]|value 每一项的值表示选择了 range 对应项中的第几个<br>（下标从 0 开始）|
|bindchange|EventHandler||value 改变时触发 change 事件，<br>event.detail = {value: value}|
|bindcolumnchange|EventHandler||某一列的值改变时触发 columnchange 事件，<br>event.detail = {column: column, value: value}，<br>column 的值表示改变了第几列（下标从0开始），<br>value 的值表示变更值的下标|
|bindcancel|EventHandler||取消选择时触发|
|disabled|Boolean|false|是否禁用|

## mode = time

时间选择器。

|属性名|类型|默认值|说明|
|-----|---|-----|---|
|value|String||表示选中的时间，格式为"hh:mm"|
|start|String||表示有效时间范围的开始，字符串格式为"hh:mm"|
|end|String||表示有效时间范围的结束，字符串格式为"hh:mm"|
|bindchange|EventHandler||value 改变时触发 change 事件，event.detail = {value: value}|
|bindcancel|EventHandler||取消选择时触发|
|disabled|Boolean|false|是否禁用|

## mode = date

日期选择器。

|属性名|类型|默认值|说明|
|-----|--|------|---|
|value|String||表示选中的日期，格式为"YYYY-MM-DD"|
|start|String||表示有效日期范围的开始，字符串格式为"YYYY-MM-DD" **PC端暂不支持**|
|end|String||表示有效日期范围的结束，字符串格式为"YYYY-MM-DD" **PC端暂不支持**|
|fields|String|day|有效值 year,month,day，表示选择器的粒度|
|bindchange|EventHandler||value 改变时触发 change 事件，event.detail = {value: value}|
|bindcancel|EventHandler||取消选择时触发|
|disabled|Boolean|false|是否禁用|

## mode = region

地区选择器。

::: note 
**PC端暂不支持**
:::

|属性名|类型|默认值|说明|
|-----|--|------|---|
|value|Array||表示选中的省市区，默认选中每一列的第一个值|
|custom-item|String||可为每一列的顶部添加一个自定义的项|
|bindchange|EventHandler||value 改变时触发 change 事件，<br>event.detail = {value: value, code: code}, <br>其中字段code是统计用区划代码|
|bindcancel|EventHandler||取消选择时触发|
|disabled|Boolean|false|是否禁用|


示例代码：

```html
<view class="section__title">普通选择器</view>
<picker value="{{index}}" range="{{array}}"
    bindchange="bindPickerChange" bindcancel="bindPickerCancel">
  <view class="picker">
    当前选择：{{array[index]}}
  </view>
</picker>

<view class="section__title">普通Object选择器</view>
<picker bindchange="bindObjectPickerChange"
    value="{{objectIndex}}" range="{{objectArray}}" range-key="en">
  <view class="picker">
    当前选择：{{objectArray[objectIndex]['en']}}
  </view>
</picker>

<view class="section__title">多列选择器</view>
<picker mode="multiSelector"
    bindchange="bindMultiPickerChange"
    bindcolumnchange="bindMultiPickerColumnChange"
    value="{{multiIndex}}" range="{{multiArray}}">
  <view class="picker">
    当前选择：{{multiArray[0][multiIndex[0]]}}，
    {{multiArray[1][multiIndex[1]]}}，{{multiArray[2][multiIndex[2]]}}
  </view>
</picker>

<view class="section__title">时间选择器</view>
<picker mode="time" value="{{time}}" start="{{timeStart}}" end="{{timeEnd}}"
    bindchange="bindTimeChange">
  <view class="picker">
    当前选择: {{time}}
  </view>
</picker>

<view class="section__title">日期选择器</view>
<picker mode="date" value="{{date}}" start="2015-09-01" end="2017-09-01"
    bindchange="bindDateChange" fields="year">
  <view class="picker">
    当前选择: {{date}}
  </view>
</picker>

<view class="section__title">日期选择器</view>
<picker mode="date" value="{{date}}" start="2015-09-01" end="2017-09-01"
    bindchange="bindDateChange" fields="month">
  <view class="picker">
    当前选择: {{date}}
  </view>
</picker>

<view class="section__title">日期选择器</view>
<picker mode="date" value="{{date}}" start="2015-09-01" end="2017-09-01"
    bindchange="bindDateChange" fields="day">
  <view class="picker">
    当前选择: {{date}}
  </view>
</picker>
```

```js
var ms = [
    [// 0
        '无脊柱动物',
        '脊柱动物'
    ],
    [// 1
        [// 1 0
            '扁性动物',
            '线形动物',
            '环节动物',
            '软体动物',
            '节肢动物'
        ],
        [// 1 1
            '鱼',
            '两栖动物',
            '爬行动物',
      'test'
        ]
    ],
    [// 2
        [// 2 0
            ['猪肉绦虫', '吸血虫'],
            ['蛔虫'],
            ['蚂蚁', '蚂蟥'],
            ['河蚌', '蜗牛', '蛞蝓'],
            ['昆虫', '甲壳动物', '蛛形动物', '多足动物']
        ],
        [// 2 1
            ['鲫鱼', '带鱼'],
            ['青蛙', '娃娃鱼'],
            ['蜥蜴', '龟', '壁虎'],
      []
        ]
    ]
];
Page({
    data: {
        array: ['美国', '中国', '巴西', '日本'],
        index: 0,
        objectArray: [
            {
                id: 0,
                name: '美国',
                en: 'USA'
            },
            {
                id: 1,
                name: '中国',
                en: 'China'
            },
            {
                id: 2,
                name: '巴西',
                en: "Brasil"
            },
            {
                id: 3,
                name: '日本',
                en: "Japan"
            }
        ],
        objectIndex: 0,
        multiArray: [
            ['无脊柱动物', '脊柱动物'],
            ['扁性动物', '线形动物', '环节动物', '软体动物', '节肢动物'],
            ['猪肉绦虫', '吸血虫']
        ],
        multiIndex: [0, 0, 0],
        objectMultiIndex: [0,0,0],
        date: '2016-09-01',
        time: '12:01',
        timeStart: '09:01',
        timeEnd: '21:01'
    },
    bindPickerChange: function (e) {
        console.log('picker发送选择改变，携带值为', e.detail.value)
        this.setData({
            index: e.detail.value
        })
    },
    bindObjectPickerChange: function (e) {
        console.log('picker发送选择改变，携带值为', e.detail.value)
        this.setData({
            objectIndex: e.detail.value
        })
    },
    bindMultiPickerChange: function (e) {
        console.log('picker发送选择改变，携带值为', e.detail.value)
        this.setData({
            multiIndex: e.detail.value
        })
    },
    bindMultiPickerColumnChange: function (e) {
        // return;
        console.log('修改的列为', e.detail.column, '，值为', e.detail.value);
        var data = {
            multiArray: this.data.multiArray,
            multiIndex: this.data.multiIndex
        };
        switch (e.detail.column) {
            case 0:
                data.multiIndex[0] = e.detail.value;
                data.multiIndex[1] = 0;
                data.multiIndex[2] = 0;

                data.multiArray[1] = ms[1][data.multiIndex[0]];
                data.multiArray[2] = ms[2][data.multiIndex[0]][data.multiIndex[1]];
                break;
            case 1:
                data.multiIndex[1] = e.detail.value;
                data.multiIndex[2] = 0;

                data.multiArray[2] = ms[2][data.multiIndex[0]][data.multiIndex[1]];
                break;
            case 2:
                data.multiIndex[2] = e.detail.value;
                break;
        }
        this.setData(data);
    },
    bindDateChange: function (e) {
        console.log('picker发送选择改变，携带值为', e.detail.value)
        this.setData({
            date: e.detail.value
        })
    },
    bindTimeChange: function (e) {
        console.log('picker发送选择改变，携带值为', e.detail.value)
        this.setData({
            time: e.detail.value
        })
    },
    bindPickerCancel: function (e) {
        console.log(e);
        tt.showToast({title: 'cancel'})
    }
})
```
<br>
![图片名称](http://sf16-sg.larksuitecdn.com/obj/open-platform-opendoc-sg/ark/79e005cbb1bf944f99857b17ba0b5dce_15.png)
