// 示例JavaScript文件，用于测试Lumen编译器

// 变量声明示例
var a = 1;
let b = 2;
const c = 3;

// 函数声明示例
function add(x, y) {
  return x + y;
}

// 箭头函数示例
const multiply = (x, y) => x * y;

// 类示例
class Calculator {
  constructor() {
    this.value = 0;
  }
  
  add(x) {
    this.value += x;
    return this;
  }
  
  subtract(x) {
    this.value -= x;
    return this;
  }
  
  getValue() {
    return this.value;
  }
}

// 模块导出
export { add, multiply, Calculator };
export default Calculator;

// 循环和条件示例
for (let i = 0; i < 10; i++) {
  if (i % 2 === 0) {
    console.log("偶数: " + i);
  } else {
    console.log("奇数: " + i);
  }
}

// 数组和对象操作
const numbers = [1, 2, 3, 4, 5];
const doubled = numbers.map(n => n * 2);
const sum = numbers.reduce((acc, n) => acc + n, 0);

const person = {
  name: "张三",
  age: 30,
  greet() {
    return `你好，我是${this.name}，今年${this.age}岁`;
  }
};

// 异步代码示例
async function fetchData() {
  try {
    const response = await fetch('https://example.com/api/data');
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("获取数据失败:", error);
    return null;
  }
}

// 未使用的死代码（用于测试死代码消除）
function unusedFunction() {
  console.log("这个函数永远不会被调用");
}

// 常量表达式（用于测试常量折叠优化）
const MAGIC_NUMBER = 10 * 10 + 5 * 4;
const PI_TIMES_2 = 3.14159 * 2; 