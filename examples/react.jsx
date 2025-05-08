// React JSX示例文件，用于测试Lumen编译器的JSX支持
import React, { useState, useEffect, useCallback, useMemo } from 'react';

// 函数组件
function Counter({ initialCount = 0, step = 1 }) {
  // 使用useState钩子
  const [count, setCount] = useState(initialCount);
  
  // 使用useEffect钩子
  useEffect(() => {
    document.title = `计数: ${count}`;
    
    // 清理函数
    return () => {
      document.title = '应用';
    };
  }, [count]);
  
  // 使用useCallback钩子
  const increment = useCallback(() => {
    setCount(prevCount => prevCount + step);
  }, [step]);
  
  const decrement = useCallback(() => {
    setCount(prevCount => prevCount - step);
  }, [step]);
  
  // 使用useMemo钩子
  const isEven = useMemo(() => {
    console.log('计算奇偶性...');
    return count % 2 === 0;
  }, [count]);
  
  return (
    <div className="counter">
      <h2>计数器</h2>
      <p>当前计数: <span className={isEven ? 'even' : 'odd'}>{count}</span></p>
      <p>步长: {step}</p>
      <p>计数是: {isEven ? '偶数' : '奇数'}</p>
      
      <div className="buttons">
        <button onClick={decrement}>减少</button>
        <button onClick={increment}>增加</button>
      </div>
    </div>
  );
}

// 类组件
class TodoApp extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      todos: [],
      newTodo: '',
    };
  }
  
  componentDidMount() {
    // 从localStorage加载保存的数据
    const savedTodos = localStorage.getItem('todos');
    if (savedTodos) {
      this.setState({ todos: JSON.parse(savedTodos) });
    }
  }
  
  componentDidUpdate(prevProps, prevState) {
    // 保存到localStorage
    if (prevState.todos !== this.state.todos) {
      localStorage.setItem('todos', JSON.stringify(this.state.todos));
    }
  }
  
  handleInputChange = (e) => {
    this.setState({ newTodo: e.target.value });
  };
  
  handleAddTodo = () => {
    if (this.state.newTodo.trim() === '') return;
    
    this.setState(prevState => ({
      todos: [
        ...prevState.todos,
        {
          id: Date.now(),
          text: prevState.newTodo,
          completed: false,
        }
      ],
      newTodo: '',
    }));
  };
  
  handleToggleTodo = (id) => {
    this.setState(prevState => ({
      todos: prevState.todos.map(todo => 
        todo.id === id ? { ...todo, completed: !todo.completed } : todo
      ),
    }));
  };
  
  handleDeleteTodo = (id) => {
    this.setState(prevState => ({
      todos: prevState.todos.filter(todo => todo.id !== id),
    }));
  };
  
  render() {
    const { todos, newTodo } = this.state;
    
    return (
      <div className="todo-app">
        <h2>待办事项</h2>
        
        <div className="add-todo">
          <input
            type="text"
            value={newTodo}
            onChange={this.handleInputChange}
            placeholder="新增待办事项..."
          />
          <button onClick={this.handleAddTodo}>添加</button>
        </div>
        
        <ul className="todo-list">
          {todos.length === 0 ? (
            <li className="empty-message">暂无待办事项</li>
          ) : (
            todos.map(todo => (
              <li key={todo.id} className={todo.completed ? 'completed' : ''}>
                <input
                  type="checkbox"
                  checked={todo.completed}
                  onChange={() => this.handleToggleTodo(todo.id)}
                />
                <span>{todo.text}</span>
                <button onClick={() => this.handleDeleteTodo(todo.id)}>删除</button>
              </li>
            ))
          )}
        </ul>
        
        <div className="todo-stats">
          <p>总计: {todos.length} 项</p>
          <p>已完成: {todos.filter(todo => todo.completed).length} 项</p>
        </div>
      </div>
    );
  }
}

// 组合多个组件
function App() {
  return (
    <div className="app">
      <header>
        <h1>Lumen JSX 示例</h1>
      </header>
      
      <main>
        <Counter initialCount={10} step={2} />
        <hr />
        <TodoApp />
      </main>
      
      <footer>
        <p>由 Lumen 编译器编译 &copy; {new Date().getFullYear()}</p>
      </footer>
    </div>
  );
}

// 导出
export { Counter, TodoApp };
export default App; 