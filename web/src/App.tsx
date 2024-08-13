import './App.css'
import Header from './components/Header/header'
import TaskForm from './components/Task/taskForm'
import TaskList from './components/Task/taskList'

function App() {
  return (
    <>
      <center>
        <Header />
        <hr />
        <TaskList />
        <TaskForm />
      </center>
    </>
  )
}

export default App
