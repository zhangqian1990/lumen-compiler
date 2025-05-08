// TypeScript示例文件，用于测试Lumen编译器的TypeScript支持

// 类型定义
type ID = string | number;
interface Person {
  id: ID;
  name: string;
  age: number;
  email?: string;
  address: {
    city: string;
    country: string;
  };
}

// 泛型
interface Repository<T> {
  getById(id: ID): T | null;
  getAll(): T[];
  save(item: T): void;
  delete(id: ID): boolean;
}

// 类实现接口
class PersonRepository implements Repository<Person> {
  private items: Person[] = [];
  
  getById(id: ID): Person | null {
    return this.items.find(item => item.id === id) || null;
  }
  
  getAll(): Person[] {
    return [...this.items];
  }
  
  save(person: Person): void {
    const index = this.items.findIndex(item => item.id === person.id);
    if (index >= 0) {
      this.items[index] = { ...person };
    } else {
      this.items.push({ ...person });
    }
  }
  
  delete(id: ID): boolean {
    const initialLength = this.items.length;
    this.items = this.items.filter(item => item.id !== id);
    return initialLength !== this.items.length;
  }
}

// 枚举
enum UserRole {
  Admin = "ADMIN",
  Editor = "EDITOR",
  Viewer = "VIEWER",
}

// 使用高级类型
type Nullable<T> = T | null;
type UserWithRole = Person & { role: UserRole };

// 函数重载
function process(input: number): number;
function process(input: string): string;
function process(input: number | string): number | string {
  if (typeof input === 'number') {
    return input * 2;
  } else {
    return input.toUpperCase();
  }
}

// 泛型函数
function firstOrNull<T>(array: T[]): Nullable<T> {
  return array.length > 0 ? array[0] : null;
}

// 使用类型断言
const someValue: any = "this is a string";
const strLength: number = (someValue as string).length;

// 普通类，不使用装饰器
class Calculator {
  add(a: number, b: number): number {
    console.log(`Calling add with:`, [a, b]);
    return a + b;
  }
}

// 导出
export { Person, Repository, PersonRepository, UserRole, process, firstOrNull }; 