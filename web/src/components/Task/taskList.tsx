import { useEffect, useState } from "react";
import { Task } from "../../interfaces/task";
import { TaskAPI } from "../../api";

const TaskList = () => {
  const [tasks, setTasks] = useState<Array<Task>>([]);

  useEffect(() => {
    const fetchTask = async () => {
      try {
        const getTasks = await TaskAPI.getTasks();
        setTasks([...getTasks]);
      } catch {
        console.log("Error fetching tasks: ", console.error);
      }
    };

    fetchTask();
  }, []);

  const updateTask = (task: Task) => {
    task.completed = !task.completed;
    TaskAPI.updateTask(task);
    window.location.reload();
  };

  const deleteTask = (event: React.MouseEvent, task: Task) => {
    event.stopPropagation();
    TaskAPI.deleteTask(task);
    window.location.reload();
  };

  return (
    <>
      <div>
        {tasks.length > 0 && (
          <ul>
            {tasks.map((task) => (
              <li key={task.id}>
                <span>
                  {task.completed
                    ? (
                      <span
                        className="text-1xl"
                        style={{ cursor: "pointer" }}
                        onClick={() => updateTask(task)}
                      >
                        <s>{task.text}</s>
                        <span
                          className="text-1xl"
                          style={{ cursor: "pointer" }}
                          onClick={(event) => deleteTask(event, task)}
                        >
                          : DELETE
                        </span>
                      </span>
                    )
                    : (
                      <p
                        className="text-1xl"
                        style={{ cursor: "pointer" }}
                        onClick={() => updateTask(task)}
                      >
                        {task.text}
                      </p>
                    )}
                </span>
              </li>
            ))}
          </ul>
        )}
      </div>
    </>
  );
};

export default TaskList;
