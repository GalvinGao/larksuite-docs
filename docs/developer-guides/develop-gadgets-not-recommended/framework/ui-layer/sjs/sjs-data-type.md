---
document_id: '7073691561007792133'
directory_id: '7072701762536538117'
title: SJS 数据类型
full_path: /uYjL24iN/uEjMuEjMuEjM/sjs/sjs-data-type
breadcrumb:
- Developer Guides
- Develop Gadgets (Not Recommended)
- Framework
- UI Layer
- SJS
- SJS Data Type
document_type: GuideDocumentType
updated_at: 2022-03-11T04:11:31Z
source_url: https://open.larksuite.com/document/uYjL24iN/uEjMuEjMuEjM/sjs/sjs-data-type
---

# SJS 数据类型

sjs 中支持以下数据类型：

- 数值 `number`
- 字符串 `string`
- 布尔值 `boolean`
- 对象 `object`
- 函数 `function`
- 数组 `array`
- 日期 `date`
- 正则 `regexp`

以上数据类型的属性和方法与 ES5 基本相同，具体请参考 JavaScript 的用法

## 数据类型判断

准确判断数据类型的方法是根据 `constructor` 属性来判断

```javascript
var number = 10;
console.log("Number" === number.constructor);

var string = "str";
console.log("String" === string.constructor);

var boolean = true;
console.log("Boolean" === boolean.constructor);

var object = {};
console.log("Object" === object.constructor);

var func = function(){};
console.log("Function" === func.constructor);

var array = [];
console.log("Array" === array.constructor);

var date = getDate();
console.log("Date" === date.constructor);

var regexp = getRegExp();
console.log("RegExp" === regexp.constructor);
```

### typeof

另外使用 `typeof` 可以部分判断

```javascript
var number = 10;
var boolean = true;
var object = {};
var func = function () {};
var array = [];
var date = getDate();
var regexp = getRegExp();

console.log("number" === typeof number);
console.log("boolean" === typeof boolean);
console.log("object" === typeof object);
console.log("function" === typeof func);
console.log("object" === typeof array);
console.log("object" === typeof date);
console.log("object" === typeof regexp);

console.log("undefined" === typeof undefined);
console.log("object" === typeof null);
```

## Date 和 RegExp

SJS 不支持 new 关键字（new 运算），所以无法通过以下方式创建 date 和 regexp 类型

```javascript
new Date();
new RegExp("/d+/");
```

需要通过 `getDate` 和 `getRegExp` 方法获取

```javascript
getDate();
getDate(milliseconds);
getDate(datestring);
getDate(year, month[, date[, hours[, minutes[, seconds[, milliseconds]]]]]);

getRegExp(pattern[, flags]);
```
