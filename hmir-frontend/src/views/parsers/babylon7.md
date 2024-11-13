此文档详细描述了 `@babel/parser`（前身为 babylon）所解析的抽象语法树（AST）节点类型。这些节点类型是 JavaScript 代码在被解析成 AST 后的具体表示形式，对于编译器、代码分析工具等来说非常重要。

此文档根据 Babel AST 格式(https://github.com/babel/babel/blob/main/packages/babel-parser/ast/spec.md) 翻译

#### Node Objects

所有 AST 节点都实现了以下接口：

```typescript
interface Node {
  type: string;
  loc: SourceLocation | null;
}
```

- **type**: 字符串，表示节点的类型。
- **loc**: `SourceLocation` 对象，包含节点在源代码中的位置信息（起始和结束位置），或者为 `null`。

#### Identifier

表示变量名或属性名。

```typescript
interface Identifier extends Node {
  type: "Identifier";
  name: string;
}
```

- **name**: 字符串，表示标识符的名称。

#### PrivateName

表示类中的私有成员名称。

```typescript
interface PrivateName extends Node {
  type: "PrivateName";
  id: Identifier;
}
```

- **id**: `Identifier` 对象，表示私有成员的名称。

#### Literals

##### RegExpLiteral

正则表达式字面量。

```typescript
interface RegExpLiteral extends Node {
  type: "RegExpLiteral";
  pattern: string;
  flags: string;
}
```

- **pattern**: 字符串，表示正则表达式的模式。
- **flags**: 字符串，表示正则表达式的标志（如 `g`, `i` 等）。

##### NullLiteral

`null` 值。

```typescript
interface NullLiteral extends Node {
  type: "NullLiteral";
}
```

##### StringLiteral

字符串字面量。

```typescript
interface StringLiteral extends Node {
  type: "StringLiteral";
  value: string;
}
```

- **value**: 字符串，表示字符串的值。

##### BooleanLiteral

布尔值字面量。

```typescript
interface BooleanLiteral extends Node {
  type: "BooleanLiteral";
  value: boolean;
}
```

- **value**: 布尔值，表示布尔字面量的值。

##### NumericLiteral

数字字面量。

```typescript
interface NumericLiteral extends Node {
  type: "NumericLiteral";
  value: number;
}
```

- **value**: 数字，表示数字字面量的值。

##### BigIntLiteral

大整数字面量。

```typescript
interface BigIntLiteral extends Node {
  type: "BigIntLiteral";
  value: string;
}
```

- **value**: 字符串，表示大整数字面量的值。

##### DecimalLiteral

十进制字面量。

```typescript
interface DecimalLiteral extends Node {
  type: "DecimalLiteral";
  value: string;
}
```

- **value**: 字符串，表示十进制字面量的值。

#### Programs

包含整个程序的顶级结构。

```typescript
interface Program extends Node {
  type: "Program";
  body: Array<Statement | ModuleDeclaration>;
  sourceType: "script" | "module";
}
```

- **body**: 一个包含 `Statement` 或 `ModuleDeclaration` 的数组，表示程序的主体部分。
- **sourceType**: 字符串，表示源代码的类型，可以是 `"script"` 或 `"module"`。

#### Functions

函数定义。

```typescript
interface Function extends Node {
  type: "Function";
  id: Identifier | null;
  params: Array<Pattern>;
  body: BlockStatement;
  generator: boolean;
  async: boolean;
}
```

- **id**: `Identifier` 对象或 `null`，表示函数的名称。
- **params**: 一个包含 `Pattern` 的数组，表示函数的参数。
- **body**: `BlockStatement` 对象，表示函数的主体部分。
- **generator**: 布尔值，表示是否为生成器函数。
- **async**: 布尔值，表示是否为异步函数。

#### Statements

各种语句类型。

##### ExpressionStatement

表达式语句。

```typescript
interface ExpressionStatement extends Node {
  type: "ExpressionStatement";
  expression: Expression;
}
```

- **expression**: `Expression` 对象，表示表达式。

##### BlockStatement

块语句。

```typescript
interface BlockStatement extends Node {
  type: "BlockStatement";
  body: Array<Statement>;
}
```

- **body**: 一个包含 `Statement` 的数组，表示块语句的主体部分。

##### EmptyStatement

空语句。

```typescript
interface EmptyStatement extends Node {
  type: "EmptyStatement";
}
```

##### DebuggerStatement

调试语句。

```typescript
interface DebuggerStatement extends Node {
  type: "DebuggerStatement";
}
```

##### WithStatement

with 语句。

```typescript
interface WithStatement extends Node {
  type: "WithStatement";
  object: Expression;
  body: Statement;
}
```

- **object**: `Expression` 对象，表示 with 语句的对象。
- **body**: `Statement` 对象，表示 with 语句的主体部分。

#### Control Flow Statements

控制流语句。

##### ReturnStatement

返回语句。

```typescript
interface ReturnStatement extends Node {
  type: "ReturnStatement";
  argument: Expression | null;
}
```

- **argument**: `Expression` 对象或 `null`，表示返回的值。

##### LabeledStatement

标签语句。

```typescript
interface LabeledStatement extends Node {
  type: "LabeledStatement";
  label: Identifier;
  body: Statement;
}
```

- **label**: `Identifier` 对象，表示标签的名称。
- **body**: `Statement` 对象，表示标签后的语句。

##### BreakStatement

中断语句。

```typescript
interface BreakStatement extends Node {
  type: "BreakStatement";
  label: Identifier | null;
}
```

- **label**: `Identifier` 对象或 `null`，表示中断的目标标签。

##### ContinueStatement

继续语句。

```typescript
interface ContinueStatement extends Node {
  type: "ContinueStatement";
  label: Identifier | null;
}
```

- **label**: `Identifier` 对象或 `null`，表示继续的目标标签。

##### IfStatement

条件选择语句。

```typescript
interface IfStatement extends Node {
  type: "IfStatement";
  test: Expression;
  consequent: Statement;
  alternate: Statement | null;
}
```

- **test**: `Expression` 对象，表示条件表达式。
- **consequent**: `Statement` 对象，表示条件为真时执行的语句。
- **alternate**: `Statement` 对象或 `null`，表示条件为假时执行的语句。

##### SwitchStatement

开关语句。

```typescript
interface SwitchStatement extends Node {
  type: "SwitchStatement";
  discriminant: Expression;
  cases: Array<SwitchCase>;
}
```

- **discriminant**: `Expression` 对象，表示开关表达式。
- **cases**: 一个包含 `SwitchCase` 的数组，表示开关的各个分支。

##### SwitchCase

开关分支。

```typescript
interface SwitchCase extends Node {
  type: "SwitchCase";
  test: Expression | null;
  consequent: Array<Statement>;
}
```

- **test**: `Expression` 对象或 `null`，表示分支的条件。
- **consequent**: 一个包含 `Statement` 的数组，表示分支的主体部分。

#### Exceptions

异常处理相关语句。

##### ThrowStatement

抛出异常语句。

```typescript
interface ThrowStatement extends Node {
  type: "ThrowStatement";
  argument: Expression;
}
```

- **argument**: `Expression` 对象，表示抛出的异常。

##### TryStatement

尝试捕获异常语句。

```typescript
interface TryStatement extends Node {
  type: "TryStatement";
  block: BlockStatement;
  handler: CatchClause | null;
  finalizer: BlockStatement | null;
}
```

- **block**: `BlockStatement` 对象，表示尝试块。
- **handler**: `CatchClause` 对象或 `null`，表示捕获块。
- **finalizer**: `BlockStatement` 对象或 `null`，表示最终块。

##### CatchClause

捕获子句。

```typescript
interface CatchClause extends Node {
  type: "CatchClause";
  param: Pattern | null;
  body: BlockStatement;
}
```

- **param**: `Pattern` 对象或 `null`，表示捕获的参数。
- **body**: `BlockStatement` 对象，表示捕获块的主体部分。

#### Loops

循环语句。

##### WhileStatement

while 语句。

```typescript
interface WhileStatement extends Node {
  type: "WhileStatement";
  test: Expression;
  body: Statement;
}
```

- **test**: `Expression` 对象，表示条件表达式。
- **body**: `Statement` 对象，表示循环体。

##### DoWhileStatement

do-while 语句。

```typescript
interface DoWhileStatement extends Node {
  type: "DoWhileStatement";
  body: Statement;
  test: Expression;
}
```

- **body**: `Statement` 对象，表示循环体。
- **test**: `Expression` 对象，表示条件表达式。

##### ForStatement

for 语句。

```typescript
interface ForStatement extends Node {
  type: "ForStatement";
  init: VariableDeclaration | Expression | null;
  test: Expression | null;
  update: Expression | null;
  body: Statement;
}
```

- **init**: `VariableDeclaration` 对象、`Expression` 对象或 `null`，表示初始化表达式。
- **test**: `Expression` 对象或 `null`，表示条件表达式。
- **update**: `Expression` 对象或 `null`，表示更新表达式。
- **body**: `Statement` 对象，表示循环体。

##### ForInStatement

for-in 语句。

```typescript
interface ForInStatement extends Node {
  type: "ForInStatement";
  left: VariableDeclaration | Expression;
  right: Expression;
  body: Statement;
}
```

- **left**: `VariableDeclaration` 对象或 `Expression` 对象，表示左侧表达式。
- **right**: `Expression` 对象，表示右侧表达式。
- **body**: `Statement` 对象，表示循环体。

##### ForOfStatement

for-of 语句。

```typescript
interface ForOfStatement extends Node {
  type: "ForOfStatement";
  left: VariableDeclaration | Expression;
  right: Expression;
  body: Statement;
  await: boolean;
}
```

- **left**: `VariableDeclaration` 对象或 `Expression` 对象，表示左侧表达式。
- **right**: `Expression` 对象，表示右侧表达式。
- **body**: `Statement` 对象，表示循环体。
- **await**: 布尔值，表示是否为异步迭代。

#### Declarations

声明语句。

##### FunctionDeclaration

函数声明。

```typescript
interface FunctionDeclaration extends Function, Declaration {
  type: "FunctionDeclaration";
  id: Identifier;
}
```

- **id**: `Identifier` 对象，表示函数的名称。

##### VariableDeclaration

变量声明。

```typescript
interface VariableDeclaration extends Node {
  type: "VariableDeclaration";
  declarations: Array<VariableDeclarator>;
  kind: "var" | "let" | "const";
}
```

- **declarations**: 一个包含 `VariableDeclarator` 的数组，表示变量声明。
- **kind**: 字符串，表示变量声明的类型，可以是 `"var"`, `"let"`, 或 `"const"`。

##### VariableDeclarator

变量声明子句。

```typescript
interface VariableDeclarator extends Node {
  type: "VariableDeclarator";
  id: Pattern;
  init: Expression | null;
}
```

- **id**: `Pattern` 对象，表示变量的名称。
- **init**: `Expression` 对象或 `null`，表示变量的初始值。

#### Expressions

表达式。

##### Super

super 关键字。

```typescript
interface Super extends Node {
  type: "Super";
}
```

##### Import

导入表达式。

```typescript
interface Import extends Node {
  type: "Import";
  arguments: Array<Expression>;
}
```

- **arguments**: 一个包含 `Expression` 的数组，表示导入的参数。

##### ThisExpression

this 表达式。

```typescript
interface ThisExpression extends Node {
  type: "ThisExpression";
}
```

##### ArrowFunctionExpression

箭头函数表达式。

```typescript
interface ArrowFunctionExpression extends Node {
  type: "ArrowFunctionExpression";
  params: Array<Pattern>;
  body: BlockStatement | Expression;
  async: boolean;
}
```

- **params**: 一个包含 `Pattern` 的数组，表示函数的参数。
- **body**: `BlockStatement` 对象或 `Expression` 对象，表示函数的主体部分。
- **async**: 布尔值，表示是否为异步函数。

##### YieldExpression

yield 表达式。

```typescript
interface YieldExpression extends Node {
  type: "YieldExpression";
  argument: Expression | null;
  delegate: boolean;
}
```

- **argument**: `Expression` 对象或 `null`，表示 yield 的值。
- **delegate**: 布尔值，表示是否为委托 yield。

##### AwaitExpression

await 表达式。

```typescript
interface AwaitExpression extends Node {
  type: "AwaitExpression";
  argument: Expression;
}
```

- **argument**: `Expression` 对象，表示 await 的值。

##### ArrayExpression

数组表达式。

```typescript
interface ArrayExpression extends Node {
  type: "ArrayExpression";
  elements: Array<Expression | SpreadElement | null>;
}
```

- **elements**: 一个包含 `Expression`、`SpreadElement` 或 `null` 的数组，表示数组的元素。

##### ObjectExpression

对象表达式。

```typescript
interface ObjectExpression extends Node {
  type: "ObjectExpression";
  properties: Array<ObjectProperty | ObjectMethod | SpreadElement>;
}
```

- **properties**: 一个包含 `ObjectProperty`、`ObjectMethod` 或 `SpreadElement` 的数组，表示对象的属性。

##### ObjectProperty

对象属性。

```typescript
interface ObjectProperty extends Node {
  type: "ObjectProperty";
  key: Expression;
  value: Expression;
  computed: boolean;
  shorthand: boolean;
}
```

- **key**: `Expression` 对象，表示属性的键。
- **value**: `Expression` 对象，表示属性的值。
- **computed**: 布尔值，表示属性键是否为计算属性。
- **shorthand**: 布尔值，表示是否为简写属性。

##### ObjectMethod

对象方法。

```typescript
interface ObjectMethod extends Node {
  type: "ObjectMethod";
  key: Expression;
  value: Function;
  computed: boolean;
  kind: "method" | "get" | "set";
}
```

- **key**: `Expression` 对象，表示方法的键。
- **value**: `Function` 对象，表示方法的实现。
- **computed**: 布尔值，表示方法键是否为计算属性。
- **kind**: 字符串，表示方法的类型，可以是 `"method"`, `"get"`, 或 `"set"`。

##### RecordExpression

记录表达式。

```typescript
interface RecordExpression extends Node {
  type: "RecordExpression";
  properties: Array<ObjectProperty>;
}
```

- **properties**: 一个包含 `ObjectProperty` 的数组，表示记录的属性。

##### TupleExpression

元组表达式。

```typescript
interface TupleExpression extends Node {
  type: "TupleExpression";
  elements: Array<Expression | SpreadElement | null>;
}
```

- **elements**: 一个包含 `Expression`、`SpreadElement` 或 `null` 的数组，表示元组的元素。

##### FunctionExpression

函数表达式。

```typescript
interface FunctionExpression extends Function {
  type: "FunctionExpression";
  id: Identifier | null;
}
```

- **id**: `Identifier` 对象或 `null`，表示函数的名称。

##### UnaryExpression

一元运算符表达式。

```typescript
interface UnaryExpression extends Node {
  type: "UnaryExpression";
  operator: UnaryOperator;
  prefix: boolean;
  argument: Expression;
}
```

- **operator**: 字符串，表示一元运算符。
- **prefix**: 布尔值，表示运算符是否为前缀形式。
- **argument**: `Expression` 对象，表示运算符的操作数。

##### UpdateExpression

更新表达式。

```typescript
interface UpdateExpression extends Node {
  type: "UpdateExpression";
  operator: UpdateOperator;
  prefix: boolean;
  argument: Expression;
}
```

- **operator**: 字符串，表示更新运算符。
- **prefix**: 布尔值，表示运算符是否为前缀形式。
- **argument**: `Expression` 对象，表示运算符的操作数。

##### BinaryExpression

二元运算符表达式。

```typescript
interface BinaryExpression extends Node {
  type: "BinaryExpression";
  operator: BinaryOperator;
  left: Expression;
  right: Expression;
}
```

- **operator**: 字符串，表示二元运算符。
- **left**: `Expression` 对象，表示左操作数。
- **right**: `Expression` 对象，表示右操作数。

##### AssignmentExpression

赋值表达式。

```typescript
interface AssignmentExpression extends Node {
  type: "AssignmentExpression";
  operator: AssignmentOperator;
  left: Pattern | MemberExpression;
  right: Expression;
}
```

- **operator**: 字符串，表示赋值运算符。
- **left**: `Pattern` 或 `MemberExpression` 对象，表示左操作数。
- **right**: `Expression` 对象，表示右操作数。

##### LogicalExpression

逻辑表达式。

```typescript
interface LogicalExpression extends Node {
  type: "LogicalExpression";
  operator: LogicalOperator;
  left: Expression;
  right: Expression;
}
```

- **operator**: 字符串，表示逻辑运算符。
- **left**: `Expression` 对象，表示左操作数。
- **right**: `Expression` 对象，表示右操作数。

##### SpreadElement

展开元素。

```typescript
interface SpreadElement extends Node {
  type: "SpreadElement";
  argument: Expression;
}
```

- **argument**: `Expression` 对象，表示要展开的值。

##### ArgumentPlaceholder

参数占位符。

```typescript
interface ArgumentPlaceholder extends Node {
  type: "ArgumentPlaceholder";
}
```

##### MemberExpression

成员表达式。

```typescript
interface MemberExpression extends Node {
  type: "MemberExpression";
  object: Expression;
  property: Expression;
  computed: boolean;
  optional: boolean;
}
```

- **object**: `Expression` 对象，表示成员表达式的对象。
- **property**: `Expression` 对象，表示成员表达式的属性。
- **computed**: 布尔值，表示属性是否为计算属性。
- **optional**: 布尔值，表示是否为可选链操作。

##### OptionalMemberExpression

可选成员表达式。

```typescript
interface OptionalMemberExpression extends MemberExpression {
  type: "OptionalMemberExpression";
  optional: true;
}
```

- **optional**: 布尔值，表示是否为可选链操作。

##### BindExpression

绑定表达式。

```typescript
interface BindExpression extends Node {
  type: "BindExpression";
  object: Expression;
  callee: Expression;
}
```

- **object**: `Expression` 对象，表示绑定表达式的对象。
- **callee**: `Expression` 对象，表示绑定表达式的调用者。

##### ConditionalExpression

条件表达式。

```typescript
interface ConditionalExpression extends Node {
  type: "ConditionalExpression";
  test: Expression;
  consequent: Expression;
  alternate: Expression;
}
```

- **test**: `Expression` 对象，表示条件表达式。
- **consequent**: `Expression` 对象，表示条件为真时的值。
- **alternate**: `Expression` 对象，表示条件为假时的值。

##### CallExpression

调用表达式。

```typescript
interface CallExpression extends Node {
  type: "CallExpression";
  callee: Expression;
  arguments: Array<Expression | SpreadElement>;
  optional: boolean;
}
```

- **callee**: `Expression` 对象，表示调用的目标。
- **arguments**: 一个包含 `Expression` 或 `SpreadElement` 的数组，表示调用的参数。
- **optional**: 布尔值，表示是否为可选链调用。

##### OptionalCallExpression

可选调用表达式。

```typescript
interface OptionalCallExpression extends CallExpression {
  type: "OptionalCallExpression";
  optional: true;
}
```

- **optional**: 布尔值，表示是否为可选链调用。

##### NewExpression

新建对象表达式。

```typescript
interface NewExpression extends Node {
  type: "NewExpression";
  callee: Expression;
  arguments: Array<Expression | SpreadElement> | null;
}
```

- **callee**: `Expression` 对象，表示构造函数。
- **arguments**: 一个包含 `Expression` 或 `SpreadElement` 的数组，表示构造函数的参数，或者为 `null`。

##### SequenceExpression

序列表达式。

```typescript
interface SequenceExpression extends Node {
  type: "SequenceExpression";
  expressions: Array<Expression>;
}
```

- **expressions**: 一个包含 `Expression` 的数组，表示序列中的表达式。

##### ParenthesizedExpression

圆括号内的表达式。

```typescript
interface ParenthesizedExpression extends Node {
  type: "ParenthesizedExpression";
  expression: Expression;
}
```

- **expression**: `Expression` 对象，表示括号内的表达式。

##### DoExpression

do 表达式。

```typescript
interface DoExpression extends Node {
  type: "DoExpression";
  body: Expression;
}
```

- **body**: `Expression` 对象，表示 do 表达式的主体部分。

##### ModuleExpression

模块表达式。

```typescript
interface ModuleExpression extends Node {
  type: "ModuleExpression";
  body: Program;
}
```

- **body**: `Program` 对象，表示模块的主体部分。

#### Template Literals

模板字符串。

##### TemplateLiteral

模板字符串。

```typescript
interface TemplateLiteral extends Node {
  type: "TemplateLiteral";
  quasis: Array<TemplateElement>;
  expressions: Array<Expression>;
}
```

- **quasis**: 一个包含 `TemplateElement` 的数组，表示模板字符串的静态部分。
- **expressions**: 一个包含 `Expression` 的数组，表示模板字符串的动态部分。

##### TaggedTemplateExpression

带标签的模板字符串。

```typescript
interface TaggedTemplateExpression extends Node {
  type: "TaggedTemplateExpression";
  tag: Expression;
  quasi: TemplateLiteral;
}
```

- **tag**: `Expression` 对象，表示标签函数。
- **quasi**: `TemplateLiteral` 对象，表示模板字符串。

##### TemplateElement

模板字符串元素。

```typescript
interface TemplateElement extends Node {
  type: "TemplateElement";
  value: { raw: string; cooked: string };
  tail: boolean;
}
```

- **value**: 对象，包含 `raw` 和 `cooked` 字段，分别表示原始字符串和解析后的字符串。
- **tail**: 布尔值，表示是否为最后一个元素。

#### Patterns

模式匹配。

##### ObjectPattern

对象模式。

```typescript
interface ObjectPattern extends Node {
  type: "ObjectPattern";
  properties: Array<ObjectProperty | RestElement>;
}
```

- **properties**: 一个包含 `ObjectProperty` 或 `RestElement` 的数组，表示对象模式的属性。

##### ArrayPattern

数组模式。

```typescript
interface ArrayPattern extends Node {
  type: "ArrayPattern";
  elements: Array<Pattern | RestElement | null>;
}
```

- **elements**: 一个包含 `Pattern`、`RestElement` 或 `null` 的数组，表示数组模式的元素。

##### RestElement

剩余元素。

```typescript
interface RestElement extends Node {
  type: "RestElement";
  argument: Pattern;
}
```

- **argument**: `Pattern` 对象，表示剩余元素的模式。

##### AssignmentPattern

赋值模式。

```typescript
interface AssignmentPattern extends Node {
  type: "AssignmentPattern";
  left: Pattern;
  right: Expression;
}
```

- **left**: `Pattern` 对象，表示赋值模式的左侧。
- **right**: `Expression` 对象，表示赋值模式的右侧。

#### Classes

类。

##### ClassBody

类体。

```typescript
interface ClassBody extends Node {
  type: "ClassBody";
  body: Array<ClassMethod | ClassPrivateMethod | ClassProperty | ClassPrivateProperty | StaticBlock>;
}
```

- **body**: 一个包含 `ClassMethod`、`ClassPrivateMethod`、`ClassProperty`、`ClassPrivateProperty` 或 `StaticBlock` 的数组，表示类体的内容。

##### ClassMethod

类方法。

```typescript
interface ClassMethod extends Node {
  type: "ClassMethod";
  key: Expression;
  value: Function;
  kind: "method" | "get" | "set";
  computed: boolean;
  static: boolean;
}
```

- **key**: `Expression` 对象，表示方法的键。
- **value**: `Function` 对象，表示方法的实现。
- **kind**: 字符串，表示方法的类型，可以是 `"method"`, `"get"`, 或 `"set"`。
- **computed**: 布尔值，表示方法键是否为计算属性。
- **static**: 布尔值，表示方法是否为静态方法。

##### ClassPrivateMethod

类私有方法。

```typescript
interface ClassPrivateMethod extends Node {
  type: "ClassPrivateMethod";
  key: PrivateName;
  value: Function;
  kind: "method" | "get" | "set";
  computed: false;
  static: boolean;
}
```

- **key**: `PrivateName` 对象，表示私有方法的键。
- **value**: `Function` 对象，表示私有方法的实现。
- **kind**: 字符串，表示私有方法的类型，可以是 `"method"`, `"get"`, 或 `"set"`。
- **computed**: 布尔值，表示私有方法键是否为计算属性（始终为 `false`）。
- **static**: 布尔值，表示私有方法是否为静态方法。

##### ClassProperty

类属性。

```typescript
interface ClassProperty extends Node {
  type: "ClassProperty";
  key: Expression;
  value: Expression | null;
  computed: boolean;
  static: boolean;
}
```

- **key**: `Expression` 对象，表示属性的键。
- **value**: `Expression` 对象或 `null`，表示属性的初始值。
- **computed**: 布尔值，表示属性键是否为计算属性。
- **static**: 布尔值，表示属性是否为静态属性。

##### ClassPrivateProperty

类私有属性。

```typescript
interface ClassPrivateProperty extends Node {
  type: "ClassPrivateProperty";
  key: PrivateName;
  value: Expression | null;
  computed: false;
  static: boolean;
}
```

- **key**: `PrivateName` 对象，表示私有属性的键。
- **value**: `Expression` 对象或 `null`，表示私有属性的初始值。
- **computed**: 布尔值，表示私有属性键是否为计算属性（始终为 `false`）。
- **static**: 布尔值，表示私有属性是否为静态属性。

##### StaticBlock

静态初始化块。

```typescript
interface StaticBlock extends Node {
  type: "StaticBlock";
  body: Array<Statement>;
}
```

- **body**: 一个包含 `Statement` 的数组，表示静态初始化块的主体部分。

##### ClassDeclaration

类声明。

```typescript
interface ClassDeclaration extends Class, Declaration {
  type: "ClassDeclaration";
  id: Identifier;
}
```

- **id**: `Identifier` 对象，表示类的名称。

##### ClassExpression

类表达式。

```typescript
interface ClassExpression extends Class, Expression {
  type: "ClassExpression";
  id: Identifier | null;
}
```

- **id**: `Identifier` 对象或 `null`，表示类的名称。

##### MetaProperty

元属性。

```typescript
interface MetaProperty extends Node {
  type: "MetaProperty";
  meta: Identifier;
  property: Identifier;
}
```

- **meta**: `Identifier` 对象，表示元属性的前缀。
- **property**: `Identifier` 对象，表示元属性的后缀。

#### Modules

模块。

##### ModuleSpecifier

模块指定符。

```typescript
interface ModuleSpecifier extends Node {
  type: "ModuleSpecifier";
  local: Identifier;
}
```

- **local**: `Identifier` 对象，表示模块指定符的本地名称。

##### ImportDeclaration

导入声明。

```typescript
interface ImportDeclaration extends Node {
  type: "ImportDeclaration";
  importKind: "type" | "typeof" | "value" | null;
  specifiers: Array<ImportSpecifier | ImportDefaultSpecifier | ImportNamespaceSpecifier>;
  source: StringLiteral;
  assertions?: Array<ImportAttribute>;
}
```

- **importKind**: 字符串，表示导入的类型，可以是 `"type"`, `"typeof"`, 或 `"value"`，或者为 `null`。
- **specifiers**: 一个包含 `ImportSpecifier`、`ImportDefaultSpecifier` 或 `ImportNamespaceSpecifier` 的数组，表示导入的指定符。
- **source**: `StringLiteral` 对象，表示导入的模块路径。
- **assertions**: 一个包含 `ImportAttribute` 的数组，表示导入的属性（可选）。

##### ImportSpecifier

导入指定符。

```typescript
interface ImportSpecifier extends ModuleSpecifier {
  type: "ImportSpecifier";
  imported: Identifier | StringLiteral;
}
```

- **imported**: `Identifier` 对象或 `StringLiteral` 对象，表示导入的模块中的名称。
- **local**: `Identifier` 对象，表示导入到本地作用域的名称。

##### ImportDefaultSpecifier

默认导入指定符。

```typescript
interface ImportDefaultSpecifier extends ModuleSpecifier {
  type: "ImportDefaultSpecifier";
  local: Identifier;
}
```

- **local**: `Identifier` 对象，表示导入到本地作用域的名称。

##### ImportNamespaceSpecifier

命名空间导入指定符。

```typescript
interface ImportNamespaceSpecifier extends ModuleSpecifier {
  type: "ImportNamespaceSpecifier";
  local: Identifier;
}
```

- **local**: `Identifier` 对象，表示导入到本地作用域的名称。

##### ImportAttribute

导入属性。

```typescript
interface ImportAttribute extends Node {
  type: "ImportAttribute";
  key: Identifier;
  value: StringLiteral;
}
```

- **key**: `Identifier` 对象，表示属性的键。
- **value**: `StringLiteral` 对象，表示属性的值。

##### ExportDeclaration

导出声明。

```typescript
interface ExportDeclaration extends Node {
  type: "ExportDeclaration";
}
```

##### ExportNamedDeclaration

命名导出声明。

```typescript
interface ExportNamedDeclaration extends ExportDeclaration {
  type: "ExportNamedDeclaration";
  declaration: Declaration | null;
  specifiers: Array<ExportSpecifier | ExportNamespaceSpecifier>;
  source: StringLiteral | null;
  assertions?: Array<ImportAttribute>;
}
```

- **declaration**: `Declaration` 对象或 `null`，表示导出的声明。
- **specifiers**: 一个包含 `ExportSpecifier` 或 `ExportNamespaceSpecifier` 的数组，表示导出的指定符。
- **source**: `StringLiteral` 对象或 `null`，表示导出的模块路径。
- **assertions**: 一个包含 `ImportAttribute` 的数组，表示导出的属性（可选）。

##### ExportSpecifier

导出指定符。

```typescript
interface ExportSpecifier extends ModuleSpecifier {
  type: "ExportSpecifier";
  exported: Identifier | StringLiteral;
  local: Identifier | StringLiteral;
}
```

- **exported**: `Identifier` 对象或 `StringLiteral` 对象，表示导出的模块中的名称。
- **local**: `Identifier` 对象或 `StringLiteral` 对象，表示导出到模块外部的名称。

##### ExportNamespaceSpecifier

命名空间导出指定符。

```typescript
interface ExportNamespaceSpecifier extends ModuleSpecifier {
  type: "ExportNamespaceSpecifier";
  exported: Identifier;
}
```

- **exported**: `Identifier` 对象，表示导出到模块外部的名称。

##### ExportDefaultDeclaration

默认导出声明。

```typescript
interface ExportDefaultDeclaration extends ExportDeclaration {
  type: "ExportDefaultDeclaration";
  declaration: FunctionDeclaration | ClassDeclaration | Expression;
}
```

- **declaration**: `FunctionDeclaration` 对象、`ClassDeclaration` 对象或 `Expression` 对象，表示默认导出的声明。

##### ExportAllDeclaration

全部导出声明。

```typescript
interface ExportAllDeclaration extends ExportDeclaration {
  type: "ExportAllDeclaration";
  source: StringLiteral;
  assertions?: Array<ImportAttribute>;
}
```

- **source**: `StringLiteral` 对象，表示导出的模块路径。
- **assertions**: 一个包含 `ImportAttribute` 的数组，表示导出的属性（可选）。
