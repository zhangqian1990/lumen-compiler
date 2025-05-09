// Lumen 编译速度测试文件
// 一个包含各种常见JavaScript模式的测试文件

// 1. 变量声明与赋值
const VERSION = '1.0.0';
let counter = 0;
var legacy = true;

// 2. 函数声明
function add(a, b) {
  return a + b;
}

// 3. 箭头函数
const multiply = (a, b) => a * b;
const divide = (a, b) => {
  if (b === 0) throw new Error('除数不能为零');
  return a / b;
};

// 4. 类
class Calculator {
  constructor(initialValue = 0) {
    this.value = initialValue;
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
  
  static create(initialValue) {
    return new Calculator(initialValue);
  }
}

// 5. 对象字面量
const config = {
  apiUrl: 'https://api.example.com',
  timeout: 3000,
  retries: 3,
  headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer token123'
  },
  debug: process.env.NODE_ENV !== 'production'
};

// 6. 数组和数组方法
const numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const doubled = numbers.map(n => n * 2);
const evens = numbers.filter(n => n % 2 === 0);
const sum = numbers.reduce((acc, curr) => acc + curr, 0);

// 7. 异步代码
async function fetchData(url) {
  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status}`);
    }
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('获取数据失败:', error);
    return null;
  }
}

// 8. Promise 链
function processInSequence(items) {
  return items.reduce((promise, item) => {
    return promise
      .then(results => {
        return processItem(item).then(result => {
          results.push(result);
          return results;
        });
      });
  }, Promise.resolve([]));
}

function processItem(item) {
  return new Promise(resolve => {
    setTimeout(() => resolve(`处理项目: ${item}`), 100);
  });
}

// 9. 解构赋值
const { apiUrl, timeout, headers: { Authorization } } = config;
const [first, second, ...rest] = numbers;

// 10. 默认参数和剩余参数
function createUser(name, age = 25, ...hobbies) {
  return {
    name,
    age,
    hobbies
  };
}

// 11. 模板字符串
const greeting = `欢迎，${name}！今天是${new Date().toLocaleDateString()}`;

// 12. 条件和循环结构
for (let i = 0; i < 10; i++) {
  if (i % 2 === 0) {
    console.log(`${i} 是偶数`);
  } else {
    console.log(`${i} 是奇数`);
  }
}

let i = 0;
while (i < 5) {
  console.log(`循环计数: ${i}`);
  i++;
}

// 13. Switch语句
function getDayName(dayNum) {
  switch (dayNum) {
    case 0: return '星期日';
    case 1: return '星期一';
    case 2: return '星期二';
    case 3: return '星期三';
    case 4: return '星期四';
    case 5: return '星期五';
    case 6: return '星期六';
    default: return '无效日期';
  }
}

// 14. 迭代器和生成器
function* fibonacciGenerator() {
  let a = 1, b = 1;
  while (true) {
    yield a;
    [a, b] = [b, a + b];
  }
}

// 15. Map 和 Set
const userMap = new Map();
userMap.set('user1', { name: '张三', age: 30 });
userMap.set('user2', { name: '李四', age: 25 });

const uniqueNumbers = new Set([1, 2, 3, 3, 4, 5, 5]);

// 16. 错误处理
try {
  const result = divide(10, 0);
  console.log(result);
} catch (e) {
  console.error('计算错误:', e.message);
} finally {
  console.log('计算操作完成');
}

// 17. 模块导出
export { add, multiply, divide, Calculator };
export default {
  VERSION,
  config,
  createUser
}; 