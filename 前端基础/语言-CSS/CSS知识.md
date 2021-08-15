### 页面上隐藏元素的方法

##### 占位:

- `visibility: hidden;`
- `margin-left: -100%;`
- `opacity: 0;`
- `transform: scale(0);`

##### 不占位:

- `display: none;`
- `width: 0; height: 0; overflow: hidden;`

仅对块内文本元素:

- `text-indent: -9999px;`
- `font-size: 0;`

### CSS继承属性

- 文本类型

- 字体类型
- visitability
- cursor属性
- list-style属性

### 多行文本省略

```css
display: -webkit-box;
-webkit-box-orient: vertical;
-webkit-line-clamp: 2;
overflow: hidden;
text-ellipsis: ellipsis;
```

### CSS文档流脱离

一共有三个属性会使元素脱离文档流：

- 浮动，但是浮动还会占据原来的位置。对所有的元素都设置了浮动，你会看到这几个div并不会重叠，而是会顺序排列

- 绝对定位，不占据原来的位置

- 固定定位，不占据原来的位置

元素脱离文档流会导致父元素高度塌陷，要修复高度塌陷问题，可以将子元素与父元素放在同一个BFC中

解决高度塌陷的方法就是清除浮动

### CSS伪类

- :first-child 选择某个元素的第一个子元素；  
- :last-child 选择某个元素的最后一个子元素；
- :nth-child() 选择某个元素的一个或多个特定的子元素；
- :nth-last-child() 选择某个元素的一个或多个特定的子元素，从这个元素的最后一个子元素开始算；
- :nth-of-type() 选择指定的元素；
- :nth-last-of-type() 选择指定的元素，从元素的最后一个开始计算;
- :first-of-type 选择一个上级元素下的第一个同类子元素；
- :last-of-type 选择一个上级元素的最后一个同类子元素；
- :only-child 选择的元素是它的父元素的唯一一个子元素；
- :only-of-type 选择一个元素是它的上级元素的唯一一个相同类型的子元素；
- :empty 选择的元素里面没有任何内容。
- :checked匹配被选中的input元素，这个input元素包括radio和checkbox。
- :default匹配默认选中的元素，例如：提交按钮总是表单的默认按钮。
- :disabled匹配禁用的表单元素。
- :enabled匹配没有设置disabled属性的表单元素。
- :valid匹配条件验证正确的表单元素。

### CSS伪元素

常见的伪元素选择器：

- ::first-letter 选择元素文本的第一个字（母）。
- ::first-line 选择元素文本的第一行。
- ::before 在元素内容的最前面添加新内容。
- ::after 在元素内容的最后面添加新内容。
- ::selection匹配用户被用户选中或者处于高亮状态的部分
- ::placeholder匹配占位符的文本，只有元素设置了placeholder属性时，该伪元素才能生效

### div盒子 宽度未知,怎么实现宽高比2:1

高度设0。用padding垂直方向百分比，比如padding-top:50%撑开。 因为margin和padding垂直方向设置百分比是依据自身宽度的。

